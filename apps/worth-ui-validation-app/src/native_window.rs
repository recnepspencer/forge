use eframe::{egui, NativeOptions};

const INITIAL_INNER_SIZE: egui::Vec2 = egui::vec2(1180.0, 760.0);
const MIN_INNER_SIZE: egui::Vec2 = egui::vec2(640.0, 420.0);
const MAX_INNER_SIZE: egui::Vec2 = egui::vec2(3840.0, 2160.0);

pub fn validation_native_options() -> NativeOptions {
    NativeOptions {
        viewport: egui::ViewportBuilder::default()
            .with_inner_size(INITIAL_INNER_SIZE)
            .with_min_inner_size(MIN_INNER_SIZE)
            .with_max_inner_size(MAX_INNER_SIZE)
            .with_clamp_size_to_monitor_size(true),
        persist_window: false,
        ..NativeOptions::default()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn validation_window_options_bound_native_surface_size() {
        let options = validation_native_options();

        assert_eq!(options.viewport.inner_size, Some(INITIAL_INNER_SIZE));
        assert_eq!(options.viewport.min_inner_size, Some(MIN_INNER_SIZE));
        assert_eq!(options.viewport.max_inner_size, Some(MAX_INNER_SIZE));
        assert_eq!(options.viewport.clamp_size_to_monitor_size, Some(true));
        assert!(!options.persist_window);
    }
}
