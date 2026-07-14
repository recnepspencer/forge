#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiFontMeasurementKey {
    token: Box<str>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTextIntrinsicSizeRequest {
    text: Box<str>,
    font: UiFontMeasurementKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiTextBaselineMetricsRequest {
    text: Box<str>,
    font: UiFontMeasurementKey,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiFontMetricsRequest {
    font: UiFontMeasurementKey,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiNativeControlKind {
    Button,
    Checkbox,
    TextField,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct UiNativeControlIntrinsicSizeRequest {
    kind: UiNativeControlKind,
    label: Option<Box<str>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiViewportExtentRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiDpiScaleFactorRequest;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPortalAnchorTargetIdentity(u64);

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiPortalAnchorRectRequest {
    anchor_identity: UiPortalAnchorTargetIdentity,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiScrollContainerViewportRequest {
    container_identity: u64,
}

impl UiFontMeasurementKey {
    pub fn new(token: impl Into<Box<str>>) -> Self {
        Self {
            token: token.into(),
        }
    }

    pub fn token(&self) -> &str {
        &self.token
    }
}

impl UiTextIntrinsicSizeRequest {
    pub fn single_line(text: impl Into<Box<str>>, font: UiFontMeasurementKey) -> Self {
        Self {
            text: text.into(),
            font,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font(&self) -> &UiFontMeasurementKey {
        &self.font
    }
}

impl UiTextBaselineMetricsRequest {
    pub fn single_line(text: impl Into<Box<str>>, font: UiFontMeasurementKey) -> Self {
        Self {
            text: text.into(),
            font,
        }
    }

    pub fn text(&self) -> &str {
        &self.text
    }

    pub fn font(&self) -> &UiFontMeasurementKey {
        &self.font
    }
}

impl UiFontMetricsRequest {
    pub fn new(font: UiFontMeasurementKey) -> Self {
        Self { font }
    }

    pub fn font(&self) -> &UiFontMeasurementKey {
        &self.font
    }
}

impl UiNativeControlIntrinsicSizeRequest {
    pub fn new(kind: UiNativeControlKind, label: Option<impl Into<Box<str>>>) -> Self {
        Self {
            kind,
            label: label.map(|value| value.into()),
        }
    }

    pub fn kind(&self) -> UiNativeControlKind {
        self.kind
    }

    pub fn label(&self) -> Option<&str> {
        self.label.as_deref()
    }
}

impl UiPortalAnchorRectRequest {
    pub const fn new(anchor_identity: u64) -> Self {
        Self {
            anchor_identity: UiPortalAnchorTargetIdentity(anchor_identity),
        }
    }

    pub const fn anchor_identity(&self) -> u64 {
        self.anchor_identity.raw()
    }

    pub const fn target_identity(&self) -> UiPortalAnchorTargetIdentity {
        self.anchor_identity
    }
}

impl UiPortalAnchorTargetIdentity {
    pub const fn new(raw: u64) -> Self {
        Self(raw)
    }

    pub const fn raw(self) -> u64 {
        self.0
    }
}

impl UiScrollContainerViewportRequest {
    pub const fn new(container_identity: u64) -> Self {
        Self { container_identity }
    }

    pub const fn container_identity(&self) -> u64 {
        self.container_identity
    }
}
