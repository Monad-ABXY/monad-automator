use egui::{Align, ComboBox, Context, Id, Modal, ScrollArea, Ui};

use crate::{
    core::{
        action::{ActionType, KeyCode, KeyType},
        expression::{CmpOp, Condition, Operand, VarOp},
    },
    fl,
    gui::app::MyApp,
};

pub fn show_add_action_modal(ctx: &Context, app: &mut MyApp) {
    Modal::new(Id::new("action_modal"))
        .frame(egui::Frame::popup(&ctx.style()))
        .show(ctx, |ui| {
            ui.vertical_centered(|ui| {
                show_add_action_modal_header(ui, app);

                show_action_type_selector(ui, app);

                ui.separator();

                show_action_type_editor(ui, app);

                ui.add_space(8.0);
                ui.separator();

                show_confirm_cancel_ui(ctx, ui, app);
            });
        });
}

fn show_add_action_modal_header(ui: &mut Ui, app: &mut MyApp) {
    match app.edit_action {
        Some(_) => {
            ui.heading(fl!("add-action-modal-heading-edit"));
        }
        None => {
            ui.heading(fl!("add-action-modal-heading-add"));
        }
    }
}

fn show_action_type_selector(ui: &mut Ui, app: &mut MyApp) {
    ComboBox::from_label("")
        .selected_text(label_for_action_type(&app.action_type))
        .show_ui(ui, |ui| {
            ui.selectable_value(
                &mut app.action_type,
                ActionType::LeftClick {
                    x: 0,
                    y: 0,
                    use_matched_position: false,
                    delay: 50,
                },
                fl!("action-panel-left-click"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::RightClick {
                    x: 0,
                    y: 0,
                    use_matched_position: false,
                    delay: 50,
                },
                fl!("action-panel-right-click"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::Drag {
                    start: (0, 0),
                    end: (0, 0),
                    duration_ms: 500,
                },
                fl!("action-panel-drag"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::Scroll {
                    x: 0,
                    y: 0,
                    delta: 120,
                    use_matched_position: false,
                },
                fl!("action-panel-scroll"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::KeyInput {
                    keys: vec![KeyType::default()],
                },
                fl!("action-panel-key-input"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::TextInput { text: "".to_string() },
                fl!("action-panel-text-input"),
            );
            ui.selectable_value(&mut app.action_type, ActionType::Delay { millis: 500 }, fl!("action-panel-delay"));
            ui.selectable_value(
                &mut app.action_type,
                ActionType::SendDiscord {
                    webhook_url: "".into(),
                    message: "".into(),
                    use_screenshot: false,
                },
                fl!("action-panel-send-discord"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::ConditionCheck {
                    condition: Condition::Cmp {
                        left: Operand::Var("".to_string()),
                        op: CmpOp::Eq,
                        right: Operand::Var("".to_string()),
                    },
                },
                fl!("action-panel-check-condition"),
            );
            ui.selectable_value(
                &mut app.action_type,
                ActionType::VariableValueChange {
                    name: "".to_string(),
                    op: VarOp::Assign,
                    value: Operand::Var("".to_string()),
                },
                fl!("action-panel-variable"),
            );
        });
}

fn show_action_type_editor(ui: &mut Ui, app: &mut MyApp) {
    match &mut app.action_type {
        ActionType::LeftClick {
            x,
            y,
            use_matched_position,
            delay,
        } => {
            ui.horizontal(|ui| {
                ui.add_enabled_ui(!*use_matched_position, |ui| {
                    ui.label("x: ");
                    ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));
    
                    ui.label("y: ");
                    ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));
                });
                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(fl!("action-panel-delay"));
                ui.add(egui::DragValue::new(delay).range(0..=usize::MAX).speed(1));
                ui.label(fl!(
                    "action-panel-label-millis-with-seconds",
                    seconds = format!("{}", *delay as f64 / 1000.0)
                ));
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }
        ActionType::RightClick {
            x,
            y,
            use_matched_position,
            delay,
        } => {
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));

                ui.label("y: ");
                ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));
                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            ui.separator();
            ui.horizontal(|ui| {
                ui.label(fl!("action-panel-delay"));
                ui.add(egui::DragValue::new(delay).range(0..=usize::MAX).speed(1));
                ui.label(fl!(
                    "action-panel-label-millis-with-seconds",
                    seconds = format!("{}", *delay as f64 / 1000.0)
                ));
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }
        ActionType::Drag { start, end, duration_ms } => {
            ui.horizontal(|ui| {
                ui.label("start: ");
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut start.0).range(0..=usize::MAX).speed(1));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut start.1).range(0..=usize::MAX).speed(1));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    start.0 = pick_x;
                    start.1 = pick_y;
                }
            }
            ui.horizontal(|ui| {
                ui.label("end: ");
                ui.label("x: ");
                ui.add(egui::DragValue::new(&mut end.0).range(0..=usize::MAX).speed(1));
                ui.label("y: ");
                ui.add(egui::DragValue::new(&mut end.1).range(0..=usize::MAX).speed(1));
            });
            ui.horizontal(|ui| {
                ui.label(fl!("action-panel-duration-ms"));
                ui.add(egui::DragValue::new(duration_ms).range(0..=usize::MAX).speed(1));
            });
        }
        ActionType::Scroll {
            x,
            y,
            delta,
            use_matched_position,
        } => {
            ui.horizontal(|ui| {
                let s = if *delta == 120 {
                    fl!("action-panel-scroll-direction-option-up").to_string()
                } else {
                    fl!("action-panel-scroll-direction-option-down").to_string()
                };
                ComboBox::from_label("").selected_text(s.to_string()).show_ui(ui, |ui| {
                    ui.selectable_value(delta, 120, fl!("action-panel-scroll-direction-option-up"));
                    ui.selectable_value(delta, -120, fl!("action-panel-scroll-direction-option-down"));
                });
            });
            ui.horizontal(|ui| {
                ui.label("x: ");
                ui.add(egui::DragValue::new(x).range(0..=usize::MAX).speed(1));

                ui.label("y: ");
                ui.add(egui::DragValue::new(y).range(0..=usize::MAX).speed(1));

                ui.checkbox(use_matched_position, fl!("action-panel-checkbox-use-matched-position"));
            });
            ui.horizontal(|ui| {
                if ui.button(fl!("action-panel-button-open-coordinate-picker")).clicked() {
                    app.picked_position = None;
                    app.coordinate_picker_enable = true;
                }
            });
            if app.coordinate_picker_enable {
                if let Some((pick_x, pick_y)) = app.picked_position {
                    *x = pick_x;
                    *y = pick_y;
                }
            }
        }
        ActionType::KeyInput { keys } => {
            ScrollArea::vertical().max_height(30.0).show(ui, |ui| {
                ui.horizontal(|ui| {
                    let columns = 8;
                    egui::Grid::new("key_grid").num_columns(columns).show(ui, |ui| {
                        for (i, key) in keys.iter().enumerate() {
                            let enable = Some(i) != app.key_index;
                            ui.add_enabled_ui(enable, |ui| {
                                let button = ui.button(format!("{key}"));
                                if button.clicked() {
                                    app.key_index = Some(i);
                                    ui.scroll_to_rect(button.rect, Some(Align::Center));
                                }
                                if i == keys.len() - 1 && app.scroll_to_bottom {
                                    ui.scroll_to_rect(button.rect, Some(Align::Center));
                                    app.scroll_to_bottom = false;
                                }
                            });
                            if (i + 1) % columns == 0 {
                                ui.end_row();
                            }
                        }
                    });
                });
            });
            ui.separator();
            if ui.button(fl!("action-panel-key-add")).clicked() {
                app.key_index = match app.key_index {
                    Some(index) => {
                        keys.push(keys[index]);
                        Some(keys.len() - 1)
                    }
                    None => {
                        keys.push(KeyType::DownAndUp(KeyCode::A));
                        Some(0)
                    }
                };
                app.scroll_to_bottom = true;
            }

            ui.horizontal(|ui| {
                if let Some(index) = app.key_index {
                    let key_code = match keys[index] {
                        KeyType::Down(key_code) => key_code,
                        KeyType::Up(key_code) => key_code,
                        KeyType::DownAndUp(key_code) => key_code,
                        KeyType::Delay(_) => KeyCode::A,
                    };
                    let key_type = match keys[index] {
                        KeyType::Down(_) => fl!("action-panel-key-type-down"),
                        KeyType::Up(_) => fl!("action-panel-key-type-up"),
                        KeyType::DownAndUp(_) => fl!("action-panel-key-type-down-and-up"),
                        KeyType::Delay(_) => fl!("action-panel-key-type-delay"),
                    };
                    ComboBox::new("key type", "").selected_text(key_type).show_ui(ui, |ui| {
                        ui.selectable_value(&mut keys[index], KeyType::Down(key_code), fl!("action-panel-key-type-down"));
                        ui.selectable_value(&mut keys[index], KeyType::Up(key_code), fl!("action-panel-key-type-up"));
                        ui.selectable_value(
                            &mut keys[index],
                            KeyType::DownAndUp(key_code),
                            fl!("action-panel-key-type-down-and-up"),
                        );
                        ui.selectable_value(&mut keys[index], KeyType::Delay(500), fl!("action-panel-key-type-delay"));
                    });

                    match &mut keys[index] {
                        KeyType::Down(ref mut current) | KeyType::Up(ref mut current) | KeyType::DownAndUp(ref mut current) => {
                            ComboBox::new("key code", "").selected_text(current.to_string()).show_ui(ui, |ui| {
                                for k in [
                                    KeyCode::A,
                                    KeyCode::B,
                                    KeyCode::C,
                                    KeyCode::D,
                                    KeyCode::E,
                                    KeyCode::F,
                                    KeyCode::G,
                                    KeyCode::H,
                                    KeyCode::I,
                                    KeyCode::J,
                                    KeyCode::K,
                                    KeyCode::L,
                                    KeyCode::M,
                                    KeyCode::N,
                                    KeyCode::O,
                                    KeyCode::P,
                                    KeyCode::Q,
                                    KeyCode::R,
                                    KeyCode::S,
                                    KeyCode::T,
                                    KeyCode::U,
                                    KeyCode::V,
                                    KeyCode::W,
                                    KeyCode::X,
                                    KeyCode::Y,
                                    KeyCode::Z,
                                    KeyCode::Num0,
                                    KeyCode::Num1,
                                    KeyCode::Num2,
                                    KeyCode::Num3,
                                    KeyCode::Num4,
                                    KeyCode::Num5,
                                    KeyCode::Num6,
                                    KeyCode::Num7,
                                    KeyCode::Num8,
                                    KeyCode::Num9,
                                    KeyCode::NumPad0,
                                    KeyCode::NumPad1,
                                    KeyCode::NumPad2,
                                    KeyCode::NumPad3,
                                    KeyCode::NumPad4,
                                    KeyCode::NumPad5,
                                    KeyCode::NumPad6,
                                    KeyCode::NumPad7,
                                    KeyCode::NumPad8,
                                    KeyCode::NumPad9,
                                    KeyCode::Enter,
                                    KeyCode::Escape,
                                    KeyCode::Backspace,
                                    KeyCode::Tab,
                                    KeyCode::Space,
                                    KeyCode::Shift,
                                    KeyCode::Ctrl,
                                    KeyCode::Alt,
                                    KeyCode::Left,
                                    KeyCode::Up,
                                    KeyCode::Right,
                                    KeyCode::Down,
                                    KeyCode::F1,
                                    KeyCode::F2,
                                    KeyCode::F3,
                                    KeyCode::F4,
                                    KeyCode::F5,
                                    KeyCode::F6,
                                    KeyCode::F7,
                                    KeyCode::F8,
                                    KeyCode::F9,
                                    KeyCode::F10,
                                    KeyCode::F11,
                                    KeyCode::F12,
                                    KeyCode::Delete,
                                    KeyCode::Insert,
                                    KeyCode::Home,
                                    KeyCode::End,
                                    KeyCode::PageUp,
                                    KeyCode::PageDown,
                                ] {
                                    if ui.selectable_label(*current == k, format!("{k}")).clicked() {
                                        *current = k;
                                    }
                                }
                                if ui.selectable_label(matches!(current, KeyCode::Custom(_)), "Custom...").clicked() {
                                    *current = KeyCode::Custom(0);
                                }
                            });
                            if let KeyCode::Custom(ref mut val) = current {
                                ui.horizontal(|ui| {
                                    ui.hyperlink_to(
                                        fl!("action-panel-key-custom-vk"),
                                        "https://learn.microsoft.com/en-us/windows/win32/inputdev/virtual-key-codes",
                                    );
                                    ui.add(egui::DragValue::new(val).range(0..=255).speed(1));
                                });
                            }
                        }
                        KeyType::Delay(delay) => {
                            ui.add(egui::DragValue::new(delay).range(0..=u32::MAX).speed(1));
                        }
                    }
                    if ui.button(fl!("action-panel-context-delete")).clicked() {
                        app.key_index = None;
                        keys.remove(index);
                    }
                }
            });
        }
        ActionType::TextInput { text } => {
            ui.text_edit_multiline(text);
        }
        ActionType::Delay { millis } => {
            ui.horizontal(|ui| {
                ui.add(egui::DragValue::new(millis).range(0..=usize::MAX).speed(1));
                ui.label(fl!(
                    "action-panel-label-millis-with-seconds",
                    seconds = format!("{}", *millis as f64 / 1000.0)
                ));
            });
        }
        ActionType::SendDiscord {
            webhook_url,
            message,
            use_screenshot,
        } => {
            ui.label(fl!("action-panel-label-webhook-url"));

            ScrollArea::both().max_width(ui.available_width()).show(ui, |ui| {
                ui.text_edit_singleline(webhook_url);
            });
            ui.label(fl!("action-panel-label-message"));
            ui.text_edit_singleline(message);
            ui.checkbox(use_screenshot, fl!("action-panel-send-screenshot"));
        }
        ActionType::ConditionCheck { condition } => {
            let kind_text = match condition {
                Condition::Cmp { .. } => fl!("action-panel-cond-kind-compare"),
                Condition::ImageMatch { .. } => fl!("action-panel-cond-kind-image-match"),
                Condition::All(..) => todo!(),
                Condition::Any(..) => todo!(),
                Condition::Not(..) => todo!(),
            };
            egui::ComboBox::new("cond_kind", "").selected_text(kind_text).show_ui(ui, |ui| {
                if ui
                    .selectable_label(matches!(condition, Condition::Cmp { .. }), fl!("action-panel-cond-kind-compare"))
                    .clicked()
                    && !matches!(condition, Condition::Cmp { .. })
                {
                    *condition = Condition::Cmp {
                        left: Operand::Var(String::new()),
                        op: CmpOp::Eq,
                        right: Operand::LitStr(String::new()),
                    };
                }
                if ui
                    .selectable_label(
                        matches!(condition, Condition::ImageMatch { .. }),
                        fl!("action-panel-cond-kind-image-match"),
                    )
                    .clicked()
                    && !matches!(condition, Condition::ImageMatch { .. })
                {
                    *condition = Condition::ImageMatch { target: String::new() };
                }
            });

            fn cmp_op_combo(ui: &mut egui::Ui, op: &mut CmpOp) {
                egui::ComboBox::new("cmp_op", "").selected_text(op.to_string()).show_ui(ui, |ui| {
                    ui.selectable_value(op, CmpOp::Eq, "==");
                    ui.selectable_value(op, CmpOp::Ne, "!=");
                    ui.selectable_value(op, CmpOp::Lt, "<");
                    ui.selectable_value(op, CmpOp::Le, "<=");
                    ui.selectable_value(op, CmpOp::Gt, ">");
                    ui.selectable_value(op, CmpOp::Ge, ">=");
                    ui.selectable_value(op, CmpOp::Contains, "contains");
                });
            }

            match condition {
                Condition::Cmp { left, op, right } => {
                    ui.group(|ui| {
                        ui.horizontal(|ui| {
                            ui.label(fl!("action-panel-label-left"));
                            operand_editor(ui, &app.project, "left".to_string(), left);
                        });

                        ui.horizontal(|ui| {
                            ui.label(fl!("action-panel-label-op"));
                            cmp_op_combo(ui, op);
                        });

                        ui.horizontal(|ui| {
                            ui.label(fl!("action-panel-label-right"));
                            operand_editor(ui, &app.project, "right".to_string(), right);
                        });

                        if matches!(op, CmpOp::Contains) {
                            ui.weak(fl!("action-panel-hint-contains"));
                        }
                    });
                }
                Condition::ImageMatch { target } => {
                    ui.group(|ui| {
                        ui.label(fl!("action-panel-label-target"));

                        let mut selected = target.clone();
                        egui::ComboBox::new("image_target_combo", "")
                            .selected_text(if selected.is_empty() { "<image/item>" } else { &selected })
                            .show_ui(ui, |ui| {
                                for it in &app.project.items {
                                    if it.image_path.is_some() && ui.selectable_label(selected == it.name, &it.name).clicked() {
                                        selected = it.name.clone();
                                    }
                                }
                            });

                        ui.text_edit_singleline(&mut selected);
                        if &selected != target {
                            *target = selected;
                        }
                    });
                }
                Condition::All(..) => todo!(),
                Condition::Any(..) => todo!(),
                Condition::Not(..) => todo!(),
            }
        }
        ActionType::VariableValueChange { name, op, value } => {
            ui.group(|ui| {
                ui.horizontal(|ui| {
                    ui.label(fl!("action-panel-variable"));

                    let mut selected = name.clone();
                    egui::ComboBox::new("var_change_target", "")
                        .selected_text(if selected.is_empty() { "<var>" } else { &selected })
                        .show_ui(ui, |ui| {
                            for v in &app.project.user_variables {
                                if ui.selectable_label(selected.eq_ignore_ascii_case(&v.name), &v.name).clicked() {
                                    selected = v.name.clone();
                                }
                            }
                        });
                    ui.text_edit_singleline(&mut selected);
                    if &selected != name {
                        *name = selected;
                    }
                });

                ui.horizontal(|ui| {
                    ui.label(fl!("action-panel-label-operator"));
                    egui::ComboBox::new("var_op_combo", "")
                        .selected_text(op.to_string())
                        .show_ui(ui, |ui| {
                            ui.selectable_value(op, crate::core::VarOp::Assign, "=");
                            ui.selectable_value(op, crate::core::VarOp::Add, "+=");
                            ui.selectable_value(op, crate::core::VarOp::Sub, "-=");
                            ui.selectable_value(op, crate::core::VarOp::Mul, "*=");
                            ui.selectable_value(op, crate::core::VarOp::Div, "/=");
                            ui.selectable_value(op, crate::core::VarOp::Toggle, "toggle");
                        });
                });

                ui.horizontal(|ui| {
                    ui.label(fl!("action-panel-label-value"));
                    let need_rhs = !matches!(op, crate::core::VarOp::Toggle);
                    ui.add_enabled_ui(need_rhs, |ui| {
                        ui.push_id("var_change_value", |ui| {
                            operand_editor(ui, &app.project, "var_value".to_string(), value);
                        });
                    });
                    if !need_rhs {
                        ui.weak(fl!("action-panel-hint-toggle-no-value"));
                    }
                });

                if let Some(var) = app.project.user_variables.iter().find(|v| v.name.eq_ignore_ascii_case(name)) {
                    ui.weak(fl!("action-panel-hint-var-expected-type", ty = format!("{:?}", var.ty)));
                }
            });
        }
    }
}

