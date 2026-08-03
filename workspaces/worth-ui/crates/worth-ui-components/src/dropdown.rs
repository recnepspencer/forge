//! FgDropdown — searchable dropdown selector.
//!
//! Always includes a search filter. Designed to be standalone so it can plug into
//! forms, toolbars, or nested list selectors.

use egui::{Color32, CornerRadius, Frame, Pos2, Sense, Stroke, Ui, Vec2};
use worth_ui_theme::WorthTheme;

/// A single item in the dropdown.
#[derive(Debug, Clone)]
pub struct DropdownItem {
    pub id: String,
    pub label: String,
    /// Optional group/category for nested lists.
    pub group: Option<String>,
}

impl DropdownItem {
    pub fn new(id: impl Into<String>, label: impl Into<String>) -> Self {
        Self {
            id: id.into(),
            label: label.into(),
            group: None,
        }
    }
    pub fn group(mut self, g: impl Into<String>) -> Self {
        self.group = Some(g.into());
        self
    }
}

/// Persistent state for a dropdown instance. Store this on your page/state struct.
#[derive(Debug, Clone, Default)]
pub struct DropdownState {
    pub open: bool,
    pub search: String,
}

/// Props for FgDropdown.
pub struct FgDropdown<'a> {
    pub label: Option<&'a str>,
    pub placeholder: &'a str,
    pub items: &'a [DropdownItem],
    pub selected_id: Option<&'a str>,
    pub dropdown_state: &'a mut DropdownState,
}

impl<'a> FgDropdown<'a> {
    pub fn new(
        items: &'a [DropdownItem],
        selected_id: Option<&'a str>,
        dropdown_state: &'a mut DropdownState,
    ) -> Self {
        Self {
            label: None,
            placeholder: "Select…",
            items,
            selected_id,
            dropdown_state,
        }
    }
    pub fn label(mut self, l: &'a str) -> Self {
        self.label = Some(l);
        self
    }
    pub fn placeholder(mut self, p: &'a str) -> Self {
        self.placeholder = p;
        self
    }
}

