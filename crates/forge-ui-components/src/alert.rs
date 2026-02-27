//! FgAlert — modal alert dialog box.
//!
//! Renders as a centered dialog with a title, message, and action buttons.
//! Typically used for destructive actions (e.g. Delete confirmation) or important notifications.

use egui::{Color32, CornerRadius, Frame, Stroke};
use forge_ui_theme::ForgeTheme;

use crate::{fg_button, FgButton, FgButtonVariant};

/// Describes a button action on the alert.
pub struct AlertAction<'a> {
    pub label: &'a str,
    pub variant: FgButtonVariant,
}

impl<'a> AlertAction<'a> {
    pub fn new(label: &'a str) -> Self {
        Self {
            label,
            variant: FgButtonVariant::Secondary,
        }
    }
    pub fn primary(label: &'a str) -> Self {
        Self {
            label,
            variant: FgButtonVariant::Primary,
        }
    }
    pub fn danger(label: &'a str) -> Self {
        Self {
            label,
            variant: FgButtonVariant::Danger,
        }
    }
    pub fn cancel(label: &'a str) -> Self {
        Self {
            label,
            variant: FgButtonVariant::Ghost,
        }
    }
}

pub struct FgAlert<'a> {
    pub title: &'a str,
    pub message: &'a str,
    pub actions: Vec<AlertAction<'a>>,
}

impl<'a> FgAlert<'a> {
    pub fn new(title: &'a str, message: &'a str) -> Self {
        Self {
            title,
            message,
            actions: Vec::new(),
        }
    }

    pub fn with_action(mut self, action: AlertAction<'a>) -> Self {
        self.actions.push(action);
        self
    }
}

/// Renders a modal alert dialog. Returns the index of the clicked action button, if any.
pub fn fg_alert(
    ctx: &egui::Context,
    theme: &ForgeTheme,
    icons: &crate::IconStore,
    id: &str,
    props: FgAlert<'_>,
) -> Option<usize> {
    let mut clicked_idx = None;

    // Dark scrim
    let _scrim_resp = egui::Area::new(egui::Id::new(format!("{id}_scrim")))
        .fixed_pos(egui::Pos2::ZERO)
        .order(egui::Order::Foreground)
        .interactable(true)
        .show(ctx, |ui| {
            let screen = ctx.screen_rect();
            let (rect, resp) = ui.allocate_exact_size(screen.size(), egui::Sense::click());
            ui.painter()
                .rect_filled(rect, 0.0, Color32::from_black_alpha(160));
            resp
        });

    // Dialog card
    egui::Area::new(egui::Id::new(id))
        .anchor(egui::Align2::CENTER_CENTER, egui::vec2(0.0, 0.0))
        .order(egui::Order::Foreground)
        .show(ctx, |ui| {
            Frame::new()
                .fill(theme.bg_surface)
                .stroke(Stroke::new(1.0, theme.border_subtle))
                .corner_radius(CornerRadius::same(theme.radius_lg as u8))
                .inner_margin(egui::Margin::same(24))
                .shadow(egui::Shadow {
                    offset: [0, 16],
                    blur: 32,
                    spread: 4,
                    color: Color32::from_black_alpha(120),
                })
                .show(ui, |ui| {
                    ui.set_width(320.0);

                    // Center title and message
                    ui.vertical_centered(|ui| {
                        ui.label(
                            egui::RichText::new(props.title)
                                .color(theme.text_primary)
                                .size(theme.font_size_lg)
                                .strong(),
                        );
                        ui.add_space(8.0);
                        ui.label(
                            egui::RichText::new(props.message)
                                .color(theme.text_secondary)
                                .size(theme.font_size_md),
                        );
                    });

                    ui.add_space(24.0);

                    // Actions
                    ui.horizontal(|ui| {
                        let btn_width = (ui.available_width()
                            - ((props.actions.len().saturating_sub(1) as f32) * 8.0))
                            / (props.actions.len().max(1) as f32);

                        for (i, action) in props.actions.iter().enumerate() {
                            let resp = crate::fg_button(
                                ui,
                                theme,
                                icons,
                                FgButton::new(action.label)
                                    .variant(action.variant)
                                    .width(btn_width),
                            );
                            if resp.clicked() {
                                clicked_idx = Some(i);
                            }
                        }
                    });
                })
        });

    clicked_idx
}
