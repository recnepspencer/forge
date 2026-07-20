#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum WorthServerQueryDependencyAuditPathKind {
    WorthNativeDirectRead,
    WorthNativeDirectState,
    WorthNativeDirectInspection,
    WorthNativeDirectProjection,
    WorthNativeDirectMutation,
    DirectDeclarationSupportPosture,
    CompatibilityHttpRead,
    CompatibilityHttpMutation,
    QueryHandoffRead,
    QueryHandoffMutation,
    QueryHandoffDownstreamDelivery,
    ServerConsumerBoundaryAudit,
    CertificationTestBackendSupport,
}

impl WorthServerQueryDependencyAuditPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::WorthNativeDirectRead => "Worth-native-direct-read",
            Self::WorthNativeDirectState => "Worth-native-direct-state",
            Self::WorthNativeDirectInspection => "Worth-native-direct-inspection",
            Self::WorthNativeDirectProjection => "Worth-native-direct-projection",
            Self::WorthNativeDirectMutation => "Worth-native-direct-mutation",
            Self::DirectDeclarationSupportPosture => "direct-declaration-support-posture",
            Self::CompatibilityHttpRead => "compat-http-read",
            Self::CompatibilityHttpMutation => "compat-http-mutation",
            Self::QueryHandoffRead => "query-handoff-read",
            Self::QueryHandoffMutation => "query-handoff-mutation",
            Self::QueryHandoffDownstreamDelivery => "query-handoff-downstream-delivery",
            Self::ServerConsumerBoundaryAudit => "server-consumer-boundary-audit",
            Self::CertificationTestBackendSupport => "certification-test-backend-support",
        }
    }
}
