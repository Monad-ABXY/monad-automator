use egui::{RichText, ScrollArea, Ui};

use crate::{core::action::Action, fl, gui::app::MyApp};

struct Pending {
    edit: Option<(usize, Action)>,
    delete: Option<usize>,
    move_up: Option<usize>,
    move_down: Option<usize>,
}

pub fn action_panel(ui: &mut Ui, app: &mut MyApp) {
    let mut pending = Pending {
        edit: None,
        delete: None,
        move_up: None,
        move_down: None,
    };

    let enabled = !app.is_automation_running && app.selected_item_index.is_some();

    show_action_panel_header(ui, app, enabled);

    ui.add_space(2.0);

    ui.add_enabled_ui(!app.is_automation_running, |ui| {
        show_action_list_panel(ui, app, &mut pending);
    });

    handle_action_events(app, pending);
}

fn show_action_panel_header(ui: &mut Ui, app: &mut MyApp, enabled: bool) {
    ui.horizontal(|ui| {
        ui.label(RichText::new(fl!("action-panel-header")).strong());
        ui.add_enabled_ui(enabled, |ui| {
            if ui.button(fl!("action-panel-button-add-action")).clicked() {
                app.show_action_modal = true;
                app.key_index = None;
            }
        });
    });
}

fn show_action_list_panel(ui: &mut Ui, app: &mut MyApp, pending: &mut Pending) {
    ScrollArea::vertical().max_height(ui.available_height()).show(ui, |ui| {
        ui.with_layout(egui::Layout::top_down(egui::Align::TOP), |ui| {
            ui.set_width(ui.available_width());
            if let Some(item_index) = app.selected_item_index {
                let actions = &mut app.project.items[item_index].actions;
                for (index, action) in actions.iter_mut().enumerate() {
                    ui.horizontal(|ui| {
                        let checkbox = ui
                            .checkbox(&mut action.enabled, action.action.to_localized_string())
                            .on_hover_text(format!("{:?}", action.action));

                        checkbox.context_menu(|ui| {
                            if ui.button(fl!("action-panel-context-edit")).clicked() {
                                pending.edit = Some((index, action.clone()));
                                ui.close();
                            }
                            if ui.button(fl!("action-panel-context-delete")).clicked() {
                                pending.delete = Some(index);
                                ui.close();
                            }
                            if ui.button(fl!("action-panel-context-move-up")).clicked() {
                                pending.move_up = Some(index);
                                ui.close();
                            }
                            if ui.button(fl!("action-panel-context-move-down")).clicked() {
                                pending.move_down = Some(index);
                                ui.close();
                            }
                        });
                    });
                }
            }
        });
    });
}

fn handle_action_events(app: &mut MyApp, pending: Pending) {
    if let Some(item_index) = app.selected_item_index {
        if let Some((index, action)) = pending.edit {
            app.action_type = action.action;
            app.edit_action = Some(index);
            app.show_action_modal = true;
        }

        if let Some(index) = pending.delete {
            app.project.items[item_index].actions.remove(index);
            app.project.save_file();
        }

        if let Some(index) = pending.move_up {
            if index > 0 && index < app.project.items[item_index].actions.len() {
                app.project.items[item_index].actions.swap(index, index - 1);
                app.project.save_file();
            }
        }

        if let Some(index) = pending.move_down {
            if index + 1 < app.project.items[item_index].actions.len() {
                app.project.items[item_index].actions.swap(index, index + 1);
                app.project.save_file();
            }
        }
    }
}
