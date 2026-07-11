use crate::{
    ChecksumScopeMismatchDenial, GenerationIntegrityReport, IntactWrongScopeDenial,
    IntegrityCheckedFrame, IntegrityCheckedPage, IntegrityCheckedPhysicalFormKind,
    PhysicalScopeAdmissionRequest, PhysicalScopeBasis, PhysicalScopeDenial,
    PhysicalScopeDenialKind,
};
use forge_store_physical_format::{
    PhysicalGenerationOwner, PhysicalReferenceScope, PhysicalScopeFamily,
};

#[derive(Debug, Clone, PartialEq, Eq)]
enum ScopedCheckedPhysicalForm<'lease> {
    Page(IntegrityCheckedPage<'lease>),
    Frame(IntegrityCheckedFrame<'lease>),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct PhysicalScopeAdmission<'lease> {
    checked: ScopedCheckedPhysicalForm<'lease>,
    basis: PhysicalScopeBasis,
}

impl<'lease> PhysicalScopeAdmission<'lease> {
    pub fn admit_page(
        checked: IntegrityCheckedPage<'lease>,
        request: PhysicalScopeAdmissionRequest,
    ) -> Result<Self, PhysicalScopeDenial> {
        admit_checked_form(ScopedCheckedPhysicalForm::Page(checked), request)
    }

    pub fn admit_frame(
        checked: IntegrityCheckedFrame<'lease>,
        request: PhysicalScopeAdmissionRequest,
    ) -> Result<Self, PhysicalScopeDenial> {
        admit_checked_form(ScopedCheckedPhysicalForm::Frame(checked), request)
    }

    pub const fn basis(&self) -> &PhysicalScopeBasis {
        &self.basis
    }

    pub const fn checked_form_kind(&self) -> IntegrityCheckedPhysicalFormKind {
        match &self.checked {
            ScopedCheckedPhysicalForm::Page(_) => IntegrityCheckedPhysicalFormKind::Page,
            ScopedCheckedPhysicalForm::Frame(_) => IntegrityCheckedPhysicalFormKind::Frame,
        }
    }

    pub const fn scope_family(&self) -> PhysicalScopeFamily {
        self.basis.family()
    }

    pub(crate) const fn checked_page(&self) -> Option<&IntegrityCheckedPage<'lease>> {
        match &self.checked {
            ScopedCheckedPhysicalForm::Page(page) => Some(page),
            ScopedCheckedPhysicalForm::Frame(_) => None,
        }
    }

    pub(crate) const fn checked_frame(&self) -> Option<&IntegrityCheckedFrame<'lease>> {
        match &self.checked {
            ScopedCheckedPhysicalForm::Frame(frame) => Some(frame),
            ScopedCheckedPhysicalForm::Page(_) => None,
        }
    }
}

fn admit_checked_form<'lease>(
    checked: ScopedCheckedPhysicalForm<'lease>,
    request: PhysicalScopeAdmissionRequest,
) -> Result<PhysicalScopeAdmission<'lease>, PhysicalScopeDenial> {
    reject_scope_family_mismatch(&checked, request.scope())?;
    reject_root_posture(&request)?;
    reject_checkpoint_adjacency(&request)?;
    reject_manifest_scope_mismatch(&request)?;
    reject_checksum_scope_mismatch(&checked, request.checksum_scope())?;
    let generation_report =
        GenerationIntegrityReport::compare(request.scope().owner(), checked_owner(&checked));
    reject_generation_mismatch(generation_report, request.scope())?;
    Ok(PhysicalScopeAdmission {
        basis: PhysicalScopeBasis::new(
            checked_identity(&checked),
            request.scope(),
            request.membership(),
            request.root_posture(),
            request.checkpoint_adjacency(),
            request.checksum_scope().clone(),
            generation_report,
        ),
        checked,
    })
}

fn reject_scope_family_mismatch(
    checked: &ScopedCheckedPhysicalForm<'_>,
    scope: PhysicalReferenceScope,
) -> Result<(), PhysicalScopeDenial> {
    if checked_family_accepts_scope(checked, scope.family()) {
        return Ok(());
    }
    Err(
        PhysicalScopeDenial::new(PhysicalScopeDenialKind::WrongPhysicalFamily)
            .with_expected_scope(scope),
    )
}

fn reject_root_posture(request: &PhysicalScopeAdmissionRequest) -> Result<(), PhysicalScopeDenial> {
    if request.root_posture().admits_scope()
        && request.root_posture().root_owner() == Some(request.membership().root_owner())
    {
        return Ok(());
    }
    Err(
        PhysicalScopeDenial::new(PhysicalScopeDenialKind::WrongRootPosture)
            .with_expected_scope(request.scope())
            .with_root_posture(request.root_posture()),
    )
}

