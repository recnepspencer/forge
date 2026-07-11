use crate::containers::container_integrity_slot_directory::inspect_record_slot_directory;
use crate::{
    ContainerIntegrityCounters, ExtentIntegrityReport, FrameIntegrityReport, IntegrityCheckedFrame,
    IntegrityCheckedPage, PageIntegrityReport, PhysicalBoundaryLocalization,
    PhysicalContainerIntegrityDenial, PhysicalContainerIntegrityDenialKind,
    ScopedPhysicalValidatorInput, SlotDirectoryIntegrityReport, TornFrameDenial,
};
use forge_store_physical_format::{PhysicalHeaderKind, PhysicalScopeFamily};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub struct PhysicalContainerIntegrity;

impl PhysicalContainerIntegrity {
    pub fn inspect_page(
        input: ScopedPhysicalValidatorInput<'_>,
    ) -> Result<PageIntegrityReport, PhysicalContainerIntegrityDenial> {
        reject_unexpected_family(input.family(), PhysicalScopeFamily::Page)?;
        let admission = input.admission();
        let Some(page) = admission.checked_page() else {
            return Err(wrong_family_denial());
        };
        let mut counters = ContainerIntegrityCounters::start().with_header_witness_check();
        reject_non_page_witness(page, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        counters = counters.with_body_boundary_check();
        reject_body_length_mismatch(page.checked_bytes().len_bytes(), page, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        let (slot_directory, counters) =
            inspect_record_slot_directory(page.checked_bytes().as_bytes(), counters)
                .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        Ok(PageIntegrityReport::new(
            admission.basis().clone(),
            counters,
            slot_directory,
        ))
    }

    pub fn inspect_frame(
        input: ScopedPhysicalValidatorInput<'_>,
    ) -> Result<FrameIntegrityReport, PhysicalContainerIntegrityDenial> {
        reject_unexpected_family(input.family(), PhysicalScopeFamily::Frame)?;
        let admission = input.admission();
        let Some(frame) = admission.checked_frame() else {
            return Err(wrong_family_denial());
        };
        let counters = ContainerIntegrityCounters::start()
            .with_header_witness_check()
            .with_frame_boundary_check();
        reject_non_frame_witness(frame, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        reject_frame_payload_length_mismatch(frame.checked_bytes().len_bytes(), frame, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        Ok(FrameIntegrityReport::new(
            admission.basis().clone(),
            counters,
            PhysicalBoundaryLocalization::FrameBody,
        ))
    }

    pub fn inspect_extent(
        input: ScopedPhysicalValidatorInput<'_>,
    ) -> Result<ExtentIntegrityReport, PhysicalContainerIntegrityDenial> {
        reject_unexpected_family(input.family(), PhysicalScopeFamily::ChunkLike)?;
        let admission = input.admission();
        let Some(frame) = admission.checked_frame() else {
            return Err(wrong_family_denial());
        };
        let counters = ContainerIntegrityCounters::start()
            .with_header_witness_check()
            .with_extent_boundary_check()
            .with_frame_boundary_check();
        reject_non_frame_witness(frame, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        reject_frame_payload_length_mismatch(frame.checked_bytes().len_bytes(), frame, counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        let frame_report = FrameIntegrityReport::new(
            admission.basis().clone(),
            counters,
            PhysicalBoundaryLocalization::FrameBody,
        );
        Ok(ExtentIntegrityReport::new(
            admission.basis().clone(),
            counters,
            frame_report,
        ))
    }

    pub fn inspect_slot_directory(
        input: ScopedPhysicalValidatorInput<'_>,
    ) -> Result<SlotDirectoryIntegrityReport, PhysicalContainerIntegrityDenial> {
        reject_unexpected_family(input.family(), PhysicalScopeFamily::Page)?;
        let admission = input.admission();
        let Some(page) = admission.checked_page() else {
            return Err(wrong_family_denial());
        };
        let counters = ContainerIntegrityCounters::start().with_slot_directory_read();
        let (report, _) = inspect_record_slot_directory(page.checked_bytes().as_bytes(), counters)
            .map_err(|denial| denial.with_basis(admission.basis().clone()))?;
        Ok(report)
    }
}

fn reject_non_page_witness(
    page: &IntegrityCheckedPage<'_>,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    if matches!(page.physical_witness().kind(), PhysicalHeaderKind::Page(_)) {
        return Ok(());
    }
    Err(PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
        PhysicalBoundaryLocalization::PageHeader,
        counters,
    ))
}

fn reject_non_frame_witness(
    frame: &IntegrityCheckedFrame<'_>,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    if matches!(
        frame.physical_witness().kind(),
        PhysicalHeaderKind::Frame(_)
    ) {
        return Ok(());
    }
    Err(PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::HeaderWitnessMismatch,
        PhysicalBoundaryLocalization::FrameHeader,
        counters,
    ))
}

fn reject_body_length_mismatch(
    actual_len: usize,
    page: &IntegrityCheckedPage<'_>,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    let expected_len = page.physical_witness().payload_length() as usize;
    if actual_len == expected_len {
        return Ok(());
    }
    Err(PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::BodyLengthMismatch,
        PhysicalBoundaryLocalization::LengthField,
        counters,
    )
    .with_lengths(expected_len, actual_len))
}

fn reject_frame_payload_length_mismatch(
    actual_len: usize,
    frame: &IntegrityCheckedFrame<'_>,
    counters: ContainerIntegrityCounters,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    let expected_len = frame.physical_witness().payload_length() as usize;
    if actual_len == expected_len {
        return Ok(());
    }
    let kind = if actual_len < expected_len {
        PhysicalContainerIntegrityDenialKind::TornFrame
    } else {
        PhysicalContainerIntegrityDenialKind::MalformedFrame
    };
    Err(PhysicalContainerIntegrityDenial::new(
        kind,
        PhysicalBoundaryLocalization::FrameBody,
        counters,
    )
    .with_lengths(expected_len, actual_len)
    .with_torn_frame(TornFrameDenial::new(expected_len, actual_len)))
}

fn reject_unexpected_family(
    actual: PhysicalScopeFamily,
    expected: PhysicalScopeFamily,
) -> Result<(), PhysicalContainerIntegrityDenial> {
    if actual == expected {
        return Ok(());
    }
    Err(wrong_family_denial())
}

fn wrong_family_denial() -> PhysicalContainerIntegrityDenial {
    PhysicalContainerIntegrityDenial::new(
        PhysicalContainerIntegrityDenialKind::WrongPhysicalFamily,
        PhysicalBoundaryLocalization::AmbiguousBoundary,
        ContainerIntegrityCounters::start(),
    )
}
