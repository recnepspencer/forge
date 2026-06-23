#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum ForgeServerQueryDependencyAuditPathKind {
    ForgeNativeDirectRead,
    ForgeNativeDirectState,
    ForgeNativeDirectInspection,
    ForgeNativeDirectProjection,
    ForgeNativeDirectMutation,
    DirectDeclarationSupportPosture,
    CompatibilityHttpRead,
    CompatibilityHttpMutation,
    QueryHandoffRead,
    QueryHandoffMutation,
    QueryHandoffDownstreamDelivery,
    ServerConsumerBoundaryAudit,
    CertificationTestBackendSupport,
}

impl ForgeServerQueryDependencyAuditPathKind {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::ForgeNativeDirectRead => "forge-native-direct-read",
            Self::ForgeNativeDirectState => "forge-native-direct-state",
            Self::ForgeNativeDirectInspection => "forge-native-direct-inspection",
            Self::ForgeNativeDirectProjection => "forge-native-direct-projection",
            Self::ForgeNativeDirectMutation => "forge-native-direct-mutation",
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
