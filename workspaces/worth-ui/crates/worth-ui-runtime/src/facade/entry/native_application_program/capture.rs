use super::UiNativeApplicationFrame;

impl UiNativeApplicationFrame {
    /// Capture the exact native presented source after this frame settles.
    ///
    /// This is bounded reconstructive inspection work. It is never part of
    /// ordinary presentation cost or authority.
    pub fn capture_presented_source_pixels(mut self) -> Self {
        self.capture_presented_source_pixels = true;
        self
    }

    pub(crate) const fn captures_presented_source_pixels(&self) -> bool {
        self.capture_presented_source_pixels
    }
}
