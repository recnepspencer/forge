use eframe::egui;
use egui::{Color32, CornerRadius, Frame, Margin, RichText, Stroke};
use forge_ui_components::{
    dropdown::{fg_dropdown, DropdownItem},
    fg_button, fg_chip, fg_input, fg_modal,
    icon_button::fg_icon_button,
    FgButton, FgButtonSize, FgButtonVariant, FgChip, FgIcon, FgInput,
};
use forge_ui_theme::ForgeTheme;

use crate::showcase::TodoShowcaseApp;
use crate::todo::task::Task;

const SIDEBAR_WIDTH: f32 = 236.0;

impl eframe::App for TodoShowcaseApp {
    fn update(&mut self, ctx: &egui::Context, _frame: &mut eframe::Frame) {
        let theme = self.theme.clone();
        egui::SidePanel::left("sidebar")
            .exact_width(SIDEBAR_WIDTH)
            .frame(
                Frame::new()
                    .fill(theme.bg_surface)
                    .inner_margin(Margin::same(12)),
            )
            .show(ctx, |ui| {
                self.sidebar(ui);
            });

        egui::CentralPanel::default()
            .frame(
                Frame::new()
                    .fill(theme.bg_base)
                    .inner_margin(Margin::same(24)),
            )
            .show(ctx, |ui| {
                self.workspace(ui);
            });

        self.task_details_modal(ctx, &theme);

        if self.show_add_column_modal {
            let modal = fg_modal(ctx, &theme, "add_column_modal", 420.0, |ui| {
                ui.label(
                    RichText::new("Create new column")
                        .size(18.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(20.0);
                ui.label(
                    RichText::new("Column Name")
                        .size(theme.font_size_sm)
                        .strong()
                        .color(theme.text_secondary),
                );
                let response = fg_input(
                    ui,
                    &theme,
                    FgInput::new(&mut self.new_column_name).placeholder("E.g. In Review"),
                );
                ui.memory_mut(|mem| mem.request_focus(response.id));
                let enter =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let create_clicked = fg_button(
                            ui,
                            &theme,
                            &self.icons,
                            FgButton::new("Create column")
                                .variant(FgButtonVariant::Primary)
                                .size(FgButtonSize::Sm)
                                .disabled(self.new_column_name.trim().is_empty()),
                        )
                        .clicked();
                        if create_clicked || (enter && !self.new_column_name.trim().is_empty()) {
                            self.create_column();
                            self.show_add_column_modal = false;
                        }
                        if fg_button(
                            ui,
                            &theme,
                            &self.icons,
                            FgButton::new("Cancel")
                                .variant(FgButtonVariant::Ghost)
                                .size(FgButtonSize::Sm),
                        )
                        .clicked()
                        {
                            self.show_add_column_modal = false;
                            self.new_column_name.clear();
                        }
                    });
                });
            });
            if modal.outside_clicked {
                self.show_add_column_modal = false;
                self.new_column_name.clear();
            }
        }

        if self.show_add_user_modal {
            let modal = fg_modal(ctx, &theme, "add_user_modal", 420.0, |ui| {
                ui.label(
                    RichText::new("Add team member")
                        .size(18.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.add_space(20.0);
                ui.label(
                    RichText::new("Member Name")
                        .size(theme.font_size_sm)
                        .strong()
                        .color(theme.text_secondary),
                );
                let response = fg_input(
                    ui,
                    &theme,
                    FgInput::new(&mut self.new_user_name).placeholder("E.g. Alice"),
                );
                ui.memory_mut(|mem| mem.request_focus(response.id));
                let enter =
                    response.lost_focus() && ui.input(|input| input.key_pressed(egui::Key::Enter));
                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        let create_clicked = fg_button(
                            ui,
                            &theme,
                            &self.icons,
                            FgButton::new("Add member")
                                .variant(FgButtonVariant::Primary)
                                .size(FgButtonSize::Sm)
                                .disabled(self.new_user_name.trim().is_empty()),
                        )
                        .clicked();
                        if create_clicked || (enter && !self.new_user_name.trim().is_empty()) {
                            self.create_user();
                            self.show_add_user_modal = false;
                        }
                        if fg_button(
                            ui,
                            &theme,
                            &self.icons,
                            FgButton::new("Cancel")
                                .variant(FgButtonVariant::Ghost)
                                .size(FgButtonSize::Sm),
                        )
                        .clicked()
                        {
                            self.show_add_user_modal = false;
                            self.new_user_name.clear();
                        }
                    });
                });
            });
            if modal.outside_clicked {
                self.show_add_user_modal = false;
                self.new_user_name.clear();
            }
        }
    }
}

