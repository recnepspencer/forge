use worth_ui_host_contract::WorthUiHostCapabilityReport;
use worth_ui_inspection::UiEvidenceAuthorityGeneration;

use crate::declaration::{
    stable_text_digest, UiDeclarationIdentity, UiDeclaredMeasurementBasisRequirementSet,
    UiDeclaredMeasurementBasisSource, UiDeclaredMeasurementEvidenceRequirement,
    UiDeclaredMeasurementOwnershipPosture, UiDeclaredMeasurementPolicyPosture,
};
use crate::graph::{UiGraphNodeIdentity, UiGraphWorldProfile};

use super::{
    denial::UiMeasurementBasisDenial, UiMeasurementBasisGeneration, UiMeasurementEvidenceSlot,
};
use crate::evidence::measurement::{
    MeasurementEvidenceInput, UiMeasurementDependencyLineage, UiMeasurementDependencyMap,
    UiMeasurementEvidenceCategory, UiMeasurementGenerationCompatibility,
    UiMeasurementNeighborhoodClassHint, UiMeasurementResult, UiProjectionFactReceipt,
};

pub(super) fn basis_generation(
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    query_receipt: Option<&UiProjectionFactReceipt>,
    host_capability_report: Option<&WorthUiHostCapabilityReport>,
    host_results: [Option<&UiMeasurementResult>; 6],
) -> UiMeasurementBasisGeneration {
    UiMeasurementBasisGeneration::new(
        declaration_support_authority_generation.as_u64()
            ^ query_receipt
                .map(|receipt| {
                    receipt
                        .declaration_support_authority_generation()
                        .as_u64()
                        .rotate_left(7)
                })
                .unwrap_or_default()
            ^ host_capability_report
                .map(|report| report.observation_generation().as_u64().rotate_left(13))
                .unwrap_or_default()
            ^ host_results[0]
                .map(|result| result.evidence_generation().as_u64().rotate_left(17))
                .unwrap_or_default()
            ^ host_results[1]
                .map(|result| result.evidence_generation().as_u64().rotate_left(19))
                .unwrap_or_default()
            ^ host_results[2]
                .map(|result| result.evidence_generation().as_u64().rotate_left(23))
                .unwrap_or_default()
            ^ host_results[3]
                .map(|result| result.evidence_generation().as_u64().rotate_left(29))
                .unwrap_or_default()
            ^ host_results[4]
                .map(|result| result.evidence_generation().as_u64().rotate_left(31))
                .unwrap_or_default()
            ^ host_results[5]
                .map(|result| result.evidence_generation().as_u64().rotate_left(37))
                .unwrap_or_default(),
    )
}

