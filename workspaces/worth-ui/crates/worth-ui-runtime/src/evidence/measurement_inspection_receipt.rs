use worth_ui_inspection::{
    UiInspectionMeasurementBasisInput, UiInspectionMeasurementBasisPosture,
    UiInspectionMeasurementBasisSource, UiInspectionMeasurementChildIntrinsicSource,
    UiInspectionMeasurementDenialPosture, UiInspectionMeasurementDependencyLineageEntry,
    UiInspectionMeasurementDependencyLineageKind, UiInspectionMeasurementEvidenceCategory,
    UiInspectionMeasurementEvidenceSlot, UiInspectionMeasurementEvidenceView,
    UiInspectionMeasurementFailureSource, UiInspectionMeasurementGenerationCompatibility,
    UiInspectionMeasurementNeighborhoodClassHint, UiInspectionMeasurementOwnershipPosture,
    UiInspectionSupportPosture, UiInspectionSupportReport,
};

use crate::declaration::{UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementOwnershipPosture};

use super::{
    MeasurementEvidenceInput, UiMeasurementBasis, UiMeasurementBasisDenial,
    UiMeasurementBasisPosture, UiMeasurementDependencyLineageKind, UiMeasurementEvidenceCategory,
    UiMeasurementEvidenceSlot, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint,
};

pub(crate) fn project_measurement_inspection_view(
    support_report: UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        basis,
        basis.and_then(|basis| basis.denial_posture().map(project_denial)),
        basis.map(|basis| project_generation_compatibility(basis.generation_compatibility())),
        classify_failure_source(&support_report, basis),
    )
}

pub(crate) fn project_measurement_inspection_denial_view(
    support_report: UiInspectionSupportReport,
    denial_posture: UiInspectionMeasurementDenialPosture,
    failure_source: Option<UiInspectionMeasurementFailureSource>,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        None,
        Some(denial_posture),
        None,
        failure_source,
    )
}

pub(crate) fn project_measurement_inspection_compatibility_view(
    support_report: UiInspectionSupportReport,
    compatibility: UiInspectionMeasurementGenerationCompatibility,
) -> UiInspectionMeasurementEvidenceView {
    measurement_view_from_parts(
        support_report,
        None,
        Some(
            UiInspectionMeasurementDenialPosture::GenerationIncompatible {
                compatibility: compatibility.clone(),
            },
        ),
        Some(compatibility),
        Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch),
    )
}

fn measurement_view_from_parts(
    support_report: UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
    denial_posture: Option<UiInspectionMeasurementDenialPosture>,
    generation_compatibility: Option<UiInspectionMeasurementGenerationCompatibility>,
    failure_source: Option<UiInspectionMeasurementFailureSource>,
) -> UiInspectionMeasurementEvidenceView {
    let basis_posture = basis.map(|basis| match basis.basis_posture() {
        UiMeasurementBasisPosture::QueryOnly => UiInspectionMeasurementBasisPosture::QueryOnly,
        UiMeasurementBasisPosture::HostOnly => UiInspectionMeasurementBasisPosture::HostOnly,
        UiMeasurementBasisPosture::QueryAndHost => {
            UiInspectionMeasurementBasisPosture::QueryAndHost
        }
    });
    let basis_inputs = basis
        .map(|basis| {
            basis
                .evidence_inputs()
                .iter()
                .map(project_basis_input)
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .unwrap_or_else(|| Box::new([]));
    let dependency_lineage = basis
        .map(|basis| {
            basis
                .dependency_lineage()
                .entries()
                .iter()
                .map(|entry| {
                    UiInspectionMeasurementDependencyLineageEntry::new(
                        project_lineage_kind(entry.kind()),
                        entry.identity_digest(),
                        entry.generation_digest(),
                    )
                })
                .collect::<Vec<_>>()
                .into_boxed_slice()
        })
        .unwrap_or_else(|| Box::new([]));
    let neighborhood_class_hint =
        basis.map(|basis| project_neighborhood_class_hint(basis.neighborhood_class_hint()));

    UiInspectionMeasurementEvidenceView::new(
        support_report,
        basis_posture,
        denial_posture,
        basis_inputs,
        dependency_lineage,
        generation_compatibility,
        neighborhood_class_hint,
        failure_source,
    )
}

fn classify_failure_source(
    support_report: &UiInspectionSupportReport,
    basis: Option<&UiMeasurementBasis>,
) -> Option<UiInspectionMeasurementFailureSource> {
    if !matches!(
        support_report.posture(),
        UiInspectionSupportPosture::Supported
    ) {
        return Some(UiInspectionMeasurementFailureSource::DeclarationPosture);
    }

    let basis = basis?;
    match basis.denial_posture() {
        Some(UiMeasurementBasisDenial::GenerationIncompatible { .. }) => {
            Some(UiInspectionMeasurementFailureSource::CompatibilityMismatch)
        }
        Some(UiMeasurementBasisDenial::MissingEvidence { slot })
        | Some(UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot }) => {
            Some(classify_slot_source(*slot))
        }
        Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence { .. })
        | Some(UiMeasurementBasisDenial::MissingOwnershipEvidence { .. })
        | Some(UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence { .. }) => {
            Some(UiInspectionMeasurementFailureSource::HostEvidence)
        }
        None => (!basis.generation_compatibility().is_compatible())
            .then_some(UiInspectionMeasurementFailureSource::CompatibilityMismatch),
    }
}