impl TodoShowcaseApp {
    fn sidebar(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        ui.label(
            RichText::new("Momentum")
                .size(18.0)
                .strong()
                .color(theme.text_primary),
        );
        ui.label(
            RichText::new("WORKSPACE")
                .size(10.0)
                .strong()
                .color(theme.text_muted),
        );
        ui.add_space(12.0);

        let _tasks = self.tasks();
        self.sidebar_link(ui, "Inbox");
        self.sidebar_link(ui, "Today");
        self.sidebar_link(ui, "Upcoming");

        ui.add_space(32.0);
        ui.label(
            RichText::new("TEAM")
                .size(10.0)
                .strong()
                .color(theme.text_muted),
        );
        ui.add_space(12.0);
        for user in self.users() {
            self.sidebar_link(ui, &user.name);
        }
        ui.add_space(20.0);

        if fg_button(
            ui,
            &theme,
            &self.icons,
            FgButton::new("+ Invite Member").variant(FgButtonVariant::Ghost),
        )
        .clicked()
        {
            self.show_add_user_modal = true;
        }
    }

    fn sidebar_link(&mut self, ui: &mut egui::Ui, label: &str) {
        let active = self.current_filter == label;
        let response = Frame::new()
            .fill(if active {
                self.theme.accent_subtle
            } else {
                Color32::TRANSPARENT
            })
            .corner_radius(CornerRadius::same(self.theme.radius_md as u8))
            .inner_margin(Margin::symmetric(10, 8))
            .show(ui, |ui| {
                ui.horizontal(|ui| {
                    let mut text = RichText::new(label).size(14.0).color(if active {
                        self.theme.text_primary
                    } else {
                        self.theme.text_secondary
                    });
                    if active {
                        text = text.strong();
                    }
                    ui.label(text);
                });
            })
            .response
            .interact(egui::Sense::click());

        if response.hovered() && !active {
            ui.painter().rect_filled(
                response.rect,
                CornerRadius::same(self.theme.radius_md as u8),
                self.theme.bg_raised,
            );
        }

        if response.clicked() {
            self.current_filter = label.to_string();
        }
    }

    fn workspace(&mut self, ui: &mut egui::Ui) {
        ui.vertical(|ui| {
            self.top_bar(ui);
            ui.add_space(24.0);
            self.main_panel(ui);
        });
    }

