use eframe::egui::{
    self, Color32, Context, FontId, Frame, Margin, RichText, Stroke, TextStyle, TopBottomPanel,
    Visuals,
};
use worth_ui::facade::{
    WorthUiHeaderAppearanceFrameReceipt, WorthUiHeaderFrameReceipt, WorthUiHeaderMenuCommand,
    WorthUiHeaderMenuGroup, WorthUiHeaderThemeFrameReceipt, WorthUiPaddingValue,
};

use super::dropdown_interaction::{
    dropdown_control_kind, selection_action_for_response, ValidationDropdownControlKind,
};
use super::ValidationHeaderSelectionAction;

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationHeaderAppliedStyleReceipt {
    panel_fill: Color32,
    menu_fill: Color32,
    menu_hover_fill: Color32,
    menu_active_fill: Color32,
    text_color: Color32,
    border_color: Color32,
    border_width_points: f32,
    menu_min_width_points: f32,
    font_size_points: f32,
    control_spacing_points: f32,
    row_padding_horizontal_points: f32,
    row_padding_vertical_points: f32,
    container_margin: Margin,
    menu_margin: Margin,
    shadow: eframe::epaint::Shadow,
}

pub fn render_header_only(
    ctx: &Context,
    receipt: &WorthUiHeaderFrameReceipt,
    theme: &WorthUiHeaderThemeFrameReceipt,
    appearance: &WorthUiHeaderAppearanceFrameReceipt,
) -> Vec<ValidationHeaderSelectionAction> {
    let applied = applied_header_style_receipt(theme, appearance);
    apply_validation_theme(ctx, &applied);
    let mut actions = Vec::new();
    TopBottomPanel::top("worth_ui_validation_header")
        .frame(
            Frame::NONE
                .fill(applied.panel_fill())
                .inner_margin(applied.container_margin())
                .stroke(Stroke::new(
                    applied.border_width_points(),
                    applied.border_color(),
                ))
                .shadow(applied.shadow()),
        )
        .show(ctx, |ui| {
            apply_header_style(ui, &applied);
            ui.horizontal_centered(|ui| {
                render_menus(ui, receipt, &applied, &mut actions);
            });
        });
    actions
}

pub fn applied_header_style_receipt(
    theme: &WorthUiHeaderThemeFrameReceipt,
    appearance: &WorthUiHeaderAppearanceFrameReceipt,
) -> ValidationHeaderAppliedStyleReceipt {
    ValidationHeaderAppliedStyleReceipt {
        panel_fill: theme_color(theme.panel_fill()),
        menu_fill: theme_color(theme.menu_fill()),
        menu_hover_fill: theme_color(theme.menu_hover_fill()),
        menu_active_fill: theme_color(theme.menu_active_fill()),
        text_color: theme_color(theme.text()),
        border_color: theme_color(theme.border()),
        border_width_points: appearance.border_width().points(),
        menu_min_width_points: appearance.menu_min_width().points(),
        font_size_points: appearance.font_size().points(),
        control_spacing_points: appearance.control_spacing().points(),
        row_padding_horizontal_points: appearance.row_padding().horizontal_points(),
        row_padding_vertical_points: appearance.row_padding().vertical_points(),
        container_margin: margin_from_padding(appearance.container_padding()),
        menu_margin: margin_from_padding(appearance.row_padding()),
        shadow: theme_shadow(appearance),
    }
}

fn render_menus(
    ui: &mut egui::Ui,
    receipt: &WorthUiHeaderFrameReceipt,
    applied: &ValidationHeaderAppliedStyleReceipt,
    actions: &mut Vec<ValidationHeaderSelectionAction>,
) {
    egui::menu::bar(ui, |ui| {
        for menu in receipt.groups() {
            ui.menu_button(menu.title(), |ui| {
                ui.set_min_width(applied.menu_min_width_points());
                for command in menu.commands() {
                    if let Some(action) = render_menu_command(ui, menu, command, applied) {
                        actions.push(action);
                    }
                }
            });
        }
    });
}

fn render_menu_command(
    ui: &mut egui::Ui,
    menu: &WorthUiHeaderMenuGroup,
    command: &WorthUiHeaderMenuCommand,
    applied: &ValidationHeaderAppliedStyleReceipt,
) -> Option<ValidationHeaderSelectionAction> {
    let text = menu_command_text(command);
    let selected = menu.selection_state().contains(command.command_id());
    match dropdown_control_kind(menu) {
        ValidationDropdownControlKind::MultiSelectCheckbox => {
            let mut checked = selected;
            let mut triggered = false;
            ui.horizontal(|ui| {
                let response = ui
                    .checkbox(&mut checked, "")
                    .on_hover_text(command.command_id());
                triggered = response.changed();
                render_command_icon(ui, command, applied);
                let label_response =
                    ui.selectable_label(selected, RichText::new(text).color(applied.text_color()));
                if label_response.clicked() {
                    triggered = true;
                }
            });
            selection_action_for_response(menu, command, triggered)
        }
        ValidationDropdownControlKind::SingleSelectButton => {
            let mut clicked = false;
            ui.horizontal(|ui| {
                render_command_icon(ui, command, applied);
                let label = if selected {
                    format!("[x] {text}")
                } else {
                    text.clone()
                };
                let response = ui
                    .button(RichText::new(label).color(applied.text_color()))
                    .on_hover_text(command.command_id());
                clicked = response.clicked();
            });
            selection_action_for_response(menu, command, clicked)
        }
    }
}

fn render_command_icon(
    ui: &mut egui::Ui,
    command: &WorthUiHeaderMenuCommand,
    applied: &ValidationHeaderAppliedStyleReceipt,
) {
    if let Some(icon) = command.icon_id().and_then(header_icon_for_id) {
        ui.label(
            RichText::new(icon)
                .size(applied.font_size_points())
                .color(applied.text_color()),
        );
    }
}

