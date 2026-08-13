#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiTextPoint {
    x_millipoints: i64,
    y_millipoints: i64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiTextRect {
    left_millipoints: i64,
    top_millipoints: i64,
    right_millipoints: i64,
    bottom_millipoints: i64,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiTextFontUnitRect {
    x_min: i32,
    y_min: i32,
    x_max: i32,
    y_max: i32,
}

impl UiTextPoint {
    #[doc(hidden)]
    pub const fn from_text_mechanics(x_millipoints: i64, y_millipoints: i64) -> Self {
        Self {
            x_millipoints,
            y_millipoints,
        }
    }

    pub const fn x_millipoints(self) -> i64 {
        self.x_millipoints
    }
    pub const fn y_millipoints(self) -> i64 {
        self.y_millipoints
    }
}

impl UiTextRect {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        left_millipoints: i64,
        top_millipoints: i64,
        right_millipoints: i64,
        bottom_millipoints: i64,
    ) -> Option<Self> {
        if left_millipoints <= right_millipoints && top_millipoints <= bottom_millipoints {
            Some(Self {
                left_millipoints,
                top_millipoints,
                right_millipoints,
                bottom_millipoints,
            })
        } else {
            None
        }
    }

    pub const fn left_millipoints(self) -> i64 {
        self.left_millipoints
    }
    pub const fn top_millipoints(self) -> i64 {
        self.top_millipoints
    }
    pub const fn right_millipoints(self) -> i64 {
        self.right_millipoints
    }
    pub const fn bottom_millipoints(self) -> i64 {
        self.bottom_millipoints
    }
    pub const fn width_millipoints(self) -> i64 {
        self.right_millipoints - self.left_millipoints
    }
    pub const fn height_millipoints(self) -> i64 {
        self.bottom_millipoints - self.top_millipoints
    }
    pub const fn contains(self, point: UiTextPoint) -> bool {
        point.x_millipoints >= self.left_millipoints
            && point.x_millipoints <= self.right_millipoints
            && point.y_millipoints >= self.top_millipoints
            && point.y_millipoints <= self.bottom_millipoints
    }
}

impl UiTextFontUnitRect {
    #[doc(hidden)]
    pub const fn from_text_mechanics(
        x_min: i32,
        y_min: i32,
        x_max: i32,
        y_max: i32,
    ) -> Option<Self> {
        if x_min <= x_max && y_min <= y_max {
            Some(Self {
                x_min,
                y_min,
                x_max,
                y_max,
            })
        } else {
            None
        }
    }

    pub const fn x_min(self) -> i32 {
        self.x_min
    }
    pub const fn y_min(self) -> i32 {
        self.y_min
    }
    pub const fn x_max(self) -> i32 {
        self.x_max
    }
    pub const fn y_max(self) -> i32 {
        self.y_max
    }
}