/// Render a searchable dropdown. Returns the newly selected item ID if changed.
pub fn fg_dropdown(
    ui: &mut Ui,
    theme: &WorthTheme,
    id_source: &str,
    props: FgDropdown<'_>,
) -> Option<String> {
    let mut newly_selected: Option<String> = None;

    if let Some(label) = props.label {
        ui.label(
            egui::RichText::new(label)
                .color(theme.text_secondary)
                .size(theme.font_size_sm),
        );
        ui.add_space(4.0);
    }

    // ── Trigger button ──────────────────────────────────────────
    let display_text = props
        .selected_id
        .and_then(|sid| props.items.iter().find(|i| i.id == sid))
        .map(|i| i.label.as_str())
        .unwrap_or(props.placeholder);

    let trigger_resp = Frame::new()
        .fill(theme.bg_base)
        .stroke(Stroke::new(
            1.0,
            if props.dropdown_state.open {
                theme.border_focus
            } else {
                theme.border_default
            },
        ))
        .corner_radius(CornerRadius::same(theme.radius_sm as u8))
        .inner_margin(egui::Margin {
            left: 10,
            right: 10,
            top: 6,
            bottom: 6,
        })
        .show(ui, |ui| {
            ui.horizontal(|ui| {
                let text_color = if props.selected_id.is_some() {
                    theme.text_primary
                } else {
                    theme.text_muted
                };
                ui.label(
                    egui::RichText::new(display_text)
                        .color(text_color)
                        .size(theme.font_size_sm),
                );
                ui.with_layout(egui::Layout::right_to_left(egui::Align::Center), |ui| {
                    let arrow = if props.dropdown_state.open {
                        "▲"
                    } else {
                        "▼"
                    };
                    ui.label(
                        egui::RichText::new(arrow)
                            .color(theme.text_muted)
                            .size(theme.font_size_xs),
                    );
                });
            });
        })
        .response
        .interact(Sense::click());

    if trigger_resp.clicked() {
        props.dropdown_state.open = !props.dropdown_state.open;
        if props.dropdown_state.open {
            props.dropdown_state.search.clear();
        }
    }

    // ── Popup list ──────────────────────────────────────────────
    if props.dropdown_state.open {
        let popup_id = egui::Id::new(format!("dropdown_popup_{id_source}"));
        let trigger_rect = trigger_resp.rect;

        egui::Area::new(popup_id)
            .fixed_pos(Pos2::new(trigger_rect.min.x, trigger_rect.max.y + 4.0))
            .order(egui::Order::Foreground)
            .show(ui.ctx(), |ui| {
                Frame::new()
                    .fill(theme.bg_raised)
                    .stroke(Stroke::new(1.0, theme.border_default))
                    .corner_radius(CornerRadius::same(theme.radius_md as u8))
                    .inner_margin(egui::Margin::same(8))
                    .shadow(egui::Shadow {
                        offset: [0, 4],
                        blur: 16,
                        spread: 0,
                        color: Color32::from_black_alpha(80),
                    })
                    .show(ui, |ui| {
                        ui.set_width(trigger_rect.width().max(200.0));

                        // Search field
                        ui.add(
                            egui::TextEdit::singleline(&mut props.dropdown_state.search)
                                .hint_text("🔍 Search…")
                                .desired_width(f32::INFINITY)
                                .font(egui::FontId::proportional(theme.font_size_sm))
                                .frame(egui::Frame::NONE),
                        )
                        .request_focus();
                        ui.add_space(4.0);
                        ui.separator();
                        ui.add_space(4.0);

                        // Filtered items
                        let query = props.dropdown_state.search.to_lowercase();
                        let filtered: Vec<_> = props
                            .items
                            .iter()
                            .filter(|i| query.is_empty() || i.label.to_lowercase().contains(&query))
                            .collect();

                        egui::ScrollArea::vertical()
                            .max_height(200.0)
                            .show(ui, |ui| {
                                let mut last_group: Option<&str> = None;
                                for item in &filtered {
                                    // Group header
                                    if let Some(g) = &item.group {
                                        if last_group != Some(g.as_str()) {
                                            if last_group.is_some() {
                                                ui.add_space(4.0);
                                            }
                                            ui.label(
                                                egui::RichText::new(g.as_str())
                                                    .color(theme.text_muted)
                                                    .size(theme.font_size_xs)
                                                    .strong(),
                                            );
                                            ui.add_space(2.0);
                                            last_group = Some(g.as_str());
                                        }
                                    }

                                    let is_selected = props.selected_id == Some(item.id.as_str());
                                    let row_h = 28.0;
                                    let (row_rect, row_resp) = ui.allocate_exact_size(
                                        Vec2::new(ui.available_width(), row_h),
                                        egui::Sense::click(),
                                    );

                                    if ui.is_rect_visible(row_rect) {
                                        if is_selected {
                                            ui.painter().rect_filled(
                                                row_rect,
                                                CornerRadius::same(theme.radius_sm as u8),
                                                theme.accent_subtle,
                                            );
                                        } else if row_resp.hovered() {
                                            ui.painter().rect_filled(
                                                row_rect,
                                                CornerRadius::same(theme.radius_sm as u8),
                                                Color32::from_white_alpha(8),
                                            );
                                        }
                                        let label_color = if is_selected {
                                            theme.accent_primary
                                        } else {
                                            theme.text_primary
                                        };
                                        let g = ui.fonts_mut(|f| {
                                            f.layout_no_wrap(
                                                item.label.clone(),
                                                egui::FontId::proportional(theme.font_size_sm),
                                                label_color,
                                            )
                                        });
                                        ui.painter().galley(
                                            Pos2::new(
                                                row_rect.min.x + 8.0,
                                                row_rect.center().y - g.size().y / 2.0,
                                            ),
                                            g,
                                            label_color,
                                        );
                                    }

                                    if row_resp.clicked() {
                                        newly_selected = Some(item.id.clone());
                                        props.dropdown_state.open = false;
                                    }
                                }

                                if filtered.is_empty() {
                                    ui.label(
                                        egui::RichText::new("No results")
                                            .color(theme.text_muted)
                                            .size(theme.font_size_sm),
                                    );
                                }
                            });
                    });
            });
    }

    newly_selected
}
