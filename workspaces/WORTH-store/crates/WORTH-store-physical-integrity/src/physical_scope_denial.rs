use crate::{ChecksumCoverageBasis, GenerationIntegrityReport};
use worth_store_physical_format::{
    CheckpointAdjacencyPosture, PhysicalGenerationOwner, PhysicalReferenceScope,
    RootManifestIntegrityPosture,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum PhysicalScopeDenialKind {
    WrongPhysicalFamily,
    StalePhysicalGeneration,
    MisplacedPhysicalIdentity,
    WrongPage,
    WrongSegment,
    WrongExtent,
    WrongManifestScope,
    WrongRootPosture,
    WrongCheckpointAdjacency,
    ChecksumScopeMismatch,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ChecksumScopeMismatchDenial {
    expected: ChecksumCoverageBasis,
    actual: ChecksumCoverageBasis,
}

impl ChecksumScopeMismatchDenial {
    pub(crate) const fn new(
        expected: ChecksumCoverageBasis,
        actual: ChecksumCoverageBasis,
    ) -> Self {
        Self { expected, actual }
    }

    pub const fn expected(&self) -> &ChecksumCoverageBasis {
        &self.expected
    }

    pub const fn actual(&self) -> &ChecksumCoverageBasis {
        &self.actual
    }
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct IntactWrongScopeDenial {
    expected: PhysicalReferenceScope,
    actual: PhysicalReferenceScope,
}

impl IntactWrongScopeDenial {
    pub(crate) const fn new(
        expected: PhysicalReferenceScope,
        actual: PhysicalReferenceScope,
    ) -> Self {
        Self { expected, actual }
    }

    pub const fn expected(self) -> PhysicalReferenceScope {
        self.expected
    }

    pub const fn actual(self) -> PhysicalReferenceScope {
        self.actual
    }
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScopeDenial {
    kind: PhysicalScopeDenialKind,
    expected_scope: Option<PhysicalReferenceScope>,
    actual_scope: Option<PhysicalReferenceScope>,
    expected_owner: Option<PhysicalGenerationOwner>,
    actual_owner: Option<PhysicalGenerationOwner>,
    root_posture: Option<RootManifestIntegrityPosture>,
    checkpoint_adjacency: Option<CheckpointAdjacencyPosture>,
    generation_report: Option<GenerationIntegrityReport>,
    checksum_mismatch: Option<ChecksumScopeMismatchDenial>,
    intact_wrong_scope: Option<IntactWrongScopeDenial>,
}

impl PhysicalScopeDenial {
    pub(crate) const fn new(kind: PhysicalScopeDenialKind) -> Self {
        Self {
            kind,
            expected_scope: None,
            actual_scope: None,
            expected_owner: None,
            actual_owner: None,
            root_posture: None,
            checkpoint_adjacency: None,
            generation_report: None,
            checksum_mismatch: None,
            intact_wrong_scope: None,
        }
    }

    pub(crate) const fn with_expected_scope(mut self, scope: PhysicalReferenceScope) -> Self {
        self.expected_scope = Some(scope);
        self
    }

    pub(crate) const fn with_actual_scope(mut self, scope: PhysicalReferenceScope) -> Self {
        self.actual_scope = Some(scope);
        self
    }

    pub(crate) const fn with_owners(
        mut self,
        expected: PhysicalGenerationOwner,
        actual: PhysicalGenerationOwner,
    ) -> Self {
        self.expected_owner = Some(expected);
        self.actual_owner = Some(actual);
        self
    }

    pub(crate) const fn with_root_posture(mut self, posture: RootManifestIntegrityPosture) -> Self {
        self.root_posture = Some(posture);
        self
    }

    pub(crate) const fn with_checkpoint_adjacency(
        mut self,
        posture: CheckpointAdjacencyPosture,
    ) -> Self {
        self.checkpoint_adjacency = Some(posture);
        self
    }

    pub(crate) const fn with_generation_report(
        mut self,
        report: GenerationIntegrityReport,
    ) -> Self {
        self.generation_report = Some(report);
        self
    }

    pub(crate) fn with_checksum_mismatch(mut self, mismatch: ChecksumScopeMismatchDenial) -> Self {
        self.checksum_mismatch = Some(mismatch);
        self
    }

    pub(crate) const fn with_intact_wrong_scope(
        mut self,
        wrong_scope: IntactWrongScopeDenial,
    ) -> Self {
        self.intact_wrong_scope = Some(wrong_scope);
        self
    }

    pub const fn kind(&self) -> PhysicalScopeDenialKind {
        self.kind
    }

    pub const fn expected_scope(&self) -> Option<PhysicalReferenceScope> {
        self.expected_scope
    }

    pub const fn actual_scope(&self) -> Option<PhysicalReferenceScope> {
        self.actual_scope
    }

    pub const fn expected_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.expected_owner
    }

    pub const fn actual_owner(&self) -> Option<PhysicalGenerationOwner> {
        self.actual_owner
    }

    pub const fn root_posture(&self) -> Option<RootManifestIntegrityPosture> {
        self.root_posture
    }

    pub const fn checkpoint_adjacency(&self) -> Option<CheckpointAdjacencyPosture> {
        self.checkpoint_adjacency
    }

    pub const fn generation_report(&self) -> Option<GenerationIntegrityReport> {
        self.generation_report
    }

    pub const fn checksum_mismatch(&self) -> Option<&ChecksumScopeMismatchDenial> {
        self.checksum_mismatch.as_ref()
    }

    pub const fn intact_wrong_scope(&self) -> Option<IntactWrongScopeDenial> {
        self.intact_wrong_scope
    }
}