fn menu_command_text(command: &WorthUiHeaderMenuCommand) -> String {
    let mut text = command.label().to_owned();
    if let Some(shortcut) = command.shortcut() {
        text.push_str("    ");
        text.push_str(shortcut);
    }
    text
}

fn header_icon_for_id(icon_id: &str) -> Option<&'static str> {
    match icon_id {
        "worth.icon.header.file.new" => Some("+"),
        "worth.icon.header.file.open" => Some("open"),
        "worth.icon.header.file.save" => Some("ok"),
        "worth.icon.header.file.exit" => Some("x"),
        "worth.icon.header.edit.undo" => Some("undo"),
        "worth.icon.header.edit.redo" => Some("redo"),
        "worth.icon.header.edit.cut" => Some("cut"),
        "worth.icon.header.edit.copy" => Some("copy"),
        "worth.icon.header.edit.paste" => Some("paste"),
        "worth.icon.header.terminal.new" => Some("term"),
        "worth.icon.header.terminal.split" => Some("split"),
        "worth.icon.header.terminal.clear" => Some("clear"),
        "worth.icon.header.help.palette" => Some("find"),
        "worth.icon.header.help.docs" => Some("doc"),
        "worth.icon.header.help.about" => Some("info"),
        _ => None,
    }
}

fn apply_validation_theme(ctx: &Context, applied: &ValidationHeaderAppliedStyleReceipt) {
    let mut visuals = Visuals::dark();
    visuals.panel_fill = applied.panel_fill();
    visuals.window_fill = applied.menu_fill();
    visuals.extreme_bg_color = applied.panel_fill();
    visuals.faint_bg_color = applied.menu_fill();
    visuals.widgets.inactive.bg_fill = applied.menu_fill();
    visuals.widgets.inactive.weak_bg_fill = applied.menu_fill();
    visuals.widgets.hovered.bg_fill = applied.menu_hover_fill();
    visuals.widgets.active.bg_fill = applied.menu_active_fill();
    visuals.widgets.open.bg_fill = applied.menu_fill();
    visuals.window_shadow = applied.shadow();
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, applied.text_color());
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, applied.text_color());
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.window_stroke = Stroke::new(applied.border_width_points(), applied.border_color());
    ctx.set_visuals(visuals);
}

fn theme_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    let red = parse_hex_pair(&hex[0..2]);
    let green = parse_hex_pair(&hex[2..4]);
    let blue = parse_hex_pair(&hex[4..6]);
    let alpha = if hex.len() == 8 {
        parse_hex_pair(&hex[6..8])
    } else {
        u8::MAX
    };
    Color32::from_rgba_unmultiplied(red, green, blue, alpha)
}

fn parse_hex_pair(pair: &str) -> u8 {
    u8::from_str_radix(pair, 16).expect("Worth UI theme colors are validated hex tokens")
}

fn apply_header_style(ui: &mut egui::Ui, applied: &ValidationHeaderAppliedStyleReceipt) {
    let style = ui.style_mut();
    style.spacing.button_padding = egui::vec2(
        applied.row_padding_horizontal_points(),
        applied.row_padding_vertical_points(),
    );
    style.spacing.item_spacing.x = applied.control_spacing_points();
    style.spacing.menu_margin = applied.menu_margin();
    style.text_styles.insert(
        TextStyle::Button,
        FontId::proportional(applied.font_size_points()),
    );
}

fn theme_shadow(appearance: &WorthUiHeaderAppearanceFrameReceipt) -> eframe::epaint::Shadow {
    let shadow = appearance.panel_shadow();
    eframe::epaint::Shadow {
        offset: [shadow.offset_x_points(), shadow.offset_y_points()],
        blur: shadow.blur_points(),
        spread: shadow.spread_points(),
        color: theme_color(shadow.color().as_str()),
    }
}

fn margin_from_padding(padding: &WorthUiPaddingValue) -> Margin {
    Margin {
        left: rounded_margin(padding.left().points()),
        right: rounded_margin(padding.right().points()),
        top: rounded_margin(padding.top().points()),
        bottom: rounded_margin(padding.bottom().points()),
    }
}

impl ValidationHeaderAppliedStyleReceipt {
    pub fn panel_fill(&self) -> Color32 {
        self.panel_fill
    }

    pub fn menu_fill(&self) -> Color32 {
        self.menu_fill
    }

    pub fn menu_hover_fill(&self) -> Color32 {
        self.menu_hover_fill
    }

    pub fn menu_active_fill(&self) -> Color32 {
        self.menu_active_fill
    }

    pub fn text_color(&self) -> Color32 {
        self.text_color
    }

    pub fn border_color(&self) -> Color32 {
        self.border_color
    }

    pub fn border_width_points(&self) -> f32 {
        self.border_width_points
    }

    pub fn menu_min_width_points(&self) -> f32 {
        self.menu_min_width_points
    }

    pub fn font_size_points(&self) -> f32 {
        self.font_size_points
    }

    pub fn control_spacing_points(&self) -> f32 {
        self.control_spacing_points
    }

    pub fn row_padding_horizontal_points(&self) -> f32 {
        self.row_padding_horizontal_points
    }

    pub fn row_padding_vertical_points(&self) -> f32 {
        self.row_padding_vertical_points
    }

    pub fn container_margin(&self) -> Margin {
        self.container_margin
    }

    pub fn menu_margin(&self) -> Margin {
        self.menu_margin
    }

    pub fn shadow(&self) -> eframe::epaint::Shadow {
        self.shadow
    }
}

fn rounded_margin(points: f32) -> i8 {
    points.round().clamp(i8::MIN as f32, i8::MAX as f32) as i8
}
