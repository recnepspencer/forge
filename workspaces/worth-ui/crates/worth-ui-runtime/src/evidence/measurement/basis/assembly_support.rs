use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementOwnershipPosture};
use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use super::assembly::HostResultSlots;
use super::{denial::UiMeasurementBasisDenial, UiMeasurementEvidenceSlot};
use crate::evidence::measurement::{
    MeasurementEvidenceInput, UiChildIntrinsicMeasurementEvidence,
    UiMeasurementDependencyLineageEntry, UiMeasurementDependencyLineageKind,
    UiMeasurementEvidenceCategory, UiMeasurementGenerationCompatibility, UiMeasurementResult,
    UiProjectionFactReceipt,
};

pub(super) fn assign_slot<'a, T>(
    slot: &mut Option<&'a T>,
    value: &'a T,
    conflicting_slot: &mut Option<UiMeasurementEvidenceSlot>,
    evidence_slot: UiMeasurementEvidenceSlot,
) {
    if slot.is_none() {
        *slot = Some(value);
    } else if conflicting_slot.is_none() {
        *conflicting_slot = Some(evidence_slot);
    }
}

pub(super) fn basis_source_denial(
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    host_results: &HostResultSlots<'_>,
) -> Option<UiMeasurementBasisDenial> {
    match basis_source {
        Some(UiDeclaredMeasurementBasisSource::ViewportExtent)
            if host_results.viewport_extent.is_none() =>
        {
            Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence {
                basis_source: UiDeclaredMeasurementBasisSource::ViewportExtent,
                slot: UiMeasurementEvidenceSlot::ViewportExtent,
            })
        }
        Some(UiDeclaredMeasurementBasisSource::ScrollViewport)
            if host_results.viewport_extent.is_none() =>
        {
            Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence {
                basis_source: UiDeclaredMeasurementBasisSource::ScrollViewport,
                slot: UiMeasurementEvidenceSlot::ViewportExtent,
            })
        }
        Some(UiDeclaredMeasurementBasisSource::PortalAnchor)
            if host_results.portal_anchor_rect.is_none() =>
        {
            Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence {
                basis_source: UiDeclaredMeasurementBasisSource::PortalAnchor,
                slot: UiMeasurementEvidenceSlot::PortalAnchorRect,
            })
        }
        _ => None,
    }
}

pub(super) fn ownership_posture_denial(
    ownership_posture: Option<UiDeclaredMeasurementOwnershipPosture>,
    host_results: &HostResultSlots<'_>,
) -> Option<UiMeasurementBasisDenial> {
    match ownership_posture {
        Some(UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis)
            if host_results.scroll_container_viewport.is_none() =>
        {
            Some(UiMeasurementBasisDenial::MissingOwnershipEvidence {
                ownership_posture: UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis,
                slot: UiMeasurementEvidenceSlot::ScrollContainerViewport,
            })
        }
        Some(UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired)
            if host_results.portal_anchor_rect.is_none() =>
        {
            Some(UiMeasurementBasisDenial::MissingOwnershipEvidence {
                ownership_posture: UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired,
                slot: UiMeasurementEvidenceSlot::PortalAnchorRect,
            })
        }
        _ => None,
    }
}

pub(super) fn host_result_compatibility(
    result: Option<&UiMeasurementResult>,
    report: &WorthUiHostCapabilityReport,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
) -> Option<UiMeasurementGenerationCompatibility> {
    let Some(result) = result else {
        return None;
    };
    if result.evidence_generation() != declaration_support_authority_generation {
        return Some(UiMeasurementGenerationCompatibility::StaleHostEvidence {
            expected: declaration_support_authority_generation,
            observed: result.evidence_generation(),
        });
    }
    if result
        .assumption_profile()
        .capability_observation_generation()
        != report.observation_generation()
    {
        return Some(UiMeasurementGenerationCompatibility::StaleHostCapability {
            expected: report.observation_generation(),
            observed: result
                .assumption_profile()
                .capability_observation_generation(),
        });
    }
    if result.assumption_profile().capability_profile_digest() != report.profile_identity_digest() {
        return Some(
            UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
                expected_profile_digest: report.profile_identity_digest(),
                observed_profile_digest: result.assumption_profile().capability_profile_digest(),
            },
        );
    }
    None
}

