use crate::source::WorthUiSourceSpan;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiSurfaceComponentSelection<'a> {
    Absent,
    Selected(&'a str),
    Malformed,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiSurfaceAuthoringValue<'a> {
    Identifier(&'a str),
    NumberLiteral(u32),
    StringLiteral(&'a str),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSurfaceAuthoringProperty<'a> {
    pub(super) key: &'a str,
    pub(super) value: WorthUiSurfaceAuthoringValue<'a>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSpannedSurfaceAuthoringProperty<'a> {
    pub(super) key: &'a str,
    pub(super) source_span: &'a WorthUiSourceSpan,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSurfaceAuthoring<'a> {
    pub(super) component_id: Option<&'a str>,
    pub(super) properties: Vec<WorthUiSurfaceAuthoringProperty<'a>>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthUiSpannedSurfaceAuthoring<'a> {
    pub(super) properties: Vec<WorthUiSpannedSurfaceAuthoringProperty<'a>>,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthUiSurfaceAuthoringParseFailure {
    Malformed,
}

impl<'a> WorthUiSurfaceAuthoring<'a> {
    pub(crate) fn component_id(&self) -> Option<&'a str> {
        self.component_id
    }

    pub(crate) fn properties(&self) -> &[WorthUiSurfaceAuthoringProperty<'a>] {
        &self.properties
    }
}

impl<'a> WorthUiSurfaceAuthoringProperty<'a> {
    pub(crate) fn key(&self) -> &'a str {
        self.key
    }

    pub(crate) fn value(&self) -> WorthUiSurfaceAuthoringValue<'a> {
        self.value
    }
}

impl<'a> WorthUiSpannedSurfaceAuthoring<'a> {
    pub(crate) fn properties(&self) -> &[WorthUiSpannedSurfaceAuthoringProperty<'a>] {
        &self.properties
    }
}

impl<'a> WorthUiSpannedSurfaceAuthoringProperty<'a> {
    pub(crate) fn key(&self) -> &'a str {
        self.key
    }

    pub(crate) fn source_span(&self) -> &'a WorthUiSourceSpan {
        self.source_span
    }
}