pub(super) fn basis_identity_digest(
    requirements: &UiDeclaredMeasurementBasisRequirementSet,
    declaration_identity: &UiDeclarationIdentity,
    graph_node_identity: UiGraphNodeIdentity,
    world_profile: &UiGraphWorldProfile,
    declaration_support_authority_generation: UiEvidenceAuthorityGeneration,
    declared_measurement_policy: &UiDeclaredMeasurementPolicyPosture,
    evidence_inputs: &[MeasurementEvidenceInput],
    dependency_lineage: &UiMeasurementDependencyLineage,
    dependency_map: &UiMeasurementDependencyMap,
    neighborhood_class_hint: UiMeasurementNeighborhoodClassHint,
    generation_compatibility: &UiMeasurementGenerationCompatibility,
    denial_posture: Option<&UiMeasurementBasisDenial>,
) -> u64 {
    let policy_digest = requirements
        .required_measurement_dependencies()
        .iter()
        .fold(
            stable_text_digest("worth-ui-measurement-policy"),
            |digest, requirement| {
                digest
                    ^ stable_text_digest(match requirement {
                        UiDeclaredMeasurementEvidenceRequirement::HostFontMetrics => {
                            "host-font-metrics"
                        }
                        UiDeclaredMeasurementEvidenceRequirement::ScrollContentExtent => {
                            "scroll-content-extent"
                        }
                        UiDeclaredMeasurementEvidenceRequirement::PortalAnchorMetrics => {
                            "portal-anchor-metrics"
                        }
                    })
                    .rotate_left(7)
            },
        )
        ^ declared_measurement_policy
            .mode()
            .map(|_| stable_text_digest("mode:hug-height").rotate_left(11))
            .unwrap_or_default()
        ^ declared_measurement_policy
            .constraint_modifier()
            .map(|_| stable_text_digest("constraint:bounded").rotate_left(13))
            .unwrap_or_default()
        ^ declared_measurement_policy
            .basis_source()
            .map(|basis_source| {
                stable_text_digest(match basis_source {
                    UiDeclaredMeasurementBasisSource::ViewportExtent => {
                        "basis-source:viewport-extent"
                    }
                    UiDeclaredMeasurementBasisSource::ScrollViewport => {
                        "basis-source:scroll-viewport"
                    }
                    UiDeclaredMeasurementBasisSource::PortalAnchor => "basis-source:portal-anchor",
                })
                .rotate_left(17)
            })
            .unwrap_or_default()
        ^ declared_measurement_policy
            .ownership_posture()
            .map(|ownership_posture| {
                stable_text_digest(match ownership_posture {
                    UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis => {
                        "ownership:scroll-container-basis"
                    }
                    UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired => {
                        "ownership:portal-anchor-basis-required"
                    }
                })
                .rotate_left(19)
            })
            .unwrap_or_default();

    evidence_inputs.iter().fold(
        stable_text_digest("worth-ui-measurement-basis")
            ^ declaration_identity.digest().raw().rotate_left(7)
            ^ graph_node_identity.digest().rotate_left(13)
            ^ world_profile.identity_digest().rotate_left(17)
            ^ declaration_support_authority_generation
                .as_u64()
                .rotate_left(19)
            ^ policy_digest.rotate_left(23)
            ^ dependency_lineage.identity_digest().rotate_left(29)
            ^ dependency_map.identity_digest().rotate_left(27)
            ^ compatibility_digest(generation_compatibility).rotate_left(33)
            ^ denial_digest(denial_posture).rotate_left(35)
            ^ stable_text_digest(match neighborhood_class_hint {
                UiMeasurementNeighborhoodClassHint::LocalIntrinsicContentDependency => {
                    "local-intrinsic-content"
                }
                UiMeasurementNeighborhoodClassHint::ContainerAvailableSpaceDependency => {
                    "container-available-space"
                }
                UiMeasurementNeighborhoodClassHint::ViewportDependency => "viewport",
                UiMeasurementNeighborhoodClassHint::ScrollContainerDependency => "scroll-container",
                UiMeasurementNeighborhoodClassHint::PortalAnchorDependency => "portal-anchor",
            })
            .rotate_left(31),
        |digest, input| digest ^ input.identity_digest().rotate_left(37),
    )
}

fn compatibility_digest(compatibility: &UiMeasurementGenerationCompatibility) -> u64 {
    match compatibility {
        UiMeasurementGenerationCompatibility::Compatible => {
            stable_text_digest("compatibility:compatible")
        }
        UiMeasurementGenerationCompatibility::StaleQueryFactReceipt { expected, observed } => {
            stable_text_digest("compatibility:stale-query-fact-receipt")
                ^ expected.as_u64().rotate_left(7)
                ^ observed.as_u64().rotate_left(13)
        }
        UiMeasurementGenerationCompatibility::StaleHostEvidence { expected, observed } => {
            stable_text_digest("compatibility:stale-host-evidence")
                ^ expected.as_u64().rotate_left(7)
                ^ observed.as_u64().rotate_left(13)
        }
        UiMeasurementGenerationCompatibility::StaleHostCapability { expected, observed } => {
            stable_text_digest("compatibility:stale-host-capability")
                ^ expected.as_u64().rotate_left(7)
                ^ observed.as_u64().rotate_left(13)
        }
        UiMeasurementGenerationCompatibility::IncompatibleWorld {
            expected_query_basis_digest,
            observed_world_basis_digest,
        } => {
            stable_text_digest("compatibility:incompatible-world")
                ^ stable_text_digest(expected_query_basis_digest).rotate_left(7)
                ^ observed_world_basis_digest
                    .as_ref()
                    .map(|digest| stable_text_digest(digest).rotate_left(13))
                    .unwrap_or_default()
        }
        UiMeasurementGenerationCompatibility::IncompatibleHostProfile {
            expected_profile_digest,
            observed_profile_digest,
        } => {
            stable_text_digest("compatibility:incompatible-host-profile")
                ^ expected_profile_digest.rotate_left(7)
                ^ observed_profile_digest.rotate_left(13)
        }
    }
}

