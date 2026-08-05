use worth_foundational::facade::{CanonicalExportManifestMismatch, CanonicalMismatchBasis};

mod counters;
mod kind;

pub use counters::WorthQueryCompatibilityCounters;
pub use kind::WorthQueryCompatibilityDenialKind;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum WorthQueryCompatibilityUseDenial {
    WrongCapabilityPair,
    StaleAuthority,
    StaleConditionalLowering,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryCompatibilityDenial {
    kind: WorthQueryCompatibilityDenialKind,
    canonical_mismatch: Option<CanonicalMismatchBasis>,
    export_manifest_mismatch: Option<CanonicalExportManifestMismatch>,
    portable_operation_dimension:
        Option<worth_query_installation::facade::WorthQueryPortableOperationDimension>,
    portable_operation_category: Option<
        worth_query_installation::facade::WorthQueryPortableOperationComparisonMismatchCategory,
    >,
    installed_conditional_dimension:
        Option<worth_query_installation::facade::WorthQueryPortableConditionalDimension>,
    conditional_continuity_mismatch:
        Option<worth_runtime_bridge::facade::BridgeConditionalContinuityMismatch>,
    conditional_affinity_mismatch:
        Option<worth_runtime_bridge::facade::BridgeConditionalExecutionAffinityMismatch>,
    detail: &'static str,
    counters: WorthQueryCompatibilityCounters,
}

impl WorthQueryCompatibilityDenial {
    pub(crate) fn canonical_mismatch(&self) -> Option<&CanonicalMismatchBasis> {
        self.canonical_mismatch.as_ref()
    }

    pub(crate) fn set_kind(&mut self, kind: WorthQueryCompatibilityDenialKind) {
        self.kind = kind;
    }

    pub(crate) fn plain(
        kind: WorthQueryCompatibilityDenialKind,
        detail: &'static str,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            kind,
            canonical_mismatch: None,
            export_manifest_mismatch: None,
            portable_operation_dimension: None,
            portable_operation_category: None,
            installed_conditional_dimension: None,
            conditional_continuity_mismatch: None,
            conditional_affinity_mismatch: None,
            detail,
            counters,
        }
    }

    pub(crate) fn canonical(
        kind: WorthQueryCompatibilityDenialKind,
        mismatch: CanonicalMismatchBasis,
        detail: &'static str,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        Self {
            kind,
            canonical_mismatch: Some(mismatch),
            export_manifest_mismatch: None,
            portable_operation_dimension: None,
            portable_operation_category: None,
            installed_conditional_dimension: None,
            conditional_continuity_mismatch: None,
            conditional_affinity_mismatch: None,
            detail,
            counters,
        }
    }

    pub(crate) fn portable_operation_mismatch(
        mismatch: worth_query_installation::facade::WorthQueryPortableOperationComparisonMismatch,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let dimension = mismatch.dimension().clone();
        Self {
            kind: mismatch_kind(&dimension, false),
            canonical_mismatch: mismatch.foundational_basis().cloned(),
            export_manifest_mismatch: mismatch.export_manifest_mismatch().cloned(),
            portable_operation_dimension: Some(dimension),
            portable_operation_category: Some(mismatch.category()),
            installed_conditional_dimension: None,
            conditional_continuity_mismatch: None,
            conditional_affinity_mismatch: None,
            detail: "portable operation meaning differs",
            counters,
        }
    }

    pub(crate) fn portable_operation_unsupported(
        mismatch: worth_query_installation::facade::WorthQueryPortableOperationComparisonUnsupported,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let dimension = mismatch.dimension().clone();
        Self {
            kind: mismatch_kind(&dimension, true),
            canonical_mismatch: Some(mismatch.foundational_basis().clone()),
            export_manifest_mismatch: None,
            portable_operation_dimension: Some(dimension),
            portable_operation_category: Some(
                worth_query_installation::facade::WorthQueryPortableOperationComparisonMismatchCategory::Foundational,
            ),
            installed_conditional_dimension: None,
            conditional_continuity_mismatch: None,
            conditional_affinity_mismatch: None,
            detail: "portable operation comparison is unsupported",
            counters,
        }
    }

    pub(crate) fn conditional_continuity(
        mismatch: worth_runtime_bridge::facade::BridgeConditionalContinuityMismatch,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let mut denial = Self::plain(
            WorthQueryCompatibilityDenialKind::ConditionalLowering,
            "installed conditional lowering lacks semantic continuity",
            counters,
        );
        denial.conditional_continuity_mismatch = Some(mismatch);
        denial
    }

    pub(crate) fn installed_conditional_mismatch(
        mismatch: worth_query_installation::facade::WorthQueryPortableConditionalComparisonMismatch,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let mut denial = Self::canonical(
            WorthQueryCompatibilityDenialKind::PortableConditionalMismatched,
            mismatch.foundational_basis().clone(),
            "installed Query conditional meaning differs",
            counters,
        );
        denial.installed_conditional_dimension = Some(mismatch.dimension().clone());
        denial
    }

    pub(crate) fn installed_conditional_unsupported(
        mismatch: worth_query_installation::facade::WorthQueryPortableConditionalComparisonUnsupported,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let mut denial = Self::canonical(
            WorthQueryCompatibilityDenialKind::PortableConditionalUnsupported,
            mismatch.foundational_basis().clone(),
            "installed Query conditional comparison is unsupported",
            counters,
        );
        denial.installed_conditional_dimension = Some(mismatch.dimension().clone());
        denial
    }

    pub(crate) fn conditional_affinity(
        mismatch: worth_runtime_bridge::facade::BridgeConditionalExecutionAffinityMismatch,
        counters: WorthQueryCompatibilityCounters,
    ) -> Self {
        let mut denial = Self::plain(
            WorthQueryCompatibilityDenialKind::ConditionalLowering,
            "installed conditional lowering lacks execution affinity",
            counters,
        );
        denial.conditional_affinity_mismatch = Some(mismatch);
        denial
    }
}

