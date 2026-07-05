use super::{
    MeasurementEvidenceInput, UiMeasurementBasisDenial, UiMeasurementBasisPosture,
    UiMeasurementDependencyLineage, UiMeasurementDependencyLineageEntry,
    UiMeasurementDependencyLineageKind, UiMeasurementEvidenceCategory, UiMeasurementEvidenceSlot,
    UiMeasurementGenerationCompatibility, UiMeasurementResult, UiProjectionFactReceipt,
};
use crate::declaration::{
    UiDeclaredMeasurementBasisRequirementSet, UiDeclaredMeasurementBasisSource,
    UiDeclaredMeasurementOwnershipPosture,
};
use crate::graph::UiGraphWorldProfile;
use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

pub(super) struct SelectedEvidence<'a> {
    pub query_receipt: Option<&'a UiProjectionFactReceipt>,
    pub host_capability_report: Option<&'a WorthUiHostCapabilityReport>,
    pub host_results: HostResultSlots<'a>,
    conflicting_slot: Option<UiMeasurementEvidenceSlot>,
}

#[derive(Clone, Copy, Default)]
pub(super) struct HostResultSlots<'a> {
    pub font_metrics: Option<&'a UiMeasurementResult>,
    pub viewport_extent: Option<&'a UiMeasurementResult>,
    pub portal_anchor_rect: Option<&'a UiMeasurementResult>,
    pub scroll_container_viewport: Option<&'a UiMeasurementResult>,
}
impl<'a> SelectedEvidence<'a> {
    pub(super) fn from_inputs(
        requirements: &UiDeclaredMeasurementBasisRequirementSet,
        evidence_inputs: &'a [MeasurementEvidenceInput],
    ) -> Self {
        let mut selected = Self {
            query_receipt: None,
            host_capability_report: None,
            host_results: HostResultSlots::default(),
            conflicting_slot: None,
        };
        for input in evidence_inputs {
            if let Some(receipt) = input.as_query_projection_fact() {
                if requirements.requires_query_projection_receipt() {
                    assign_slot(
                        &mut selected.query_receipt,
                        receipt,
                        &mut selected.conflicting_slot,
                        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt,
                    );
                }
                continue;
            }

            if let Some(report) = input.as_host_capability_report() {
                if requirements.requires_host_measurement_evidence() {
                    assign_slot(
                        &mut selected.host_capability_report,
                        report,
                        &mut selected.conflicting_slot,
                        UiMeasurementEvidenceSlot::HostCapabilityReport,
                    );
                }
                continue;
            }

            if let Some(result) = input.as_host_measurement_result() {
                match result.evidence_category() {
                    UiMeasurementEvidenceCategory::FontMetrics
                        if requirements.requires_host_font_metrics() =>
                    {
                        assign_slot(
                            &mut selected.host_results.font_metrics,
                            result,
                            &mut selected.conflicting_slot,
                            UiMeasurementEvidenceSlot::HostFontMetrics,
                        );
                    }
                    UiMeasurementEvidenceCategory::ViewportExtent
                        if requirements.requires_viewport_extent() =>
                    {
                        assign_slot(
                            &mut selected.host_results.viewport_extent,
                            result,
                            &mut selected.conflicting_slot,
                            UiMeasurementEvidenceSlot::ViewportExtent,
                        );
                    }
                    UiMeasurementEvidenceCategory::PortalAnchorRect
                        if requirements.requires_portal_anchor_metrics() =>
                    {
                        assign_slot(
                            &mut selected.host_results.portal_anchor_rect,
                            result,
                            &mut selected.conflicting_slot,
                            UiMeasurementEvidenceSlot::PortalAnchorRect,
                        );
                    }
                    UiMeasurementEvidenceCategory::ScrollContainerViewport
                        if requirements.requires_scroll_container_viewport() =>
                    {
                        assign_slot(
                            &mut selected.host_results.scroll_container_viewport,
                            result,
                            &mut selected.conflicting_slot,
                            UiMeasurementEvidenceSlot::ScrollContainerViewport,
                        );
                    }
                    _ => {}
                }
            }
        }
        selected
    }

    pub(super) fn generation_compatibility(
        &self,
        world_profile: &UiGraphWorldProfile,
        declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    ) -> UiMeasurementGenerationCompatibility {
        if let Some(receipt) = self.query_receipt {
            if receipt.declaration_support_authority_generation()
                != declaration_support_authority_generation
            {
                return UiMeasurementGenerationCompatibility::StaleQueryFactReceipt {
                    expected: declaration_support_authority_generation,
                    observed: receipt.declaration_support_authority_generation(),
                };
            }

            let observed_world_basis_digest = match world_profile {
                UiGraphWorldProfile::QuerySnapshotBasis {
                    resolution_report, ..
                } => Some(resolution_report.basis_digest().as_str()),
                _ => None,
            };
            if observed_world_basis_digest != Some(receipt.query_basis_digest()) {
                return UiMeasurementGenerationCompatibility::IncompatibleWorld {
                    expected_query_basis_digest: receipt.query_basis_digest().into(),
                    observed_world_basis_digest: observed_world_basis_digest.map(Into::into),
                };
            }
        }

        if let Some(report) = self.host_capability_report {
            if let Some(compatibility) = host_result_compatibility(
                self.host_results.font_metrics,
                report,
                declaration_support_authority_generation,
            ) {
                return compatibility;
            }
            if let Some(compatibility) = host_result_compatibility(
                self.host_results.viewport_extent,
                report,
                declaration_support_authority_generation,
            ) {
                return compatibility;
            }
            if let Some(compatibility) = host_result_compatibility(
                self.host_results.portal_anchor_rect,
                report,
                declaration_support_authority_generation,
            ) {
                return compatibility;
            }
            if let Some(compatibility) = host_result_compatibility(
                self.host_results.scroll_container_viewport,
                report,
                declaration_support_authority_generation,
            ) {
                return compatibility;
            }
        }
        UiMeasurementGenerationCompatibility::Compatible
    }