fn show_confirm_cancel_ui(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal_centered(|ui| {
        if ui.button(fl!("add-action-modal-button-confirm")).clicked() {
            if let Some(item_index) = app.selected_item_index {
                match app.edit_action {
                    Some(action_index) => {
                        app.project.items[item_index].actions[action_index].action = app.action_type.clone();
                        app.edit_action = None;
                    }
                    None => {
                        app.project.add_action(item_index, app.action_type.clone());
                        app.project.save_file();
                    }
                }
            }
            app.show_action_modal = false;
        }
        if ui.button(fl!("add-action-modal-button-cancel")).clicked() {
            app.show_action_modal = false;
            app.edit_action = None;
        }
        if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
            app.show_action_modal = false;
            app.edit_action = None;
        }
    });
}

fn operand_editor(ui: &mut egui::Ui, project: &crate::core::Project, id: String, operand: &mut Operand) {
    ui.horizontal(|ui| {
        let current = match operand {
            Operand::Var(_) => "Var",
            Operand::LitBool(_) => "Bool",
            Operand::LitInt(_) => "Int",
            Operand::LitFloat(_) => "Float",
            Operand::LitStr(_) => "Str",
        };

        egui::ComboBox::new(id.clone(), "").selected_text(current).show_ui(ui, |ui| {
            if ui.selectable_label(matches!(operand, Operand::Var(_)), "Var").clicked() {
                *operand = Operand::Var(String::new());
            }
            if ui.selectable_label(matches!(operand, Operand::LitBool(_)), "Bool").clicked() {
                *operand = Operand::LitBool(false);
            }
            if ui.selectable_label(matches!(operand, Operand::LitInt(_)), "Int").clicked() {
                *operand = Operand::LitInt(0);
            }
            if ui.selectable_label(matches!(operand, Operand::LitFloat(_)), "Float").clicked() {
                *operand = Operand::LitFloat(0.0);
            }
            if ui.selectable_label(matches!(operand, Operand::LitStr(_)), "Str").clicked() {
                *operand = Operand::LitStr(String::new());
            }
        });

        match operand {
            Operand::Var(name) => {
                let mut selected = name.clone();
                egui::ComboBox::new(ui.make_persistent_id((id, ui.id())), "")
                    .selected_text(if selected.is_empty() { "<var>" } else { &selected })
                    .show_ui(ui, |ui| {
                        for v in &project.user_variables {
                            if ui.selectable_label(selected.eq_ignore_ascii_case(&v.name), &v.name).clicked() {
                                selected = v.name.clone();
                            }
                        }
                    });
                ui.text_edit_singleline(&mut selected);
                if &selected != name {
                    *name = selected;
                }
            }
            Operand::LitBool(b) => {
                egui::ComboBox::from_id_salt((id.as_str(), "bool_menu"))
                    .selected_text(if *b { "true" } else { "false" })
                    .show_ui(ui, |ui| {
                        if ui.selectable_label(*b, "true").clicked() {
                            *b = true;
                        }
                        if ui.selectable_label(!*b, "false").clicked() {
                            *b = false;
                        }
                    });
            }
            Operand::LitInt(i) => {
                ui.add(egui::DragValue::new(i));
            }
            Operand::LitFloat(fv) => {
                ui.add(egui::DragValue::new(fv).speed(0.1));
            }
            Operand::LitStr(s) => {
                ui.text_edit_singleline(s);
            }
        }
    });
}

fn label_for_action_type(action_type: &ActionType) -> String {
    use ActionType::*;
    match action_type {
        LeftClick { .. } => fl!("action-panel-left-click"),
        RightClick { .. } => fl!("action-panel-right-click"),
        Drag { .. } => fl!("action-panel-drag"),
        Scroll { .. } => fl!("action-panel-scroll"),
        KeyInput { .. } => fl!("action-panel-key-input"),
        TextInput { .. } => fl!("action-panel-text-input"),
        Delay { .. } => fl!("action-panel-delay"),
        SendDiscord { .. } => fl!("action-panel-send-discord"),
        ConditionCheck { .. } => fl!("action-panel-check-condition"),
        VariableValueChange { .. } => fl!("action-panel-variable"),
    }
}
