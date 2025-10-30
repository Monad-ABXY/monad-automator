use egui::{Context, Ui};

use crate::{
    core::{Language, Project},
    fl,
    gui::app::MyApp,
    i18n::change_language,
};

pub fn menu(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    egui::MenuBar::new().ui(ui, |ui| {
        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            ui.menu_button(fl!("menu-file"), |ui| {
                new_button(ui, app);
                open_button(ui, app);
                save_button(ui, app);
            });

            ui.menu_button(fl!("menu-menu"), |ui| {
                language_button(ui, app);
                theme_button(ctx, ui, app);
                update_button(ui);
                help_button(ui);
                quit_button(ui);
            });
        });
    });
}

fn new_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-new")).clicked() {
        ui.close();
        match Project::make_new_project() {
            Ok(project) => {
                *app = MyApp::new(app.setting.clone());
                app.project = project.clone();
                app.setting.last_project_path = format!("{}/project.json", project.path.unwrap()).into();
                app.setting.save();
            }
            Err(err) => {
                eprintln!("{err}");
                app.error_message = Some(err);
            }
        }
    }
}

fn open_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-open")).clicked() {
        match Project::load() {
            Ok(project) => {
                *app = MyApp::new(app.setting.clone());
                app.project = project.clone();
                app.setting.last_project_path = format!("{}/project.json", project.path.unwrap()).into();
                app.setting.save();
            }
            Err(error) => {
                println!("{error}");
                app.error_message = Some(error);
            }
        }
        ui.close();
    }
}

fn save_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("menu-file-save")).clicked() {
        app.project.save_file();
        ui.close();
    }
}

fn language_button(ui: &mut Ui, app: &mut MyApp) {
    let languages = [
        ("English", Language::EnUS),
        ("한국어/Korean", Language::KoKR),
        ("日本語/Japanese", Language::JaJP),
        ("简体中文/Simplified Chinese", Language::ZhCN),
    ];

    ui.menu_button(fl!("menu-lang"), |ui| {
        for (label, lang_enum) in languages {
            if ui.button(label).clicked() {
                if let Err(e) = change_language(&lang_enum.to_string()) {
                    eprintln!("Failed to change language: {e}");
                }
                app.setting.language = lang_enum;
                app.setting.save();
                ui.close();
            }
        }
    });
}

fn theme_button(ctx: &Context, ui: &mut Ui, app: &mut MyApp) {
    let theme = [
        (fl!("menu-theme-dark"), crate::core::Theme::Dark),
        (fl!("menu-theme-light"), crate::core::Theme::Light),
    ];

    ui.menu_button(fl!("menu-theme"), |ui| {
        for (label, theme_enum) in theme {
            if ui.button(label).clicked() {
                app.setting.theme = theme_enum;
                if theme_enum == crate::core::Theme::Dark {
                    ctx.set_theme(egui::Theme::Dark);
                }
                if theme_enum == crate::core::Theme::Light {
                    ctx.set_theme(egui::Theme::Light);
                }
                app.setting.save();
                ui.close();
            }
        }
    });
}

fn update_button(ui: &mut Ui) {
    ui.hyperlink_to(fl!("menu-check-updates"), "https://github.com/Monad-ABXY/monad-automator/releases");
}
fn help_button(ui: &mut Ui) {
    ui.hyperlink_to(fl!("menu-help"), "https://github.com/Monad-ABXY/monad-automator");
}
fn quit_button(ui: &mut Ui) {
    if ui.button(fl!("menu-quit")).clicked() {
        std::process::exit(0);
    }
}