fn denial_digest(denial_posture: Option<&UiMeasurementBasisDenial>) -> u64 {
    match denial_posture {
        None => stable_text_digest("denial:none"),
        Some(UiMeasurementBasisDenial::GenerationIncompatible { compatibility }) => {
            stable_text_digest("denial:generation-incompatible")
                ^ compatibility_digest(compatibility).rotate_left(7)
        }
        Some(UiMeasurementBasisDenial::MissingEvidence { slot }) => {
            stable_text_digest("denial:missing-evidence") ^ slot_digest(*slot).rotate_left(7)
        }
        Some(UiMeasurementBasisDenial::MissingBasisSourceEvidence { basis_source, slot }) => {
            stable_text_digest("denial:missing-basis-source-evidence")
                ^ stable_text_digest(match basis_source {
                    UiDeclaredMeasurementBasisSource::ViewportExtent => "viewport-extent",
                    UiDeclaredMeasurementBasisSource::ScrollViewport => "scroll-viewport",
                    UiDeclaredMeasurementBasisSource::PortalAnchor => "portal-anchor",
                })
                .rotate_left(7)
                ^ slot_digest(*slot).rotate_left(13)
        }
        Some(UiMeasurementBasisDenial::MissingOwnershipEvidence {
            ownership_posture,
            slot,
        }) => {
            stable_text_digest("denial:missing-ownership-evidence")
                ^ stable_text_digest(match ownership_posture {
                    UiDeclaredMeasurementOwnershipPosture::ScrollContainerBasis => {
                        "scroll-container-basis"
                    }
                    UiDeclaredMeasurementOwnershipPosture::PortalAnchorBasisRequired => {
                        "portal-anchor-basis-required"
                    }
                })
                .rotate_left(7)
                ^ slot_digest(*slot).rotate_left(13)
        }
        Some(UiMeasurementBasisDenial::MissingRequiredMeasurementEvidence { category, slot }) => {
            stable_text_digest("denial:missing-required-measurement-evidence")
                ^ measurement_category_digest(*category).rotate_left(7)
                ^ slot_digest(*slot).rotate_left(13)
        }
        Some(UiMeasurementBasisDenial::ConflictingEvidenceInputs { slot }) => {
            stable_text_digest("denial:conflicting-evidence-inputs")
                ^ slot_digest(*slot).rotate_left(7)
        }
    }
}

fn slot_digest(slot: UiMeasurementEvidenceSlot) -> u64 {
    stable_text_digest(match slot {
        UiMeasurementEvidenceSlot::QueryProjectionFactReceipt => "query-projection-fact-receipt",
        UiMeasurementEvidenceSlot::HostCapabilityReport => "host-capability-report",
        UiMeasurementEvidenceSlot::HostTextIntrinsicSize => "host-text-intrinsic-size",
        UiMeasurementEvidenceSlot::HostFontMetrics => "host-font-metrics",
        UiMeasurementEvidenceSlot::HostNativeControlIntrinsicSize => {
            "host-native-control-intrinsic-size"
        }
        UiMeasurementEvidenceSlot::ViewportExtent => "viewport-extent",
        UiMeasurementEvidenceSlot::PortalAnchorRect => "portal-anchor-rect",
        UiMeasurementEvidenceSlot::ScrollContainerViewport => "scroll-container-viewport",
    })
}

fn measurement_category_digest(category: UiMeasurementEvidenceCategory) -> u64 {
    stable_text_digest(match category {
        UiMeasurementEvidenceCategory::TextIntrinsicSize => "text-intrinsic-size",
        UiMeasurementEvidenceCategory::TextBaselineMetrics => "text-baseline-metrics",
        UiMeasurementEvidenceCategory::FontMetrics => "font-metrics",
        UiMeasurementEvidenceCategory::NativeControlIntrinsicSize => {
            "native-control-intrinsic-size"
        }
        UiMeasurementEvidenceCategory::ViewportExtent => "viewport-extent",
        UiMeasurementEvidenceCategory::DpiScaleFactor => "dpi-scale-factor",
        UiMeasurementEvidenceCategory::PortalAnchorRect => "portal-anchor-rect",
        UiMeasurementEvidenceCategory::ScrollContainerViewport => "scroll-container-viewport",
    })
}
