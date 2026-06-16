use eframe::egui::{self, Color32, Context, Frame, RichText, Stroke, TopBottomPanel, Visuals};
use worth_ui::facade::{
    CommandProjectionSelectionMode, WorthUiHeaderFrameReceipt, WorthUiHeaderMenuCommand,
    WorthUiHeaderThemeFrameReceipt,
};

pub fn render_header_only(
    ctx: &Context,
    receipt: &WorthUiHeaderFrameReceipt,
    theme: &WorthUiHeaderThemeFrameReceipt,
) {
    apply_validation_theme(ctx, theme);
    let panel_fill = theme_color(theme.panel_fill());
    TopBottomPanel::top("worth_ui_validation_header")
        .exact_height(34.0)
        .frame(Frame::NONE.fill(panel_fill))
        .show(ctx, |ui| {
            ui.horizontal_centered(|ui| {
                ui.add_space(8.0);
                render_menus(ui, receipt, theme);
            });
        });
}

fn render_menus(
    ui: &mut egui::Ui,
    receipt: &WorthUiHeaderFrameReceipt,
    theme: &WorthUiHeaderThemeFrameReceipt,
) {
    egui::menu::bar(ui, |ui| {
        for menu in receipt.groups() {
            ui.menu_button(menu.title(), |ui| {
                ui.set_min_width(220.0);
                for command in menu.commands() {
                    render_menu_command(ui, command, menu.selection_mode(), theme);
                }
            });
        }
    });
}

fn render_menu_command(
    ui: &mut egui::Ui,
    command: &WorthUiHeaderMenuCommand,
    selection_mode: CommandProjectionSelectionMode,
    theme: &WorthUiHeaderThemeFrameReceipt,
) {
    let text = menu_command_text(command);
    match selection_mode {
        CommandProjectionSelectionMode::SingleSelect => {
            ui.button(RichText::new(text).color(theme_color(theme.text())))
                .on_hover_text(command.command_id());
        }
        CommandProjectionSelectionMode::MultiSelect => {
            let mut selected = false;
            ui.checkbox(
                &mut selected,
                RichText::new(text).color(theme_color(theme.text())),
            )
            .on_hover_text(command.command_id());
        }
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

fn apply_validation_theme(ctx: &Context, theme: &WorthUiHeaderThemeFrameReceipt) {
    let mut visuals = Visuals::dark();
    let panel = theme_color(theme.panel_fill());
    let menu = theme_color(theme.menu_fill());
    let hover = theme_color(theme.menu_hover_fill());
    let active = theme_color(theme.menu_active_fill());
    let text = theme_color(theme.text());
    let border = theme_color(theme.border());

    visuals.panel_fill = panel;
    visuals.window_fill = menu;
    visuals.extreme_bg_color = panel;
    visuals.faint_bg_color = menu;
    visuals.widgets.inactive.bg_fill = menu;
    visuals.widgets.inactive.weak_bg_fill = menu;
    visuals.widgets.hovered.bg_fill = hover;
    visuals.widgets.active.bg_fill = active;
    visuals.widgets.open.bg_fill = menu;
    visuals.widgets.noninteractive.fg_stroke = Stroke::new(1.0, text);
    visuals.widgets.inactive.fg_stroke = Stroke::new(1.0, text);
    visuals.widgets.hovered.fg_stroke = Stroke::new(1.0, Color32::WHITE);
    visuals.window_stroke = Stroke::new(1.0, border);
    ctx.set_visuals(visuals);
}

fn theme_color(hex: &str) -> Color32 {
    let hex = hex.trim_start_matches('#');
    let red = parse_hex_pair(&hex[0..2]);
    let green = parse_hex_pair(&hex[2..4]);
    let blue = parse_hex_pair(&hex[4..6]);
    Color32::from_rgb(red, green, blue)
}

fn parse_hex_pair(pair: &str) -> u8 {
    u8::from_str_radix(pair, 16).expect("Worth UI theme colors are validated hex RGB tokens")
}