fn reject_checkpoint_adjacency(
    request: &PhysicalScopeAdmissionRequest,
) -> Result<(), PhysicalScopeDenial> {
    if request.checkpoint_adjacency().admits_scope() {
        return Ok(());
    }
    Err(
        PhysicalScopeDenial::new(PhysicalScopeDenialKind::WrongCheckpointAdjacency)
            .with_expected_scope(request.scope())
            .with_checkpoint_adjacency(request.checkpoint_adjacency()),
    )
}

fn reject_manifest_scope_mismatch(
    request: &PhysicalScopeAdmissionRequest,
) -> Result<(), PhysicalScopeDenial> {
    if request.membership().scope() == request.scope() {
        return Ok(());
    }
    Err(
        PhysicalScopeDenial::new(PhysicalScopeDenialKind::WrongManifestScope)
            .with_expected_scope(request.scope())
            .with_actual_scope(request.membership().scope())
            .with_intact_wrong_scope(IntactWrongScopeDenial::new(
                request.scope(),
                request.membership().scope(),
            )),
    )
}

fn reject_checksum_scope_mismatch(
    checked: &ScopedCheckedPhysicalForm<'_>,
    expected: &crate::ChecksumCoverageBasis,
) -> Result<(), PhysicalScopeDenial> {
    let actual = checked_checksum_scope(checked);
    if actual == expected {
        return Ok(());
    }
    Err(
        PhysicalScopeDenial::new(PhysicalScopeDenialKind::ChecksumScopeMismatch)
            .with_checksum_mismatch(ChecksumScopeMismatchDenial::new(
                expected.clone(),
                actual.clone(),
            )),
    )
}

fn reject_generation_mismatch(
    report: GenerationIntegrityReport,
    scope: PhysicalReferenceScope,
) -> Result<(), PhysicalScopeDenial> {
    match report {
        GenerationIntegrityReport::SamePhysicalGeneration { .. } => Ok(()),
        GenerationIntegrityReport::StalePhysicalGeneration { expected, actual } => Err(
            PhysicalScopeDenial::new(PhysicalScopeDenialKind::StalePhysicalGeneration)
                .with_expected_scope(scope)
                .with_owners(expected, actual)
                .with_generation_report(report),
        ),
        GenerationIntegrityReport::MisplacedPhysicalIdentity { expected, actual } => {
            let kind = misplaced_kind(expected, actual);
            Err(PhysicalScopeDenial::new(kind)
                .with_expected_scope(scope)
                .with_owners(expected, actual)
                .with_generation_report(report))
        }
    }
}

fn checked_family_accepts_scope(
    checked: &ScopedCheckedPhysicalForm<'_>,
    family: PhysicalScopeFamily,
) -> bool {
    match checked {
        ScopedCheckedPhysicalForm::Page(_) => matches!(
            family,
            PhysicalScopeFamily::Page
                | PhysicalScopeFamily::Manifest
                | PhysicalScopeFamily::DerivedIndex
        ),
        ScopedCheckedPhysicalForm::Frame(_) => matches!(
            family,
            PhysicalScopeFamily::Frame
                | PhysicalScopeFamily::WalFrame
                | PhysicalScopeFamily::ChunkLike
        ),
    }
}

fn checked_owner(checked: &ScopedCheckedPhysicalForm<'_>) -> PhysicalGenerationOwner {
    match checked {
        ScopedCheckedPhysicalForm::Page(page) => page.physical_witness().owner(),
        ScopedCheckedPhysicalForm::Frame(frame) => frame.physical_witness().owner(),
    }
}

fn checked_identity(checked: &ScopedCheckedPhysicalForm<'_>) -> crate::LogicalDecodeGateIdentity {
    match checked {
        ScopedCheckedPhysicalForm::Page(page) => page.gate_evidence().identity().clone(),
        ScopedCheckedPhysicalForm::Frame(frame) => frame.gate_evidence().identity().clone(),
    }
}

fn checked_checksum_scope<'a>(
    checked: &'a ScopedCheckedPhysicalForm<'_>,
) -> &'a crate::ChecksumCoverageBasis {
    match checked {
        ScopedCheckedPhysicalForm::Page(page) => page.gate_evidence().coverage_basis(),
        ScopedCheckedPhysicalForm::Frame(frame) => frame.gate_evidence().coverage_basis(),
    }
}

fn misplaced_kind(
    expected: PhysicalGenerationOwner,
    actual: PhysicalGenerationOwner,
) -> PhysicalScopeDenialKind {
    if expected.segment_id() != actual.segment_id() {
        PhysicalScopeDenialKind::WrongSegment
    } else if expected.extent_id() != actual.extent_id() {
        PhysicalScopeDenialKind::WrongExtent
    } else if expected.page_id() != actual.page_id() {
        PhysicalScopeDenialKind::WrongPage
    } else {
        PhysicalScopeDenialKind::MisplacedPhysicalIdentity
    }
}