    fn top_bar(&mut self, ui: &mut egui::Ui) {
        let theme = self.theme.clone();
        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.label(
                    RichText::new("Inbox")
                        .size(24.0)
                        .strong()
                        .color(theme.text_primary),
                );
                ui.label(
                    RichText::new(format!("{} active tasks", self.tasks().len()))
                        .size(theme.font_size_sm)
                        .color(theme.text_secondary),
                );
            });
            ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                if fg_button(
                    ui,
                    &theme,
                    &self.icons,
                    FgButton::new("Add Task")
                        .variant(FgButtonVariant::Primary)
                        .size(FgButtonSize::Sm),
                )
                .clicked()
                {
                    self.show_add_task_modal = true;
                }
            });
        });
    }

    fn main_panel(&mut self, ui: &mut egui::Ui) {
        let _theme = self.theme.clone();
        ui.add_space(8.0);
        self.board(ui);
    }

    fn board(&mut self, ui: &mut egui::Ui) {
        let tasks = self.filtered_tasks();
        let mut drop_target = None;
        let pointer_released = ui.input(|input| input.pointer.any_released());
        let pointer_pos = ui.input(|input| input.pointer.interact_pos());
        let columns = self.columns();

        egui::ScrollArea::horizontal().show(ui, |ui| {
            ui.horizontal(|ui| {
                for (index, column) in columns.iter().enumerate() {
                    ui.vertical(|ui| {
                        ui.set_width(280.0);
                        let lane_tasks = tasks
                            .iter()
                            .filter(|t| t.column_id == column.id)
                            .collect::<Vec<_>>();

                        ui.add_space(8.0);

                        // Column Header
                        ui.horizontal(|ui| {
                            let label = RichText::new(&column.name)
                                .size(13.0)
                                .color(self.theme.text_secondary)
                                .strong();
                            ui.label(label);

                            ui.with_layout(
                                egui::Layout::right_to_left(egui::Align::Center),
                                |ui| {
                                    ui.label(
                                        RichText::new(lane_tasks.len().to_string())
                                            .size(11.0)
                                            .color(self.theme.text_muted.gamma_multiply(0.6)),
                                    );
                                },
                            );
                        });
                        ui.add_space(16.0);

                        let response = ui.allocate_response(
                            egui::vec2(ui.available_width(), ui.available_height().max(500.0)),
                            egui::Sense::hover(),
                        );
                        let rect = response.rect;
                        ui.painter().rect_filled(
                            rect.expand2(egui::vec2(8.0, 12.0)),
                            self.theme.radius_md,
                            self.theme.bg_surface,
                        );

                        if self.dragged_task.is_some()
                            && pointer_pos.is_some_and(|pos| rect.contains(pos))
                        {
                            drop_target = Some(column.id.clone());
                            ui.painter().rect_filled(
                                rect.expand2(egui::vec2(2.0, 2.0)),
                                self.theme.radius_md,
                                self.theme.accent_subtle,
                            );
                        }

                        let mut ui_child = ui.new_child(egui::UiBuilder::new().max_rect(rect));
                        for task in lane_tasks {
                            self.render_card(&mut ui_child, task);
                            ui_child.add_space(8.0);
                        }
                    });

                    if index < columns.len() - 1 {
                        ui.add_space(8.0);
                        let (rect, _) = ui.allocate_exact_size(
                            egui::vec2(1.0, ui.available_height().max(200.0)),
                            egui::Sense::hover(),
                        );
                        ui.painter()
                            .rect_filled(rect, 0.0, self.theme.border_subtle);
                        ui.add_space(8.0);
                    }
                }

                ui.add_space(8.0);
                ui.vertical(|ui| {
                    ui.set_width(280.0);
                    ui.add_space(8.0);
                    ui.horizontal(|ui| {
                        ui.label(
                            RichText::new("Add column")
                                .size(13.0)
                                .color(self.theme.text_muted)
                                .strong(),
                        );
                    });
                    ui.add_space(16.0);
                    let _response = Frame::new()
                        .fill(Color32::TRANSPARENT)
                        .stroke(Stroke::new(1.0, self.theme.border_subtle))
                        .corner_radius(CornerRadius::same(self.theme.radius_md as u8))
                        .inner_margin(Margin::same(12))
                        .show(ui, |ui| {
                            ui.set_width(ui.available_width());
                            ui.centered_and_justified(|ui| {
                                if fg_button(
                                    ui,
                                    &self.theme,
                                    &self.icons,
                                    FgButton::new("+ Add column").variant(FgButtonVariant::Ghost),
                                )
                                .clicked()
                                {
                                    self.show_add_column_modal = true;
                                }
                            });
                        })
                        .response;
                });
            });
        });

        if let (Some(dragged_id), Some(col_id)) = (self.dragged_task.clone(), drop_target) {
            if pointer_released {
                if let Some(mut task) = self.truth.live_task(&dragged_id) {
                    if task.column_id != col_id {
                        task.column_id = col_id;
                        self.write_field(
                            task.id.clone(),
                            "column",
                            "id",
                            forge_query::facade::DeclarativeWritebackValue::String(
                                task.column_id.clone(),
                            ),
                        );
                    }
                }
                self.dragged_task = None;
            }
        }

        if pointer_released {
            self.dragged_task = None;
        }

        if let (Some(dragged_id), Some(pos)) = (&self.dragged_task, pointer_pos) {
            if let Some(task) = tasks.iter().find(|task| task.id == *dragged_id) {
                self.render_drag_ghost(ui.ctx(), task, pos);
            }
        }
    }

    fn render_card(&mut self, ui: &mut egui::Ui, task: &Task) {
        let selected = self.selected_task == task.id;

        let mut frame = panel_frame(&self.theme, self.theme.bg_raised);

        frame = frame.stroke(Stroke::new(
            1.5,
            if selected {
                self.theme.accent_primary
            } else {
                self.theme.border_default
            },
        ));

        let response = frame
            .show(ui, |ui| {
                ui.set_min_height(72.0);
                ui.vertical(|ui| {
                    ui.label(
                        RichText::new(&task.title)
                            .color(self.theme.text_primary)
                            .size(15.0)
                            .line_height(Some(20.0))
                            .strong(),
                    );

                    ui.add_space(12.0);

                    ui.horizontal(|ui| {
                        let user_name = task
                            .assignee_id
                            .as_ref()
                            .and_then(|id| {
                                self.users()
                                    .iter()
                                    .find(|u| u.id == *id)
                                    .map(|u| u.name.clone())
                            })
                            .unwrap_or_else(|| "?".to_string());
                        avatar(ui, &self.theme, &user_name);

                        ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                            if !task.priority.is_empty() && task.priority != "P2" {
                                let p_color = match task.priority.as_str() {
                                    "P0" => self.theme.danger,
                                    "P1" => self.theme.warning,
                                    _ => self.theme.text_secondary,
                                };
                                chip(ui, &self.theme, &task.priority, p_color);
                            }

                            if let Some(due) = &task.due_date {
                                ui.label(
                                    RichText::new(due)
                                        .size(11.0)
                                        .color(self.theme.text_secondary),
                                );
                            }
                        });
                    });

                    if !task.tag.is_empty() {
                        ui.add_space(6.0);
                        ui.horizontal(|ui| {
                            ui.label(
                                RichText::new(&task.tag)
                                    .size(10.0)
                                    .color(self.theme.text_secondary)
                                    .italics(),
                            );
                        });
                    }
                });
            })
            .response;

        let response = response.interact(egui::Sense::click_and_drag());

        if response.hovered() {
            ui.ctx().set_cursor_icon(egui::CursorIcon::PointingHand);
            ui.painter().rect_stroke(
                response.rect,
                self.theme.radius_md,
                Stroke::new(1.0, self.theme.accent_primary.gamma_multiply(0.5)),
                egui::StrokeKind::Inside,
            );
        }

        if response.clicked() {
            self.selected_task = task.id.clone();
            self.show_edit_task_modal = true;
        }
        if response.drag_started() {
            self.dragged_task = Some(task.id.clone());
        }
    }

    fn render_drag_ghost(&self, ctx: &egui::Context, task: &Task, pointer_pos: egui::Pos2) {
        egui::Area::new(egui::Id::new(format!("drag_ghost_{}", task.id)))
            .order(egui::Order::Tooltip)
            .fixed_pos(pointer_pos + egui::vec2(14.0, 14.0))
            .interactable(false)
            .show(ctx, |ui| {
                ui.set_width(260.0);
                panel_frame(&self.theme, self.theme.bg_raised)
                    .stroke(Stroke::new(1.5, self.theme.accent_primary))
                    .shadow(egui::Shadow {
                        offset: [0, 12],
                        blur: 28,
                        spread: 2,
                        color: Color32::from_black_alpha(110),
                    })
                    .show(ui, |ui| {
                        ui.label(
                            RichText::new(&task.title)
                                .color(self.theme.text_primary)
                                .size(15.0)
                                .strong(),
                        );
                    });
            });
    }

    fn task_details_modal(&mut self, ctx: &egui::Context, theme: &ForgeTheme) {
        if self.show_edit_task_modal {
            let Some(task) = self.selected() else { return };
            let modal = fg_modal(ctx, theme, "edit_task", 480.0, |ui| {
                ui.horizontal(|ui| {
                    ui.label(RichText::new("Edit Task").size(18.0).strong());
                    ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                        if fg_icon_button(
                            ui,
                            theme,
                            &self.icons,
                            forge_ui_components::icon_button::FgIconButton::new(FgIcon::Trash2)
                                .tint(theme.danger),
                        )
                        .clicked()
                        {
                            self.delete_task();
                        }
                    });
                });
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(16.0);

                self.render_task_form(ui, &task, false);

                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if fg_button(
                        ui,
                        theme,
                        &self.icons,
                        FgButton::new("Close").variant(FgButtonVariant::Secondary),
                    )
                    .clicked()
                    {
                        self.show_edit_task_modal = false;
                    }
                });
            });
            if modal.outside_clicked {
                self.show_edit_task_modal = false;
            }
        }

        if self.show_add_task_modal {
            let task = self.new_task_draft.clone();
            let modal = fg_modal(ctx, theme, "add_task", 480.0, |ui| {
                ui.label(RichText::new("Create Task").size(18.0).strong());
                ui.add_space(8.0);
                ui.separator();
                ui.add_space(16.0);

                self.render_task_form(ui, &task, true);

                ui.add_space(24.0);
                ui.horizontal(|ui| {
                    if fg_button(
                        ui,
                        theme,
                        &self.icons,
                        FgButton::new("Create Task").variant(FgButtonVariant::Primary),
                    )
                    .clicked()
                    {
                        self.create_task();
                    }
                    if fg_button(
                        ui,
                        theme,
                        &self.icons,
                        FgButton::new("Cancel").variant(FgButtonVariant::Ghost),
                    )
                    .clicked()
                    {
                        self.show_add_task_modal = false;
                    }
                });
            });
            if modal.outside_clicked {
                self.show_add_task_modal = false;
            }
        }
    }

    fn render_task_form(&mut self, ui: &mut egui::Ui, task: &Task, is_create: bool) {
        let theme = self.theme.clone();

        // Title
        field_group(ui, &theme, "Title");
        let mut title = if is_create {
            self.new_task_draft.title.clone()
        } else {
            task.title.clone()
        };
        if fg_input(
            ui,
            &theme,
            FgInput::new(&mut title).placeholder("Enter task title..."),
        )
        .changed()
        {
            if is_create {
                self.new_task_draft.title = title;
            } else {
                self.write_field(
                    task.id.clone(),
                    "title",
                    "value",
                    forge_query::facade::DeclarativeWritebackValue::String(title),
                );
            }
        }

        ui.add_space(18.0);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(200.0);
                field_group(ui, &theme, "Column");
                let columns = self.columns();
                let items: Vec<DropdownItem> = columns
                    .iter()
                    .map(|c| DropdownItem::new(&c.id, &c.name))
                    .collect();
                let current_id = if is_create {
                    Some(self.new_task_draft.column_id.as_str())
                } else {
                    Some(task.column_id.as_str())
                };

                if let Some(new_id) = fg_dropdown(
                    ui,
                    &theme,
                    "col_drop",
                    forge_ui_components::dropdown::FgDropdown::new(
                        &items,
                        current_id,
                        &mut self.column_dropdown,
                    ),
                ) {
                    if is_create {
                        self.new_task_draft.column_id = new_id;
                    } else {
                        self.write_field(
                            task.id.clone(),
                            "column",
                            "id",
                            forge_query::facade::DeclarativeWritebackValue::String(new_id),
                        );
                    }
                }
            });

            ui.add_space(20.0);

            ui.vertical(|ui| {
                field_group(ui, &theme, "Owner");
                let users = self.users();
                let items: Vec<DropdownItem> = users
                    .iter()
                    .map(|u| DropdownItem::new(&u.id, &u.name))
                    .collect();
                let current_id = if is_create {
                    self.new_task_draft.assignee_id.as_deref()
                } else {
                    task.assignee_id.as_deref()
                };

                if let Some(new_id) = fg_dropdown(
                    ui,
                    &theme,
                    "user_drop",
                    forge_ui_components::dropdown::FgDropdown::new(
                        &items,
                        current_id,
                        &mut self.assignee_dropdown,
                    )
                    .placeholder("Select user..."),
                ) {
                    if is_create {
                        self.new_task_draft.assignee_id = Some(new_id);
                    } else {
                        self.write_field(
                            task.id.clone(),
                            "assignee",
                            "id",
                            forge_query::facade::DeclarativeWritebackValue::String(new_id),
                        );
                    }
                }
            });
        });

        ui.add_space(18.0);

        ui.horizontal(|ui| {
            ui.vertical(|ui| {
                ui.set_width(200.0);
                field_group(ui, &theme, "Priority");
                let items = vec![
                    DropdownItem::new("P0", "P0 - Emergency"),
                    DropdownItem::new("P1", "P1 - High"),
                    DropdownItem::new("P2", "P2 - Medium"),
                    DropdownItem::new("P3", "P3 - Low"),
                ];
                let current_id = if is_create {
                    Some(self.new_task_draft.priority.as_str())
                } else {
                    Some(task.priority.as_str())
                };

                if let Some(new_id) = fg_dropdown(
                    ui,
                    &theme,
                    "pri_drop",
                    forge_ui_components::dropdown::FgDropdown::new(
                        &items,
                        current_id,
                        &mut self.priority_dropdown,
                    ),
                ) {
                    if is_create {
                        self.new_task_draft.priority = new_id;
                    } else {
                        self.write_field(
                            task.id.clone(),
                            "priority",
                            "level",
                            forge_query::facade::DeclarativeWritebackValue::String(new_id),
                        );
                    }
                }
            });

            ui.add_space(20.0);

            ui.vertical(|ui| {
                field_group(ui, &theme, "Due Date");
                let mut due = if is_create {
                    self.new_task_draft.due_date.clone().unwrap_or_default()
                } else {
                    task.due_date.clone().unwrap_or_default()
                };
                if fg_input(ui, &theme, FgInput::new(&mut due).placeholder("YYYY-MM-DD")).changed()
                {
                    if is_create {
                        self.new_task_draft.due_date =
                            if due.is_empty() { None } else { Some(due) };
                    } else {
                        self.write_field(
                            task.id.clone(),
                            "due",
                            "date",
                            forge_query::facade::DeclarativeWritebackValue::String(due),
                        );
                    }
                }
            });
        });

        ui.add_space(18.0);
        field_group(ui, &theme, "Tags");
        let mut tag = if is_create {
            self.new_task_draft.tag.clone()
        } else {
            task.tag.clone()
        };
        if fg_input(
            ui,
            &theme,
            FgInput::new(&mut tag).placeholder("E.g. Work, Personal"),
        )
        .changed()
        {
            if is_create {
                self.new_task_draft.tag = tag;
            } else {
                self.write_field(
                    task.id.clone(),
                    "tag",
                    "name",
                    forge_query::facade::DeclarativeWritebackValue::String(tag),
                );
            }
        }
    }
}

