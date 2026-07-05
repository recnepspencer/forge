use crate::UiInspectionSupportReport;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementFailureSource {
    DeclarationPosture,
    QueryFacts,
    HostEvidence,
    CompatibilityMismatch,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementBasisPosture {
    QueryOnly,
    HostOnly,
    QueryAndHost,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementEvidenceCategory {
    TextIntrinsicSize,
    TextBaselineMetrics,
    FontMetrics,
    NativeControlIntrinsicSize,
    ViewportExtent,
    DpiScaleFactor,
    PortalAnchorRect,
    ScrollContainerViewport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementEvidenceSlot {
    QueryProjectionFactReceipt,
    HostCapabilityReport,
    HostFontMetrics,
    ViewportExtent,
    PortalAnchorRect,
    ScrollContainerViewport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementQueryUnsupportedReason {
    MissingQueryPrerequisites,
    WrongWorldProjection,
    RebindRequired,
    AmbiguousSources,
    ProjectionConsumptionUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementQueryFactFamily {
    ScrollContentExtent,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementBasisSource {
    ScrollViewport,
    PortalAnchor,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementOwnershipPosture {
    ScrollContainerBasis,
    PortalAnchorBasisRequired,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq, Ord, PartialOrd)]
pub enum UiInspectionMeasurementDependencyLineageKind {
    QueryScrollContentExtent,
    HostFontMetrics,
    HostViewportExtent,
    HostPortalAnchorRect,
    HostScrollContainerViewport,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct UiInspectionMeasurementDependencyLineageEntry {
    kind: UiInspectionMeasurementDependencyLineageKind,
    identity_digest: u64,
    generation_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementNeighborhoodClassHint {
    LocalIntrinsicContentDependency,
    ContainerAvailableSpaceDependency,
    ViewportDependency,
    ScrollContainerDependency,
    PortalAnchorDependency,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum UiInspectionMeasurementGenerationCompatibility {
    Compatible,
    StaleQueryFactReceipt {
        expected: u64,
        observed: u64,
    },
    StaleHostEvidence {
        expected: u64,
        observed: u64,
    },
    StaleHostCapability {
        expected: u64,
        observed: u64,
    },
    IncompatibleWorld {
        expected_query_basis_digest: Box<str>,
        observed_world_basis_digest: Option<Box<str>>,
    },
    IncompatibleHostProfile {
        expected_profile_digest: u64,
        observed_profile_digest: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionMeasurementDenialPosture {
    GenerationIncompatible {
        compatibility: UiInspectionMeasurementGenerationCompatibility,
    },
    AmbiguousGraphNodeInstances {
        instance_count: usize,
    },
    UnsupportedQueryPosture {
        reason: UiInspectionMeasurementQueryUnsupportedReason,
    },
    UnavailableFactFamilies {
        available_families: Box<[UiInspectionMeasurementQueryFactFamily]>,
        missing_families: Box<[UiInspectionMeasurementQueryFactFamily]>,
    },
    MissingEvidence {
        slot: UiInspectionMeasurementEvidenceSlot,
    },
    MissingBasisSourceEvidence {
        basis_source: UiInspectionMeasurementBasisSource,
        slot: UiInspectionMeasurementEvidenceSlot,
    },
    MissingOwnershipEvidence {
        ownership_posture: UiInspectionMeasurementOwnershipPosture,
        slot: UiInspectionMeasurementEvidenceSlot,
    },
    MissingRequiredMeasurementEvidence {
        category: UiInspectionMeasurementEvidenceCategory,
        slot: UiInspectionMeasurementEvidenceSlot,
    },
    ConflictingEvidenceInputs {
        slot: UiInspectionMeasurementEvidenceSlot,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub enum UiInspectionMeasurementBasisInput {
    QueryProjectionFact {
        query_basis_digest: Box<str>,
        projection_contract_digest: Box<str>,
        required_fact_family_set_digest: u64,
        consumed_fact_family_set_digest: u64,
    },
    HostCapabilityReport {
        profile_digest: u64,
        observation_generation: u64,
    },
    HostMeasurementResult {
        category: UiInspectionMeasurementEvidenceCategory,
        identity_digest: u64,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
#[non_exhaustive]
pub struct UiInspectionMeasurementEvidenceView {
    support_report: UiInspectionSupportReport,
    basis_posture: Option<UiInspectionMeasurementBasisPosture>,
    denial_posture: Option<UiInspectionMeasurementDenialPosture>,
    basis_inputs: Box<[UiInspectionMeasurementBasisInput]>,
    dependency_lineage: Box<[UiInspectionMeasurementDependencyLineageEntry]>,
    generation_compatibility: Option<UiInspectionMeasurementGenerationCompatibility>,
    neighborhood_class_hint: Option<UiInspectionMeasurementNeighborhoodClassHint>,
    failure_source: Option<UiInspectionMeasurementFailureSource>,
}

impl UiInspectionMeasurementDependencyLineageEntry {
    pub const fn new(
        kind: UiInspectionMeasurementDependencyLineageKind,
        identity_digest: u64,
        generation_digest: u64,
    ) -> Self {
        Self {
            kind,
            identity_digest,
            generation_digest,
        }
    }

    pub const fn kind(self) -> UiInspectionMeasurementDependencyLineageKind {
        self.kind
    }

    pub const fn identity_digest(self) -> u64 {
        self.identity_digest
    }

    pub const fn generation_digest(self) -> u64 {
        self.generation_digest
    }
}

impl UiInspectionMeasurementEvidenceView {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        support_report: UiInspectionSupportReport,
        basis_posture: Option<UiInspectionMeasurementBasisPosture>,
        denial_posture: Option<UiInspectionMeasurementDenialPosture>,
        basis_inputs: Box<[UiInspectionMeasurementBasisInput]>,
        dependency_lineage: Box<[UiInspectionMeasurementDependencyLineageEntry]>,
        generation_compatibility: Option<UiInspectionMeasurementGenerationCompatibility>,
        neighborhood_class_hint: Option<UiInspectionMeasurementNeighborhoodClassHint>,
        failure_source: Option<UiInspectionMeasurementFailureSource>,
    ) -> Self {
        Self {
            support_report,
            basis_posture,
            denial_posture,
            basis_inputs,
            dependency_lineage,
            generation_compatibility,
            neighborhood_class_hint,
            failure_source,
        }
    }

    pub fn support_report(&self) -> UiInspectionSupportReport {
        self.support_report
    }

    pub fn basis_posture(&self) -> Option<UiInspectionMeasurementBasisPosture> {
        self.basis_posture
    }

    pub fn denial_posture(&self) -> Option<&UiInspectionMeasurementDenialPosture> {
        self.denial_posture.as_ref()
    }

    pub fn basis_inputs(&self) -> &[UiInspectionMeasurementBasisInput] {
        &self.basis_inputs
    }

    pub fn dependency_lineage(&self) -> &[UiInspectionMeasurementDependencyLineageEntry] {
        &self.dependency_lineage
    }

    pub fn generation_compatibility(
        &self,
    ) -> Option<&UiInspectionMeasurementGenerationCompatibility> {
        self.generation_compatibility.as_ref()
    }

    pub fn neighborhood_class_hint(&self) -> Option<UiInspectionMeasurementNeighborhoodClassHint> {
        self.neighborhood_class_hint
    }

    pub fn failure_source(&self) -> Option<UiInspectionMeasurementFailureSource> {
        self.failure_source
    }
}