pub(super) fn query_receipt_compatibility(
    receipt: &UiProjectionFactReceipt,
    world_profile: &crate::graph::UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
) -> Option<UiMeasurementGenerationCompatibility> {
    if receipt.declaration_support_authority_generation()
        != declaration_support_authority_generation
    {
        return Some(
            UiMeasurementGenerationCompatibility::StaleQueryFactReceipt {
                expected: declaration_support_authority_generation,
                observed: receipt.declaration_support_authority_generation(),
            },
        );
    }

    if let crate::graph::UiGraphWorldProfile::InstalledQueryBasis { authority } = world_profile {
        if authority.query_authority() != receipt.query_authority() {
            return Some(UiMeasurementGenerationCompatibility::IncompatibleWorld {
                expected_query_basis_digest: receipt.query_basis_digest().into(),
                observed_world_basis_digest: None,
            });
        }
        return None;
    }

    let observed_world_basis_digest = match world_profile {
        crate::graph::UiGraphWorldProfile::QuerySnapshotBasis { prerequisites } => {
            Some(prerequisites.basis_digest_for_diagnostics())
        }
        _ => None,
    };
    if observed_world_basis_digest != Some(receipt.query_basis_digest()) {
        return Some(UiMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest: receipt.query_basis_digest().into(),
            observed_world_basis_digest: observed_world_basis_digest.map(Into::into),
        });
    }

    None
}

pub(super) fn push_host_lineage(
    entries: &mut Vec<UiMeasurementDependencyLineageEntry>,
    result: Option<&UiMeasurementResult>,
    kind: UiMeasurementDependencyLineageKind,
) {
    if let Some(result) = result {
        entries.push(UiMeasurementDependencyLineageEntry::new(
            kind,
            MeasurementEvidenceInput::host_measurement_result(result).identity_digest(),
            result.evidence_generation().as_u64(),
        ));
    }
}

pub(super) fn child_intrinsic_query_compatibility(
    evidence: &UiChildIntrinsicMeasurementEvidence,
    world_profile: &crate::graph::UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
) -> Option<UiMeasurementGenerationCompatibility> {
    evidence.query_projection_fact().and_then(|receipt| {
        query_receipt_compatibility(
            receipt,
            world_profile,
            declaration_support_authority_generation,
        )
    })
}

pub(super) fn child_intrinsic_host_compatibility(
    evidence: &UiChildIntrinsicMeasurementEvidence,
    report: &WorthUiHostCapabilityReport,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
) -> Option<UiMeasurementGenerationCompatibility> {
    host_result_compatibility(
        evidence.host_measurement_result(),
        report,
        declaration_support_authority_generation,
    )
}

pub(super) fn push_child_intrinsic_lineage(
    entries: &mut Vec<UiMeasurementDependencyLineageEntry>,
    evidence: &UiChildIntrinsicMeasurementEvidence,
) {
    if let Some(receipt) = evidence.query_projection_fact() {
        entries.push(UiMeasurementDependencyLineageEntry::new(
            UiMeasurementDependencyLineageKind::QueryScrollContentExtent,
            evidence.identity_digest(),
            receipt.declaration_support_authority_generation().as_u64(),
        ));
    }
    if let Some(result) = evidence.host_measurement_result() {
        let kind = match result.evidence_category() {
            UiMeasurementEvidenceCategory::TextIntrinsicSize => {
                UiMeasurementDependencyLineageKind::HostTextIntrinsicSize
            }
            UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
                UiMeasurementDependencyLineageKind::HostNativeControlIntrinsicSize
            }
            _ => return,
        };
        entries.push(UiMeasurementDependencyLineageEntry::new(
            kind,
            evidence.identity_digest(),
            result.evidence_generation().as_u64(),
        ));
    }
}

impl<'a> HostResultSlots<'a> {
    pub(super) fn relevant_results(self) -> [Option<&'a UiMeasurementResult>; 6] {
        [
            self.text_intrinsic_size,
            self.font_metrics,
            self.native_control_intrinsic_size,
            self.viewport_extent,
            self.portal_anchor_rect,
            self.scroll_container_viewport,
        ]
    }

    pub(super) fn has_intrinsic_results(self) -> bool {
        self.text_intrinsic_size.is_some() || self.native_control_intrinsic_size.is_some()
    }
}