fn mismatch_kind(
    dimension: &worth_query_installation::facade::WorthQueryPortableOperationDimension,
    unsupported: bool,
) -> WorthQueryCompatibilityDenialKind {
    // Installed Query declarations are canonicalized by their owner under one
    // fixed rule version, so `Unsupported` cannot currently be reached through
    // an admitted installed public path. The installation-owner comparator
    // tests exercise cross-version Unsupported directly; Query retains this
    // mapping for a future owner-issued outcome rather than minting a fake
    // installed contract solely to make the branch reachable here.
    use worth_query_installation::facade::WorthQueryPortableOperationDimension as Dimension;
    match (dimension, unsupported) {
        (Dimension::NativeContract, false) => {
            WorthQueryCompatibilityDenialKind::NativeContractMismatched
        }
        (Dimension::NativeContract, true) => {
            WorthQueryCompatibilityDenialKind::NativeContractUnsupported
        }
        (Dimension::NativeProjectionMask, false) => {
            WorthQueryCompatibilityDenialKind::NativeMaskMismatched
        }
        (Dimension::NativeProjectionMask, true) => {
            WorthQueryCompatibilityDenialKind::NativeMaskUnsupported
        }
        (Dimension::NativeExport, _) => WorthQueryCompatibilityDenialKind::NativeProducerShape,
        (Dimension::Conditional(_), false) => {
            WorthQueryCompatibilityDenialKind::PortableConditionalMismatched
        }
        (Dimension::Conditional(_), true) => {
            WorthQueryCompatibilityDenialKind::PortableConditionalUnsupported
        }
        _ => WorthQueryCompatibilityDenialKind::PortableOperationContract,
    }
}

macro_rules! relationship_denial {
    ($name:ident) => {
        #[derive(Clone, Debug, Eq, PartialEq)]
        pub struct $name(WorthQueryCompatibilityDenial);

        impl $name {
            pub fn kind(&self) -> WorthQueryCompatibilityDenialKind {
                self.0.kind
            }

            pub fn canonical_mismatch(&self) -> Option<&CanonicalMismatchBasis> {
                self.0.canonical_mismatch.as_ref()
            }

            pub fn export_manifest_mismatch(&self) -> Option<&CanonicalExportManifestMismatch> {
                self.0.export_manifest_mismatch.as_ref()
            }

            pub fn portable_conditional_dimension(
                &self,
            ) -> Option<&worth_query_installation::facade::WorthQueryOperationConditionalDimension>
            {
                match self.0.portable_operation_dimension.as_ref() {
                    Some(
                        worth_query_installation::facade::WorthQueryPortableOperationDimension::Conditional(
                            dimension,
                        ),
                    ) => Some(dimension),
                    _ => None,
                }
            }

            pub fn portable_operation_dimension(
                &self,
            ) -> Option<&worth_query_installation::facade::WorthQueryPortableOperationDimension>
            {
                self.0.portable_operation_dimension.as_ref()
            }

            pub fn portable_operation_mismatch_category(
                &self,
            ) -> Option<
                worth_query_installation::facade::WorthQueryPortableOperationComparisonMismatchCategory,
            > {
                self.0.portable_operation_category
            }

            pub fn installed_conditional_dimension(
                &self,
            ) -> Option<&worth_query_installation::facade::WorthQueryPortableConditionalDimension>
            {
                self.0.installed_conditional_dimension.as_ref()
            }

            pub fn detail(&self) -> &'static str {
                self.0.detail
            }

            pub fn counters(&self) -> WorthQueryCompatibilityCounters {
                self.0.counters
            }
        }

        impl From<WorthQueryCompatibilityDenial> for $name {
            fn from(value: WorthQueryCompatibilityDenial) -> Self {
                Self(value)
            }
        }
    };
}

macro_rules! continuity_denial {
    ($name:ident) => {
        impl $name {
            pub fn conditional_continuity_mismatch(
                &self,
            ) -> Option<&worth_runtime_bridge::facade::BridgeConditionalContinuityMismatch> {
                self.0.conditional_continuity_mismatch.as_ref()
            }
        }
    };
}

macro_rules! affinity_denial {
    ($name:ident) => {
        impl $name {
            pub fn conditional_affinity_mismatch(
                &self,
            ) -> Option<&worth_runtime_bridge::facade::BridgeConditionalExecutionAffinityMismatch>
            {
                self.0.conditional_affinity_mismatch.as_ref()
            }
        }
    };
}

relationship_denial!(WorthQuerySameInstallationDenial);
relationship_denial!(WorthQueryReplacementDenial);
relationship_denial!(WorthQueryRebindCompatibilityDenial);
relationship_denial!(WorthQueryExecutionSharingDenial);
relationship_denial!(WorthQueryBasisCompatibilityDenial);

affinity_denial!(WorthQuerySameInstallationDenial);
affinity_denial!(WorthQueryExecutionSharingDenial);
affinity_denial!(WorthQueryReplacementDenial);
continuity_denial!(WorthQueryRebindCompatibilityDenial);
continuity_denial!(WorthQueryBasisCompatibilityDenial);