fn classify_slot_source(slot: UiMeasurementEvidenceSlot) -> UiInspectionMeasurementFailureSource {
    match slot {
        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt => {
            UiInspectionMeasurementFailureSource::QueryFacts
        }
        UiMeasurementEvidenceSlot::HostCapabilityReport
        | UiMeasurementEvidenceSlot::HostTextIntrinsicSize
        | UiMeasurementEvidenceSlot::HostFontMetrics
        | UiMeasurementEvidenceSlot::HostNativeControlIntrinsicSize
        | UiMeasurementEvidenceSlot::ViewportExtent
        | UiMeasurementEvidenceSlot::PortalAnchorRect
        | UiMeasurementEvidenceSlot::ScrollContainerViewport => {
            UiInspectionMeasurementFailureSource::HostEvidence
        }
    }
}

fn project_basis_input(input: &MeasurementEvidenceInput) -> UiInspectionMeasurementBasisInput {
    match input {
        MeasurementEvidenceInput::QueryProjectionFact(receipt) => {
            UiInspectionMeasurementBasisInput::QueryProjectionFact {
                query_basis_digest: receipt.query_basis_digest().into(),
                projection_contract_digest: receipt.projection_contract_digest().into(),
                required_fact_family_set_digest: receipt.required_query_fact_family_set_digest(),
                consumed_fact_family_set_digest: receipt.consumed_fact_family_set_digest(),
            }
        }
        MeasurementEvidenceInput::HostCapabilityReport(report) => {
            UiInspectionMeasurementBasisInput::HostCapabilityReport {
                profile_digest: report.profile_identity_digest(),
                observation_generation: report.observation_generation().as_u64(),
            }
        }
        MeasurementEvidenceInput::HostMeasurementResult(result) => {
            UiInspectionMeasurementBasisInput::HostMeasurementResult {
                category: project_evidence_category(result.value().category()),
                identity_digest: input.identity_digest(),
            }
        }
        MeasurementEvidenceInput::ChildIntrinsicMeasurement(evidence) => {
            let source = if evidence.query_projection_fact().is_some() {
                UiInspectionMeasurementChildIntrinsicSource::QueryProjectionFact
            } else {
                let result = evidence
                    .host_measurement_result()
                    .expect("child intrinsic evidence must carry query or host authority");
                UiInspectionMeasurementChildIntrinsicSource::HostMeasurementResult(
                    project_evidence_category(result.value().category()),
                )
            };
            UiInspectionMeasurementBasisInput::ChildIntrinsicMeasurement {
                contributor_graph_node_identity_digest: evidence
                    .contributor_graph_node_identity()
                    .digest(),
                source,
                identity_digest: input.identity_digest(),
            }
        }
        MeasurementEvidenceInput::SiblingResizeSupport(support) => {
            UiInspectionMeasurementBasisInput::SiblingResizeSupport {
                axis_scope: match support.axis_scope() {
                    crate::evidence::UiConstraintAxisScope::Primary => "primary".into(),
                    crate::evidence::UiConstraintAxisScope::Cross => "cross".into(),
                    crate::evidence::UiConstraintAxisScope::Both => "both".into(),
                },
                target_graph_node_identity_digest: support.target_graph_node_identity().digest(),
                sizing_contract_id: support
                    .sizing_contract_id()
                    .map(|contract_id| contract_id.as_str())
                    .unwrap_or("none")
                    .into(),
                source: match support.source() {
                    crate::evidence::UiMeasurementSiblingResizeSupportSource::MosaicSizingCapabilitySnapshot => {
                        "mosaic-sizing-capability-snapshot".into()
                    }
                    crate::evidence::UiMeasurementSiblingResizeSupportSource::RuntimeDurableResizeWitness => {
                        "runtime-durable-resize-witness".into()
                    }
                },
                identity_digest: input.identity_digest(),
            }
        }
    }
}

