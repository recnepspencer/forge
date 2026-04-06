#[derive(Debug, Clone, Copy, PartialEq, Eq, Default)]
pub(crate) struct RouteScope {
    _private: (),
}

impl RouteScope {
    pub(crate) fn begin() -> Self {
        Self { _private: () }
    }
}
