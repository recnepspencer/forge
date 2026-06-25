#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthTopologyQueryNativeRuntimeBoundaryResidueStatus {
    MigrateToNativeBoundary,
    DeleteWithNativeReplacement,
    CertificationSupportCutover,
    TerminalSupportCodecOnly,
    FirewallPatternOnly,
    ExplicitUpstreamBlocker,
    Unclassified,
}

impl WorthTopologyQueryNativeRuntimeBoundaryResidueStatus {
    pub const fn as_str(self) -> &'static str {
        match self {
            Self::MigrateToNativeBoundary => "migrate-to-native-boundary",
            Self::DeleteWithNativeReplacement => "delete-with-native-replacement",
            Self::CertificationSupportCutover => "certification-support-cutover",
            Self::TerminalSupportCodecOnly => "terminal-support-codec-only",
            Self::FirewallPatternOnly => "firewall-pattern-only",
            Self::ExplicitUpstreamBlocker => "explicit-upstream-blocker",
            Self::Unclassified => "unclassified",
        }
    }

    pub const fn is_classified(self) -> bool {
        !matches!(self, Self::Unclassified)
    }

    pub const fn is_ordinary_runtime_migration(self) -> bool {
        matches!(
            self,
            Self::MigrateToNativeBoundary | Self::DeleteWithNativeReplacement
        )
    }
}
