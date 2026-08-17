use worth_ui_host_contract::{
    UiFontCollectionGeneration, UiQualifiedTextLayoutIdentity, UiTextProfileGeneration,
    UiTextScaleGeneration,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiHeadlessTextMeasurement {
    layout: UiQualifiedTextLayoutIdentity,
    width_millipoints: i64,
    height_millipoints: i64,
    first_baseline_millipoints: Option<i64>,
    logical_bounds: worth_ui_host_contract::UiTextRect,
    ink_bounds: worth_ui_host_contract::UiTextRect,
    profile: UiTextProfileGeneration,
    font_collection: UiFontCollectionGeneration,
    text_scale: UiTextScaleGeneration,
}

impl super::UiHeadlessSemanticTextMechanic {
    pub fn qualified_measurement(&self) -> UiHeadlessTextMeasurement {
        let width_millipoints = content_width(self.lines());
        let height_millipoints = self
            .lines()
            .iter()
            .map(|line| line.bounds().bottom_millipoints())
            .max()
            .unwrap_or(0);
        UiHeadlessTextMeasurement {
            layout: self.layout_identity(),
            width_millipoints,
            height_millipoints,
            first_baseline_millipoints: self
                .lines()
                .first()
                .map(|line| line.baseline_millipoints()),
            logical_bounds: self.logical_bounds(),
            ink_bounds: self.ink_bounds(),
            profile: self.profile_generation(),
            font_collection: self.font_collection_generation(),
            text_scale: self.text_scale_generation(),
        }
    }
}

fn content_width(lines: &[worth_ui_host_contract::UiQualifiedTextLineRecord]) -> i64 {
    lines
        .iter()
        .map(|line| line.bounds().width_millipoints())
        .max()
        .unwrap_or(0)
}

#[cfg(test)]
mod tests {
    use super::content_width;
    use worth_ui_host_contract::{
        UiQualifiedTextLineInput, UiQualifiedTextLineRecord, UiTextOriginalRange, UiTextRect,
    };

    #[test]
    fn aligned_line_offset_is_not_counted_as_content_width() {
        let line = UiQualifiedTextLineRecord::from_text_mechanics(UiQualifiedTextLineInput {
            original_range: UiTextOriginalRange::new(0, 5).unwrap(),
            visual_run_start: 0,
            visual_run_end: 1,
            logical_bounds: UiTextRect::from_text_mechanics(40_000, 0, 100_000, 18_000).unwrap(),
            ink_bounds: UiTextRect::from_text_mechanics(39_000, 1_000, 101_000, 17_000).unwrap(),
            baseline_millipoints: 14_000,
            hard_break: false,
            overflowed: false,
        });

        assert_eq!(content_width(&[line]), 60_000);
    }
}

impl UiHeadlessTextMeasurement {
    pub const fn layout_identity(self) -> UiQualifiedTextLayoutIdentity {
        self.layout
    }
    pub const fn width_millipoints(self) -> i64 {
        self.width_millipoints
    }
    pub const fn height_millipoints(self) -> i64 {
        self.height_millipoints
    }
    pub const fn first_baseline_millipoints(self) -> Option<i64> {
        self.first_baseline_millipoints
    }
    pub const fn logical_bounds(self) -> worth_ui_host_contract::UiTextRect {
        self.logical_bounds
    }
    pub const fn ink_bounds(self) -> worth_ui_host_contract::UiTextRect {
        self.ink_bounds
    }
    pub const fn profile_generation(self) -> UiTextProfileGeneration {
        self.profile
    }
    pub const fn font_collection_generation(self) -> UiFontCollectionGeneration {
        self.font_collection
    }
    pub const fn text_scale_generation(self) -> UiTextScaleGeneration {
        self.text_scale
    }
}