fn project_denial(denial: &UiMeasurementBasisDenial) -> UiInspectionMeasurementDenialPosture {
    match denial {
        UiMeasurementBasisDenial::GenerationIncompatible { compatibility } => {
            UiInspectionMeasurementDenialPosture::GenerationIncompatible {
                compatibility: project_generation_compatibility(compatibility),
            }
        }
        UiMeasurementBasisDenial::MissingEvidence { slot } => {
            UiInspectionMeasurementDenialPosture::MissingEvidence {
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::MissingBasisSourceEvidence { basis_source, slot } => {
            UiInspectionMeasurementDenialPosture::MissingBasisSourceEvidence {
                basis_source: project_basis_source(*basis_source),
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::MissingOwnershipEvidence {
            ownership_posture,
            slot,
        } => UiInspectionMeasurementDenialPosture::MissingOwnershipEvidence {
            ownership_posture: project_ownership_posture(*ownership_posture),
            slot: project_slot(*slot),
        },
        UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence { category, slot } => {
            UiInspectionMeasurementDenialPosture::MissingRequiredMeasurementEvidence {
                category: project_evidence_category(*category),
                slot: project_slot(*slot),
            }
        }
        UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot } => {
            UiInspectionMeasurementDenialPosture::ConflictingEvidenceInputs {
                slot: project_slot(*slot),
            }
        }
    }
}

fn project_generation_compatibility(
    compatibility: &UiMeasurementGenerationCompatibility,
) -> UiInspectionMeasurementGenerationCompatibility {
    match compatibility {
        UiMeasurementGenerationCompatibility::Compatible => {
            UiInspectionMeasurementGenerationCompatibility::Compatible
        }
        UiMeasurementGenerationCompatibility::StaleQueryFactReceipt { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleQueryFactReceipt {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::StaleHostEvidence { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleHostEvidence {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::StaleHostCapability { expected, observed } => {
            UiInspectionMeasurementGenerationCompatibility::StaleHostCapability {
                expected: expected.as_u64(),
                observed: observed.as_u64(),
            }
        }
        UiMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest,
            observed_world_basis_digest,
        } => UiInspectionMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest: expected_query_basis_digest.clone(),
            observed_world_basis_digest: observed_world_basis_digest.clone(),
        },
        UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest,
            observed_profile_digest,
        } => UiInspectionMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest: *expected_profile_digest,
            observed_profile_digest: *observed_profile_digest,
        },
    }
}

fn project_lineage_kind(
    kind: UiMeasurementDependencyLineageKind,
) -> UiInspectionMeasurementDependencyLineageKind {
    match kind {
        UiMeasurementDependencyLineageKind::QueryScrollContentExtent => {
            UiInspectionMeasurementDependencyLineageKind::QueryScrollContentExtent
        }
        UiMeasurementDependencyLineageKind::HostTextIntrinsicSize => {
            UiInspectionMeasurementDependencyLineageKind::HostTextIntrinsicSize
        }
        UiMeasurementDependencyLineageKind::HostFontMetrics => {
            UiInspectionMeasurementDependencyLineageKind::HostFontMetrics
        }
        UiMeasurementDependencyLineageKind::HostNativeControlIntrinsicSize => {
            UiInspectionMeasurementDependencyLineageKind::HostNativeControlIntrinsicSize
        }
        UiMeasurementDependencyLineageKind::HostViewportExtent => {
            UiInspectionMeasurementDependencyLineageKind::HostViewportExtent
        }
        UiMeasurementDependencyLineageKind::HostPortalAnchorRect => {
            UiInspectionMeasurementDependencyLineageKind::HostPortalAnchorRect
        }
        UiMeasurementDependencyLineageKind::HostScrollContainerViewport => {
            UiInspectionMeasurementDependencyLineageKind::HostScrollContainerViewport
        }
    }
}

fn project_evidence_category(
    category: UiMeasurementEvidenceCategory,
) -> UiInspectionMeasurementEvidenceCategory {
    match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => {
            UiInspectionMeasurementEvidenceCategory::TextIntrinsicSize
        }
        UiMeasurementEvidenceCategory::TextBaselineMetrics => {
            UiInspectionMeasurementEvidenceCategory::TextBaselineMetrics
        }
        UiMeasurementEvidenceCategory::FontMetrics => {
            UiInspectionMeasurementEvidenceCategory::FontMetrics
        }
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            UiInspectionMeasurementEvidenceCategory::NativeControlIntrinsicSize
        }
        UiMeasurementEvidenceCategory::ViewportExtent => {
            UiInspectionMeasurementEvidenceCategory::ViewportExtent
        }
        UiMeasurementEvidenceCategory::DpiScaleFactor => {
            UiInspectionMeasurementEvidenceCategory::DpiScaleFactor
        }
        UiMeasurementEvidenceCategory::PortalAnchorRect => {
            UiInspectionMeasurementEvidenceCategory::PortalAnchorRect
        }
        UiMeasurementEvidenceCategory::ScrollContainerViewport => {
            UiInspectionMeasurementEvidenceCategory::ScrollContainerViewport
        }
    }
}

fn project_slot(slot: UiMeasurementEvidenceSlot) -> UiInspectionMeasurementEvidenceSlot {
    match slot {
        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt => {
            UiInspectionMeasurementEvidenceSlot::QueryProjectionFactReceipt
        }
        UiMeasurementEvidenceSlot::HostCapabilityReport => {
            UiInspectionMeasurementEvidenceSlot::HostCapabilityReport
        }
        UiMeasurementEvidenceSlot::HostTextIntrinsicSize => {
            UiInspectionMeasurementEvidenceSlot::HostTextIntrinsicSize
        }
        UiMeasurementEvidenceSlot::HostFontMetrics => {
            UiInspectionMeasurementEvidenceSlot::HostFontMetrics
        }
        UiMeasurementEvidenceSlot::HostNativeControlIntrinsicSize => {
            UiInspectionMeasurementEvidenceSlot::HostNativeControlIntrinsicSize
        }
        UiMeasurementEvidenceSlot::ViewportExtent => {
            UiInspectionMeasurementEvidenceSlot::ViewportExtent
        }
        UiMeasurementEvidenceSlot::PortalAnchorRect => {
            UiInspectionMeasurementEvidenceSlot::PortalAnchorRect
        }
        UiMeasurementEvidenceSlot::ScrollContainerViewport => {
            UiInspectionMeasurementEvidenceSlot::ScrollContainerViewport
        }
    }
}

fn project_basis_source(
    basis_source: UiDeclaredMeasurementBasisSource,
) -> UiInspectionMeasurementBasisSource {
    match basis_source {
        UiDeclaredMeasurementBasisSource::ScrollViewport => {
            UiInspectionMeasurementBasisSource::ScrollViewport
        }
        UiDeclaredMeasurementBasisSource::PortalAnchor => {
            UiInspectionMeasurementBasisSource::PortalAnchor
        }
    }
}

fn project_ownership_posture(
    ownership_posture: UiDeclaredMeasurementOwnershipPosture,
) -> UiInspectionMeasurementOwnershipPosture {
    match ownership_posture {
        UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis => {
            UiInspectionMeasurementOwnershipPosture::ScrollContainerBasis
        }
        UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired => {
            UiInspectionMeasurementOwnershipPosture::PortalAnchorBasisRequired
        }
    }
}

fn project_neighborhood_class_hint(
    hint: UiMeasurementNeighborhoodClassHint,
) -> UiInspectionMeasurementNeighborhoodClassHint {
    match hint {
        UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency
        }
        UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency
        }
        UiMeasurementNeighborhoodClassHint::ViewportDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ViewportDependency
        }
        UiMeasurementNeighborhoodClassHint::ScrollContainerDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::ScrollContainerDependency
        }
        UiMeasurementNeighborhoodClassHint::PortalAnchorDependency => {
            UiInspectionMeasurementNeighborhoodClassHint::PortalAnchorDependency
        }
    }
}