    pub(super) fn admitted_inputs(&self) -> Box<[MeasurementEvidenceInput]> {
        let mut inputs = Vec::new();
        if let Some(receipt) = self.query_receipt {
            inputs.push(MeasurementEvidenceInput::query_projection_fact(receipt));
        }
        if let Some(report) = self.host_capability_report {
            inputs.push(MeasurementEvidenceInput::host_capability_report(report));
        }
        for result in self.host_results.relevant_results().into_iter().flatten() {
            inputs.push(MeasurementEvidenceInput::host_measurement_result(result));
        }
        inputs.into_boxed_slice()
    }

    pub(super) fn dependency_lineage(&self) -> UiMeasurementDependencyLineage {
        let mut entries = Vec::new();
        if let Some(receipt) = self.query_receipt {
            entries.push(UiMeasurementDependencyLineageEntry::new(
                UiMeasurementDependencyLineageKind::QueryScrollContentExtent,
                receipt.consumed_fact_family_set_digest(),
                receipt.declaration_support_authority_generation().as_u64(),
            ));
        }
        push_host_lineage(
            &mut entries,
            self.host_results.font_metrics,
            UiMeasurementDependencyLineageKind::HostFontMetrics,
        );
        push_host_lineage(
            &mut entries,
            self.host_results.viewport_extent,
            UiMeasurementDependencyLineageKind::HostViewportExtent,
        );
        push_host_lineage(
            &mut entries,
            self.host_results.portal_anchor_rect,
            UiMeasurementDependencyLineageKind::HostPortalAnchorRect,
        );
        push_host_lineage(
            &mut entries,
            self.host_results.scroll_container_viewport,
            UiMeasurementDependencyLineageKind::HostScrollContainerViewport,
        );
        UiMeasurementDependencyLineage::new(entries)
    }

    pub(super) fn basis_posture(&self) -> UiMeasurementBasisPosture {
        match (
            self.query_receipt.is_some(),
            self.host_capability_report.is_some(),
        ) {
            (true, true) => UiMeasurementBasisPosture::QueryAndHost,
            (true, false) => UiMeasurementBasisPosture::QueryOnly,
            (false, true) | (false, false) => UiMeasurementBasisPosture::HostOnly,
        }
    }

    pub(super) fn denial_posture(
        &self,
        requirements: &UiDeclaredMeasurementBasisRequirementSet,
        generation_compatibility: &UiMeasurementGenerationCompatibility,
    ) -> Option<UiMeasurementBasisDenial> {
        if !generation_compatibility.is_compatible() {
            return Some(UiMeasurementBasisDenial::GenerationIncompatible {
                compatibility: generation_compatibility.clone(),
            });
        }
        if let Some(slot) = self.conflicting_slot {
            return Some(UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot });
        }
        if requirements.requires_query_projection_receipt() && self.query_receipt.is_none() {
            return Some(UiMeasurementBasisDenial::MissingEvidence {
                slot: UiMeasurementEvidenceSlot::QueryProjectionFactReceipt,
            });
        }
        if requirements.requires_host_measurement_evidence()
            && self.host_capability_report.is_none()
        {
            return Some(UiMeasurementBasisDenial::MissingEvidence {
                slot: UiMeasurementEvidenceSlot::HostCapabilityReport,
            });
        }
        if requirements.requires_host_font_metrics() && self.host_results.font_metrics.is_none() {
            return Some(
                UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence {
                    category: UiMeasurementEvidenceCategory::FontMetrics,
                    slot: UiMeasurementEvidenceSlot::HostFontMetrics,
                },
            );
        }
        if let Some(denial) = basis_source_denial(requirements.basis_source(), &self.host_results) {
            return Some(denial);
        }
        if let Some(denial) =
            ownership_posture_denial(requirements.ownership_posture(), &self.host_results)
        {
            return Some(denial);
        }
        if requirements.requires_portal_anchor_metrics()
            && self.host_results.portal_anchor_rect.is_none()
        {
            return Some(
                UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence {
                    category: UiMeasurementEvidenceCategory::PortalAnchorRect,
                    slot: UiMeasurementEvidenceSlot::PortalAnchorRect,
                },
            );
        }
        None
    }
}

impl<'a> HostResultSlots<'a> {
    fn relevant_results(self) -> [Option<&'a UiMeasurementResult>; 4] {
        [
            self.font_metrics,
            self.viewport_extent,
            self.portal_anchor_rect,
            self.scroll_container_viewport,
        ]
    }
}

fn assign_slot<'a, T>(
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

fn basis_source_denial(
    basis_source: Option<UiDeclaredMeasurementBasisSource>,
    host_results: &HostResultSlots<'_>,
) -> Option<UiMeasurementBasisDenial> {
    match basis_source {
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

fn ownership_posture_denial(
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

fn host_result_compatibility(
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

fn push_host_lineage(
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
