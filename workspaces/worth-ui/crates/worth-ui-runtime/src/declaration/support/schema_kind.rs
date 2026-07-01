#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiDeclarationSupportRowSchemaKind {
    QueryBinding,
    ServiceUsage,
    TouchMeaning,
    MeasurementPolicy,
    HostCapability,
}

impl UiDeclarationSupportRowSchemaKind {
    pub const fn as_support_subsystem(self) -> &'static str {
        match self {
            Self::QueryBinding => "declaration.query_binding",
            Self::ServiceUsage => "declaration.service_usage",
            Self::TouchMeaning => "declaration.touch_meaning",
            Self::MeasurementPolicy => "declaration.measurement_policy",
            Self::HostCapability => "declaration.host_capability",
        }
    }
}
