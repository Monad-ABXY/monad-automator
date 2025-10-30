use egui::{
    text_edit::{TextEdit, TextEditOutput},
    Color32, Frame, Label, Margin, RichText, ScrollArea, Sense, Ui,
};

use crate::{fl, gui::app::MyApp};

struct Pending {
    rename: Option<(usize, String)>,
    delete: Option<usize>,
    move_up: Option<usize>,
    move_down: Option<usize>,
    duplicate: Option<usize>,
}

pub fn image_list_panel(ui: &mut Ui, app: &mut MyApp) {
    let mut pending = Pending {
        rename: None,
        delete: None,
        move_up: None,
        move_down: None,
        duplicate: None,
    };

    show_image_list_header(ui, app);

    ui.add_space(2.0);

    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        show_image_list(ui, app, &mut pending);
    });

    handle_action_events(app, pending);
}

fn show_image_list_header(ui: &mut Ui, app: &mut MyApp) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(fl!("image-list-panel-label")).strong());
        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            if ui.button(fl!("image-list-panel-button-add-image")).clicked() {
                let name = app.project.next_name();
                app.project.add_action_item(&name, None, 0, 0, 0, 0);
                app.project.save_file();
            }
        });

        ui.add_enabled_ui(!app.is_automation_running, |ui| {
            if ui.button(fl!("image-list-panel-button-manage-variables")).clicked() {
                app.show_variables_viewport = true;
            }
        });
    });
}

fn show_image_list(ui: &mut Ui, app: &mut MyApp, pending: &mut Pending) {
    ScrollArea::vertical().max_height(ui.available_height()).show(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.set_width(ui.available_width());

            for (index, item) in &mut app.project.items.iter_mut().enumerate() {
                let bg = if app.selected_item_index == Some(index) {
                    ui.visuals().selection.bg_fill
                } else {
                    Color32::TRANSPARENT
                };

                Frame::new().fill(bg).inner_margin(Margin::same(4)).show(ui, |ui| {
                    ui.horizontal(|ui| {
                        ui.checkbox(&mut item.enabled, "").context_menu(|ui| {
                            if ui.button(fl!("image-list-panel-context-rename")).clicked() {
                                app.renaming = Some(index);
                                app.rename_buffer = item.name.clone();
                                ui.close();
                            }
                            if ui.button(fl!("image-list-panel-context-delete")).clicked() {
                                pending.delete = Some(index);
                                ui.close();
                            }
                            if ui.button(fl!("image-list-panel-context-move-up")).clicked() {
                                pending.move_up = Some(index);
                                ui.close();
                            }
                            if ui.button(fl!("image-list-panel-context-move-down")).clicked() {
                                pending.move_down = Some(index);
                                ui.close();
                            }
                            if ui.button(fl!("image-list-panel-context-duplicate")).clicked() {
                                pending.duplicate = Some(index);
                                ui.close();
                            }
                        });

                        if Some(index) == app.renaming {
                            let text_output: TextEditOutput = TextEdit::singleline(&mut app.rename_buffer).show(ui);
                            if text_output.response.lost_focus() {
                                if let Some(_i) = app.renaming {
                                    pending.rename = Some((index, app.rename_buffer.clone()));
                                    app.renaming = None;
                                }
                            }
                        } else {
                            let label = ui
                                .add(Label::new(&item.name).sense(Sense::click()))
                                .on_hover_text(format!("{item:#?}"));
                            label.context_menu(|ui| {
                                if ui.button(fl!("image-list-panel-context-rename")).clicked() {
                                    app.renaming = Some(index);
                                    app.rename_buffer = item.name.clone();
                                    ui.close();
                                }
                                if ui.button(fl!("image-list-panel-context-delete")).clicked() {
                                    pending.delete = Some(index);
                                    ui.close();
                                }
                                if ui.button(fl!("image-list-panel-context-move-up")).clicked() {
                                    pending.move_up = Some(index);
                                    ui.close();
                                }
                                if ui.button(fl!("image-list-panel-context-move-down")).clicked() {
                                    pending.move_down = Some(index);
                                    ui.close();
                                }
                                if ui.button(fl!("image-list-panel-context-duplicate")).clicked() {
                                    pending.duplicate = Some(index);
                                    ui.close();
                                }
                            });

                            if label.clicked() {
                                app.selected_item_index = Some(index);
                                app.selected_item = Some(item.clone());
                                app.needs_image_update = true;
                            }
                        }
                    });
                });
            }
        });
    });
}

fn handle_action_events(app: &mut MyApp, pending: Pending) {
    if let Some((i, new_name)) = pending.rename {
        app.project.items[i].name = new_name;
    }
    if let Some(index) = pending.delete {
        app.project.items.remove(index);
        if app.selected_item_index == Some(index) {
            app.selected_item_index = None;
        }
        app.project.save_file();
        app.needs_image_update = true;
    }
    if let Some(index) = pending.move_up {
        if index > 0 && index < app.project.items.len() {
            app.project.items.swap(index, index - 1);
            if app.selected_item_index == Some(index) {
                app.selected_item_index = None;
            }
            app.project.save_file();
        }
    }
    if let Some(index) = pending.move_down {
        if index + 1 < app.project.items.len() {
            app.project.items.swap(index, index + 1);
            if app.selected_item_index == Some(index) {
                app.selected_item_index = None;
            }
            app.project.save_file();
        }
    }
    if let Some(index) = pending.duplicate {
        if index < app.project.items.len() {
            let mut new_item = app.project.items[index].clone();

            let new_name = app.project.next_name();
            new_item.name = new_name;

            let insert_at = index + 1;
            app.project.items.insert(insert_at, new_item);

            app.selected_item_index = Some(insert_at);
            app.selected_item = Some(app.project.items[insert_at].clone());
            app.needs_image_update = true;

            app.project.save_file();
        }
    }
}
