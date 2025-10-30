use std::{
    fmt,
    fs::File,
    io::{Read, Write},
    path::{Path, PathBuf},
};

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Setting {
    pub language: Language,
    pub theme: Theme,
    pub capture_type: CaptureType,
    pub input_type: InputType,
    pub loop_per_second: u32,
    pub threshold: f32,
    pub last_project_path: Option<String>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Language {
    EnUS,
    KoKR,
    JaJP,
    ZhCN,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum Theme {
    Dark,
    Light,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum CaptureType {
    BitBlt,
    PrintWindow,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum InputType {
    PostMessage,
    SendMessage,
    SendInput,
}

impl Setting {
    pub fn new() -> Self {
        Setting {
            language: Language::EnUS,
            theme: Theme::Dark,
            capture_type: CaptureType::PrintWindow,
            input_type: InputType::PostMessage,
            loop_per_second: 15,
            threshold: 0.01,
            last_project_path: None,
        }
    }

    pub fn save_to_ron(&self, path: &PathBuf) {
        if let Ok(ron_data) = ron::ser::to_string_pretty(&self, ron::ser::PrettyConfig::new()) {
            if let Ok(mut file) = File::create(path) {
                if let Err(err) = file.write_all(ron_data.as_bytes()) {
                    eprintln!("Failed to save RON file: {err}");
                } else {
                    println!("RON file saved successfully: {path:?}");
                }
            } else {
                eprintln!("Failed to create RON file: {path:?}");
            }
        } else {
            eprintln!("Failed to convert to RON");
        }
    }

    pub fn save(&self) {
        let path = std::env::current_exe().unwrap().with_file_name("settings.ron");

        let exe_dir = std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|d| d.to_path_buf()))
            .unwrap_or_else(|| PathBuf::from("."));

        if let Some(p_str) = &self.last_project_path {
            let project_path = Path::new(p_str);
            if let Ok(abs_path) = std::fs::canonicalize(project_path) {
                if let Ok(abs_exe) = std::fs::canonicalize(&exe_dir) {
                    if abs_path.starts_with(&abs_exe) {
                        let rel = abs_path.strip_prefix(&abs_exe).unwrap().to_string_lossy().into_owned();
                        println!("Saving relative project path: {rel}");
                        let mut s = self.clone();
                        s.last_project_path = Some(rel);
                        s.save_to_ron(&path);
                        return;
                    }
                }
            }
        }
        self.save_to_ron(&path);
    }

    fn load_from_ron(path: PathBuf) -> Result<Setting, String> {
        let mut file = File::open(&path).map_err(|e| format!("Failed to open file: {e}"))?;
        let mut data = String::new();
        file.read_to_string(&mut data).map_err(|e| format!("Failed to read file: {e}"))?;

        let setting: Setting = ron::from_str(&data).map_err(|e| format!("Failed to parse RON: {e}"))?;

        Ok(setting)
    }

    pub fn load() -> Self {
        let path = std::env::current_exe().unwrap().with_file_name("settings.ron");
        match Self::load_from_ron(path) {
            Ok(setting) => setting,
            Err(err) => {
                eprintln!("{err}");
                println!("Creating new...");
                let setting = Setting::new();
                setting.save();
                setting
            }
        }
    }
}

impl fmt::Display for Language {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        let langid = match self {
            Language::EnUS => "en-US",
            Language::KoKR => "ko-KR",
            Language::JaJP => "ja-JP",
            Language::ZhCN => "zh-CN",
        };
        write!(f, "{langid}")
    }
}
