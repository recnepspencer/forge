#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeClientAreaBounds {
    left: i32,
    top: i32,
    right: i32,
    bottom: i32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NativeWindowIdentity(usize);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct ProcessBoundNativeClientAreaObservation {
    process_id: u32,
    window: NativeWindowIdentity,
    bounds: NativeClientAreaBounds,
    window_lookup_count: u32,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct NativeClientPixelCapture {
    process_id: u32,
    width: u32,
    height: u32,
    rgba: Vec<u8>,
    capture_count: u32,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct NormalNativeCloseRequestObservation {
    process_id: u32,
    request_count: u32,
}

impl NativeClientAreaBounds {
    pub(crate) fn new(left: i32, top: i32, right: i32, bottom: i32) -> Option<Self> {
        (right > left && bottom > top).then_some(Self {
            left,
            top,
            right,
            bottom,
        })
    }

    pub(crate) fn left(self) -> i32 {
        self.left
    }

    pub(crate) fn top(self) -> i32 {
        self.top
    }

    pub(crate) fn right(self) -> i32 {
        self.right
    }

    pub(crate) fn bottom(self) -> i32 {
        self.bottom
    }

    pub(crate) fn width(self) -> u32 {
        (self.right - self.left) as u32
    }

    pub(crate) fn height(self) -> u32 {
        (self.bottom - self.top) as u32
    }
}

impl ProcessBoundNativeClientAreaObservation {
    pub(crate) fn new(
        process_id: u32,
        window: NativeWindowIdentity,
        bounds: NativeClientAreaBounds,
        window_lookup_count: u32,
    ) -> Self {
        Self {
            process_id,
            window,
            bounds,
            window_lookup_count,
        }
    }

    pub(crate) fn process_id(self) -> u32 {
        self.process_id
    }

    pub(crate) fn bounds(self) -> NativeClientAreaBounds {
        self.bounds
    }

    pub(crate) fn window(self) -> NativeWindowIdentity {
        self.window
    }

    pub(crate) fn window_lookup_count(self) -> u32 {
        self.window_lookup_count
    }
}

impl NativeWindowIdentity {
    pub(crate) fn from_native_value(value: usize) -> Option<Self> {
        (value != 0).then_some(Self(value))
    }
}

impl NativeClientPixelCapture {
    pub(crate) fn new(process_id: u32, width: u32, height: u32, rgba: Vec<u8>) -> Option<Self> {
        let expected = (width as usize)
            .checked_mul(height as usize)?
            .checked_mul(4)?;
        (rgba.len() == expected).then_some(Self {
            process_id,
            width,
            height,
            rgba,
            capture_count: 1,
        })
    }

    pub(crate) fn process_id(&self) -> u32 {
        self.process_id
    }

    pub(crate) fn width(&self) -> u32 {
        self.width
    }

    pub(crate) fn height(&self) -> u32 {
        self.height
    }

    pub(crate) fn rgba(&self) -> &[u8] {
        &self.rgba
    }

    pub(crate) fn capture_count(&self) -> u32 {
        self.capture_count
    }
}

impl NormalNativeCloseRequestObservation {
    pub(crate) fn one(process_id: u32) -> Self {
        Self {
            process_id,
            request_count: 1,
        }
    }

    pub(crate) fn process_id(self) -> u32 {
        self.process_id
    }

    pub(crate) fn request_count(self) -> u32 {
        self.request_count
    }
}
