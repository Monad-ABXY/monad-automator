use egui::{Context, Ui};
use egui_extras::{Column, TableBuilder};

use crate::{
    core::{Project, UserVariable, VarType},
    fl,
    gui::app::MyApp,
};

pub fn show_variables_viewport(ctx: &Context, app: &mut MyApp) {
    ctx.show_viewport_immediate(
        egui::ViewportId::from_hash_of("image_edit_viewport"),
        egui::ViewportBuilder::default()
            .with_title("Variables Manage")
            .with_inner_size([520.0, 320.0])
            .with_resizable(true),
        |ctx, class| {
            assert!(
                class == egui::ViewportClass::Immediate,
                "This egui backend doesn't support multiple viewports"
            );

            egui::CentralPanel::default().show(ctx, |ui| {
                ui.heading(fl!("variables-viewport-heading"));

                ui.horizontal(|ui| {
                    show_add_button(ui, app);
                    show_save_button(ui, app);
                });

                ui.add_space(8.0);

                show_variables_table(ui, &mut app.project);
            });

            if ctx.input(|i| i.viewport().close_requested()) {
                app.show_variables_viewport = false;
            }
            if ctx.input(|i| i.key_pressed(egui::Key::Escape)) {
                app.show_variables_viewport = false;
            }
            if app.is_automation_running {
                app.show_variables_viewport = false;
            }
        },
    );
}

fn show_add_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("variables-viewport-button-add-var")).clicked() {
        app.project.user_variables.push(UserVariable {
            id: uuid::Uuid::new_v4(),
            name: format!("var{}", app.project.user_variables.len() + 1),
            ty: VarType::Str,
            value: String::new(),
        });
        app.project.save_file();
    }
}

fn show_save_button(ui: &mut Ui, app: &mut MyApp) {
    if ui.button(fl!("variables-viewport-button-save")).clicked() {
        app.project.save_file();
    }
}

pub fn show_variables_table(ui: &mut Ui, project: &mut Project) {
    let row_h = egui::TextStyle::Body.resolve(ui.style()).size.max(ui.spacing().interact_size.y);

    let mut to_delete: Option<usize> = None;
    let mut save_needed = false;

    let avail_w = ui.available_width();
    ui.set_width(avail_w);

    let table = TableBuilder::new(ui)
        .striped(true)
        .resizable(true)
        .cell_layout(egui::Layout::left_to_right(egui::Align::Center))
        .column(Column::remainder().at_least(80.0).resizable(true))
        .column(Column::auto().at_least(80.0))
        .column(Column::remainder().at_least(110.0).resizable(true))
        .column(Column::auto().at_least(80.0));

    table
        .header(24.0, |mut header| {
            header.col(|ui| {
                ui.strong(fl!("variables-viewport-var-name"));
            });
            header.col(|ui| {
                ui.strong(fl!("variables-viewport-var-kind"));
            });
            header.col(|ui| {
                ui.strong(fl!("variables-viewport-var-value"));
            });
            header.col(|ui| {
                ui.strong(fl!("variables-viewport-actions"));
            });
        })
        .body(|body| {
            let rows = project.user_variables.len().max(1);
            body.rows(row_h, rows, |mut row| {
                let i = row.index();

                if let Some(var) = project.user_variables.get_mut(i) {
                    row.col(|ui| {
                        if ui.text_edit_singleline(&mut var.name).changed() {
                            save_needed = true;
                        }
                    });

                    row.col(|ui| {
                        egui::ComboBox::from_id_salt(("var_kind", i))
                            .selected_text(match var.ty {
                                VarType::Bool => "Bool",
                                VarType::Int => "Int",
                                VarType::Float => "Float",
                                VarType::Str => "Str",
                            })
                            .show_ui(ui, |ui| {
                                let mut changed = false;
                                changed |= ui.selectable_value(&mut var.ty, VarType::Bool, "Bool").changed();
                                changed |= ui.selectable_value(&mut var.ty, VarType::Int, "Int").changed();
                                changed |= ui.selectable_value(&mut var.ty, VarType::Float, "Float").changed();
                                changed |= ui.selectable_value(&mut var.ty, VarType::Str, "Str").changed();
                                if changed {
                                    save_needed = true;
                                }
                            });
                    });

                    row.col(|ui| {
                        let resp = ui.text_edit_singleline(&mut var.value);
                        let parse_ok = UserVariable::parse_value(&var.ty, &var.value).is_ok();
                        if !parse_ok {
                            ui.colored_label(egui::Color32::RED, fl!("variables-viewport-warning-type-value-mismatch"));
                        }
                        if resp.changed() {
                            save_needed = true;
                        }
                    });

                    row.col(|ui| {
                        if ui.button(fl!("variables-viewport-button-delete")).clicked() {
                            to_delete = Some(i);
                        }
                    });
                } else {
                    row.col(|ui| {
                        ui.weak("—");
                    });
                    row.col(|ui| {
                        ui.weak("—");
                    });
                    row.col(|ui| {
                        ui.weak("—");
                    });
                    row.col(|ui| {
                        ui.weak("—");
                    });
                }
            });
        });

    if let Some(i) = to_delete {
        project.user_variables.remove(i);
        save_needed = true;
    }
    if save_needed {
        project.save_file();
    }
}
