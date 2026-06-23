#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct WorthUiPrimitiveColor {
    red: u8,
    green: u8,
    blue: u8,
    transparent: bool,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiPrimitiveAppearanceReceipt {
    background_color: WorthUiPrimitiveColor,
    foreground_color: WorthUiPrimitiveColor,
}

impl WorthUiPrimitiveAppearanceReceipt {
    pub(crate) fn new(
        background_color: WorthUiPrimitiveColor,
        foreground_color: WorthUiPrimitiveColor,
    ) -> Self {
        Self {
            background_color,
            foreground_color,
        }
    }

    pub fn background_color(&self) -> WorthUiPrimitiveColor {
        self.background_color
    }

    pub fn foreground_color(&self) -> WorthUiPrimitiveColor {
        self.foreground_color
    }
}

impl WorthUiPrimitiveColor {
    pub(crate) fn new(red: u8, green: u8, blue: u8) -> Self {
        Self {
            red,
            green,
            blue,
            transparent: false,
        }
    }

    pub(crate) fn transparent() -> Self {
        Self {
            red: 0,
            green: 0,
            blue: 0,
            transparent: true,
        }
    }

    pub fn red(self) -> u8 {
        self.red
    }

    pub fn green(self) -> u8 {
        self.green
    }

    pub fn blue(self) -> u8 {
        self.blue
    }

    pub fn is_transparent(self) -> bool {
        self.transparent
    }

    pub fn hex_triplet(self) -> String {
        if self.transparent {
            return "transparent".to_owned();
        }
        format!("#{:02x}{:02x}{:02x}", self.red, self.green, self.blue)
    }
}
