use std::fmt;

use serde::{Deserialize, Serialize};
use uuid::Uuid;
use windows::Win32::Foundation::HWND;

use crate::{
    core::{capture, matcher::match_template, Project, Setting},
    fl,
};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct UserVariable {
    pub id: Uuid,
    pub name: String,
    pub ty: VarType,
    pub value: String,
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Condition {
    Cmp { left: Operand, op: CmpOp, right: Operand },
    ImageMatch { target: String },
    All(Vec<Condition>),
    Any(Vec<Condition>),
    Not(Box<Condition>),
}

#[derive(Debug, Clone, PartialEq, Serialize, Deserialize)]
pub enum Operand {
    Var(String),
    LitBool(bool),
    LitInt(i64),
    LitFloat(f64),
    LitStr(String),
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum CmpOp {
    Eq,       // ==
    Ne,       // !=
    Lt,       // <
    Le,       // <=
    Gt,       // >
    Ge,       // >=
    Contains,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum VarOp {
    Assign, // =
    Add,    // +=
    Sub,    // -=
    Mul,    // *=
    Div,    // /=
    Toggle,
}

#[derive(Debug, Copy, Clone, Serialize, Deserialize, PartialEq, Eq, Hash)]
pub enum VarType {
    Bool,
    Int,
    Float,
    Str,
}

#[derive(Debug, Clone)]
pub enum VarValue {
    Bool(bool),
    Int(i64),
    Float(f64),
    Str(String),
}

impl fmt::Display for Operand {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Operand::Var(name) => write!(f, "${}", name),
            Operand::LitBool(v) => write!(f, "{}", v),
            Operand::LitInt(v) => write!(f, "{}", v),
            Operand::LitFloat(v) => write!(f, "{}", v),
            Operand::LitStr(v) => write!(f, "\"{}\"", v),
        }
    }
}

impl fmt::Display for CmpOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let cmp_op = match self {
            CmpOp::Eq => "==",
            CmpOp::Ne => "!=",
            CmpOp::Lt => "<",
            CmpOp::Le => "<=",
            CmpOp::Gt => ">",
            CmpOp::Ge => ">=",
            CmpOp::Contains => "Contains",
        };
        write!(f, "{cmp_op}")
    }
}

impl fmt::Display for VarOp {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let s = match self {
            VarOp::Assign => "=",
            VarOp::Add => "+=",
            VarOp::Sub => "-=",
            VarOp::Mul => "*=",
            VarOp::Div => "/=",
            VarOp::Toggle => "toggle",
        };
        write!(f, "{s}")
    }
}

impl UserVariable {
    pub fn parse_self(&self) -> Result<VarValue, String> {
        match self.ty {
            VarType::Bool => self.value.parse::<bool>().map(VarValue::Bool).map_err(|_| "err".to_string()),
            VarType::Int => self.value.parse::<i64>().map(VarValue::Int).map_err(|_| "err".to_string()),
            VarType::Float => self.value.parse::<f64>().map(VarValue::Float).map_err(|_| "err".to_string()),
            VarType::Str => Ok(VarValue::Str(self.value.to_owned())),
        }
    }

    pub fn parse_value(ty: &VarType, value: &str) -> Result<VarValue, String> {
        match ty {
            VarType::Bool => value.parse::<bool>().map(VarValue::Bool).map_err(|_| "err".to_string()),
            VarType::Int => value.parse::<i64>().map(VarValue::Int).map_err(|_| "err".to_string()),
            VarType::Float => value.parse::<f64>().map(VarValue::Float).map_err(|_| "err".to_string()),
            VarType::Str => Ok(VarValue::Str(value.to_owned())),
        }
    }
}

pub fn evaluate_operand(project: &Project, op: &Operand) -> Option<VarValue> {
    use Operand::*;
    Some(match op {
        Var(name) => project.get_variable_value(name)?,
        LitBool(b) => VarValue::Bool(*b),
        LitInt(i) => VarValue::Int(*i),
        LitFloat(f) => VarValue::Float(*f),
        LitStr(s) => VarValue::Str(s.clone()),
    })
}

fn as_f64(v: &VarValue) -> Option<f64> {
    match v {
        VarValue::Int(i) => Some(*i as f64),
        VarValue::Float(f) => Some(*f),
        _ => None,
    }
}