fn panel_frame(theme: &ForgeTheme, fill: Color32) -> Frame {
    Frame::new()
        .fill(fill)
        .stroke(Stroke::new(1.0, theme.border_subtle))
        .corner_radius(CornerRadius::same(theme.radius_md as u8))
        .inner_margin(Margin::same(12))
}

fn chip(ui: &mut egui::Ui, theme: &ForgeTheme, label: &str, color: Color32) {
    fg_chip(ui, theme, FgChip::new(label).dot(color));
}

fn avatar(ui: &mut egui::Ui, theme: &ForgeTheme, name: &str) {
    let initial = name.chars().next().unwrap_or('?').to_string();
    let (rect, _) = ui.allocate_exact_size(egui::vec2(24.0, 24.0), egui::Sense::hover());
    ui.painter()
        .circle_filled(rect.center(), 12.0, theme.accent_subtle);
    ui.painter().text(
        rect.center(),
        egui::Align2::CENTER_CENTER,
        initial,
        egui::FontId::proportional(theme.font_size_sm),
        theme.text_primary,
    );
    ui.label(
        RichText::new(name)
            .size(theme.font_size_sm)
            .color(theme.text_secondary),
    );
}

fn field_group(ui: &mut egui::Ui, theme: &ForgeTheme, label: &str) {
    ui.label(
        RichText::new(label)
            .size(theme.font_size_sm)
            .strong()
            .color(theme.text_secondary),
    );
}