fn cmp_values(op: &CmpOp, a: &VarValue, b: &VarValue) -> Option<bool> {
    use crate::VarValue::*;
    use CmpOp::*;

    match (a, b, op) {
        (Bool(x), Bool(y), Eq) => Some(x == y),
        (Bool(x), Bool(y), Ne) => Some(x != y),

        (lhs, rhs, op) if as_f64(lhs).is_some() && as_f64(rhs).is_some() => {
            let x = as_f64(lhs).unwrap();
            let y = as_f64(rhs).unwrap();
            Some(match op {
                Eq => x == y,
                Ne => x != y,
                Lt => x < y,
                Le => x <= y,
                Gt => x > y,
                Ge => x >= y,
                Contains => return None,
            })
        }

        (Str(x), Str(y), Eq) => Some(x == y),
        (Str(x), Str(y), Ne) => Some(x != y),
        (Str(x), Str(y), Contains) => Some(x.contains(y)),

        _ => None,
    }
}

pub fn evaluate_condition(condition: &Condition, project: &Project, hwnd_ptr: usize, setting: &Setting) -> bool {
    use Condition::*;
    match condition {
        Not(c) => !evaluate_condition(c, project, hwnd_ptr, setting),
        All(v) => v.iter().all(|c| evaluate_condition(c, project, hwnd_ptr, setting)),
        Any(v) => v.iter().any(|c| evaluate_condition(c, project, hwnd_ptr, setting)),
        Cmp { left, op, right } => {
            let Some(a) = evaluate_operand(project, left) else {
                return false;
            };
            let Some(b) = evaluate_operand(project, right) else {
                return false;
            };
            cmp_values(op, &a, &b).unwrap_or(false)
        }
        ImageMatch { target } => {
            let Some(item) = project.items.iter().find(|i| i.name == *target) else {
                eprintln!("ImageMatch: item not found: {target}");
                return false;
            };

            let Some(image_path) = &item.image_path else {
                eprintln!("ImageMatch: item has no image_path: {target}");
                return false;
            };

            let Some(base_path) = &project.path else {
                eprintln!("ImageMatch: project.path is None");
                return false;
            };

            let full_path = format!("{}/image/{}", base_path, image_path);

            let template = match image::open(&full_path) {
                Ok(img) => img.to_luma8(),
                Err(err) => {
                    eprintln!(
                        "{}",
                        fl!("message-automation-loop-error-fail-load-template", error = err.to_string())
                    );
                    return false;
                }
            };

            let frame_image = match capture::capture_from_hwnd(HWND(hwnd_ptr as *mut std::ffi::c_void), setting.capture_type) {
                Ok(img) => img,
                Err(err) => {
                    eprintln!("{}", fl!("message-automation-loop-error-fail-capture", error = err));
                    return false;
                }
            };

            let gray_frame = frame_image.to_luma8();

            let (_x, _y, score) = match_template(&gray_frame, &template, &item.roi);

            score <= setting.threshold
        }
    }
}

pub fn apply_var_op(project: &mut Project, name: &str, op: &VarOp, rhs: &VarValue) -> Result<VarValue, String> {
    let var = project
        .user_variables
        .iter_mut()
        .find(|uv| uv.name.eq_ignore_ascii_case(name))
        .ok_or_else(|| format!("variable not found: {name}"))?;

    let lhs_val = UserVariable::parse_value(&var.ty, &var.value)?;

    let new_val = match (op, lhs_val, rhs.clone()) {
        (VarOp::Assign, _, r) => r,

        (VarOp::Add, VarValue::Int(l), VarValue::Int(r)) => VarValue::Int(l + r),
        (VarOp::Add, VarValue::Float(l), VarValue::Float(r)) => VarValue::Float(l + r),
        (VarOp::Sub, VarValue::Int(l), VarValue::Int(r)) => VarValue::Int(l - r),
        (VarOp::Sub, VarValue::Float(l), VarValue::Float(r)) => VarValue::Float(l - r),
        (VarOp::Mul, VarValue::Int(l), VarValue::Int(r)) => VarValue::Int(l * r),
        (VarOp::Mul, VarValue::Float(l), VarValue::Float(r)) => VarValue::Float(l * r),
        (VarOp::Div, VarValue::Int(l), VarValue::Int(r)) if r != 0 => VarValue::Int(l / r),
        (VarOp::Div, VarValue::Float(l), VarValue::Float(r)) if r != 0.0 => VarValue::Float(l / r),

        (VarOp::Add, VarValue::Str(l), VarValue::Str(r)) => VarValue::Str(format!("{l}{r}")),

        (VarOp::Toggle, VarValue::Bool(b), _) => VarValue::Bool(!b),

        _ => return Err("type mismatch or invalid operation".into()),
    };

    var.value = match &new_val {
        VarValue::Bool(b) => b.to_string(),
        VarValue::Int(i) => i.to_string(),
        VarValue::Float(f) => f.to_string(),
        VarValue::Str(s) => s.clone(),
    };

    Ok(new_val)
}
