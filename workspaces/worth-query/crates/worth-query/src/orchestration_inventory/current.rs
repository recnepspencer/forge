use super::aspect::WorthQueryOrchestrationAspectPosture;
use super::authority::{
    WorthQueryOrchestrationBasisPosture, WorthQueryOrchestrationCollaborativeExtensionPosture,
    WorthQueryOrchestrationLowerAuthorityAttachment, WorthQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::WorthQueryOrchestrationContributionCompatibility;
use super::current_helpers::worth_query_family_helper_orchestration_rows;
use super::current_recovery::worth_query_recovery_orchestration_rows;
use super::current_support::{push_four_lane_rows, FourLaneSpec, RowSpec};
use super::family::{
    WorthQueryOrchestrationBindingProjection, WorthQueryOrchestrationCheckedTopologyKind,
    WorthQueryOrchestrationSupportSurface, WorthQueryOrchestrationSurfaceFamily,
    WorthQueryOrchestrationSurfaceVisibility, WorthQueryOrchestrationTranscriptFamily,
};
use super::row::{WorthQueryOrchestrationSemanticProfile, WorthQueryOrchestrationSurfaceInventory};
use super::strategy::WorthQueryOrchestrationStrategyAttachment;

pub(crate) fn worth_query_current_orchestration_surface_inventory(
) -> WorthQueryOrchestrationSurfaceInventory {
    let mut rows = current_row_specs()
        .into_iter()
        .map(RowSpec::into_row)
        .collect::<Vec<_>>();
    rows.extend(
        worth_query_recovery_orchestration_rows()
            .into_iter()
            .map(RowSpec::into_row),
    );
    rows.extend(worth_query_family_helper_orchestration_rows());
    WorthQueryOrchestrationSurfaceInventory::new(rows)
}

fn current_row_specs() -> Vec<RowSpec> {
    let declaration_doc =
        "crates/worth-query/docs/domain-capabilities/declaration-entry-orchestration.md";
    let continuation_doc = "crates/worth-query/docs/domain-capabilities/continuation-pipeline.md";
    let signal_doc =
        "crates/worth-query/docs/domain-capabilities/signal-compatibility-orchestration.md";
    let contribution_doc =
        "crates/worth-query/docs/domain-capabilities/contribution-composed-orchestration.md";

    let declaration_cert =
        "cargo test -p worth-query application::declaration_entry_orchestration -- --nocapture";
    let continuation_cert = "cargo test -p worth-query continuation_pipeline -- --nocapture";
    let signal_cert = "cargo test -p worth-query signal_compatibility_orchestration -- --nocapture";
    let contribution_cert =
        "cargo test -p worth-query contribution_composed_orchestration -- --nocapture";

    let declaration_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RequiredContract,
        WorthQueryOrchestrationBasisPosture::DeclarationEntry,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(),
        WorthQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let progression_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RequiredContract,
        WorthQueryOrchestrationBasisPosture::DeclarationEntry,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::none(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let continuation_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
        WorthQueryOrchestrationBasisPosture::ReadmissionAware,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::none(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let signal_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        WorthQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
        WorthQueryOrchestrationPolicyTenantPosture::WorkflowPreviewAware,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        WorthQueryOrchestrationContributionCompatibility::none(),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let contribution_profile = WorthQueryOrchestrationSemanticProfile::new(
        WorthQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
        WorthQueryOrchestrationBasisPosture::DeclarationScopedContribution,
        WorthQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        WorthQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        WorthQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        WorthQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::Admission,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
            crate::application::WorthQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath,
        ]),
        WorthQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );

    let mut rows = Vec::new();
    push_declaration_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        declaration_profile,
    );
    push_progressed_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        progression_profile,
    );
    push_continuation_rows(
        &mut rows,
        continuation_doc,
        continuation_cert,
        continuation_profile,
    );
    push_signal_rows(&mut rows, signal_doc, signal_cert, signal_profile);
    push_contribution_rows(
        &mut rows,
        contribution_doc,
        contribution_cert,
        contribution_profile,
    );
    rows
}

fn push_declaration_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_declaration_entry",
            family: WorthQueryOrchestrationSurfaceFamily::DeclarationEntry,
            transcript_family: WorthQueryOrchestrationTranscriptFamily::DeclarationEntry,
            checked_type_name: "WorthQueryDeclarationEntryOrchestrationChecked",
            proof_type_name: "WorthQueryDeclarationEntryOrchestrationProof",
            support_surface: WorthQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
            checked_topology_kind:
                WorthQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
            binding_projection: WorthQueryOrchestrationBindingProjection::None,
            semantic_profile,
            doc_path,
            ordinary_section: "admitted-handle entry points",
            outcome_section: "ordinary-outcome lane",
            checked_section: "checked lane",
            proof_section: "proof-visible lane",
            certification_suite: "application::declaration_entry_orchestration",
            certification_command,
        },
    );
}

fn push_progressed_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
) {
    for (name, family, transcript, checked_type, proof_type) in [
        (
            "orchestrate_routes_from_progressed",
            WorthQueryOrchestrationSurfaceFamily::RouteFromProgressed,
            WorthQueryOrchestrationTranscriptFamily::DeclarationRoute,
            "WorthQueryDeclarationRoutePlanChecked",
            "WorthQueryDeclarationRouteOrchestrationProof",
        ),
        (
            "orchestrate_receipt_from_progressed",
            WorthQueryOrchestrationSurfaceFamily::ReceiptFromProgressed,
            WorthQueryOrchestrationTranscriptFamily::DeclarationReceipt,
            "WorthQueryDeclarationReceiptChecked",
            "WorthQueryDeclarationReceiptOrchestrationProof",
        ),
        (
            "orchestrate_envelope_from_progressed",
            WorthQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed,
            WorthQueryOrchestrationTranscriptFamily::DeclarationEnvelope,
            "WorthQueryDeclarationEnvelopeChecked",
            "WorthQueryDeclarationEnvelopeOrchestrationProof",
        ),
    ] {
        for (public_name, visibility, doc_section) in [
            (
                name,
                WorthQueryOrchestrationSurfaceVisibility::Ordinary,
                "product ordinary lane",
            ),
            (
                super::current_support::leak(format!("{name}_with_intent")),
                WorthQueryOrchestrationSurfaceVisibility::Ordinary,
                "product ordinary lane",
            ),
            (
                super::current_support::leak(format!("{name}_checked")),
                WorthQueryOrchestrationSurfaceVisibility::Checked,
                "product checked lane",
            ),
            (
                super::current_support::leak(format!("{name}_checked_with_intent")),
                WorthQueryOrchestrationSurfaceVisibility::Checked,
                "product checked lane",
            ),
            (
                super::current_support::leak(format!("{name}_proof")),
                WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
                "product proof lane",
            ),
            (
                super::current_support::leak(format!("{name}_proof_with_intent")),
                WorthQueryOrchestrationSurfaceVisibility::ProofVisible,
                "product proof lane",
            ),
        ] {
            rows.push(RowSpec {
                public_name,
                canonical_base_name: name,
                family,
                visibility,
                ordinary_outcome_supported: false,
                binding_projection: WorthQueryOrchestrationBindingProjection::None,
                checked_type_name: checked_type,
                proof_type_name: proof_type,
                transcript_family: transcript,
                checked_topology_kind:
                    WorthQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
                support_surface:
                    WorthQueryOrchestrationSupportSurface::DeclarationEntryCrossingInventory,
                semantic_profile: semantic_profile.clone(),
                doc_path,
                doc_section,
                certification_suite: "application::declaration_entry_orchestration",
                certification_command,
            });
        }
    }
}

fn push_continuation_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
) {
    for (base_name, family, transcript) in [
        (
            "prepare_continuation_from_target",
            WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget,
            WorthQueryOrchestrationTranscriptFamily::PreparedContinuation,
        ),
        (
            "prepare_continuation_from_context",
            WorthQueryOrchestrationSurfaceFamily::ContinuationPrepareContext,
            WorthQueryOrchestrationTranscriptFamily::PreparedContinuation,
        ),
        (
            "execute_prepared_continuation",
            WorthQueryOrchestrationSurfaceFamily::ContinuationExecute,
            WorthQueryOrchestrationTranscriptFamily::ContinuationExecution,
        ),
    ] {
        push_four_lane_rows(
            rows,
            FourLaneSpec {
                base_name,
                family,
                transcript_family: transcript,
                checked_type_name: if base_name == "execute_prepared_continuation" {
                    "WorthQueryContinuationExecutionChecked"
                } else {
                    "WorthQueryPreparedContinuationChecked"
                },
                proof_type_name: if base_name == "execute_prepared_continuation" {
                    "WorthQueryContinuationExecutionTranscript"
                } else {
                    "WorthQueryPreparedContinuationTranscript"
                },
                support_surface:
                    WorthQueryOrchestrationSupportSurface::ContinuationPreparedContract,
                checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind::Continuation,
                binding_projection:
                    WorthQueryOrchestrationBindingProjection::SharedContinuationBinding,
                semantic_profile: semantic_profile.clone(),
                doc_path,
                ordinary_section: "ordinary lane",
                outcome_section: "ordinary outcome lane",
                checked_section: "checked lane",
                proof_section: "proof-visible lane",
                certification_suite: family.as_str(),
                certification_command,
            },
        );
    }
}

fn push_signal_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_signal_compatibility",
            family: WorthQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
            transcript_family:
                WorthQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
            checked_type_name: "WorthQuerySignalCompatibilityOrchestrationChecked",
            proof_type_name: "WorthQuerySignalCompatibilityOrchestrationTranscript",
            support_surface:
                WorthQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
            checked_topology_kind:
                WorthQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
            binding_projection:
                WorthQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
            semantic_profile,
            doc_path,
            ordinary_section: "ordinary lane",
            outcome_section: "ordinary outcome lane",
            checked_section: "checked lane",
            proof_section: "proof-visible lane",
            certification_suite: "signal_compatibility_orchestration",
            certification_command,
        },
    );
}

fn push_contribution_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    semantic_profile: WorthQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_declaration_with_contributions",
            family: WorthQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
            transcript_family:
                WorthQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
            checked_type_name: "WorthQueryContributionComposedOrchestrationChecked",
            proof_type_name: "WorthQueryContributionComposedOrchestrationTranscript",
            support_surface:
                WorthQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
            checked_topology_kind: WorthQueryOrchestrationCheckedTopologyKind::ContributionComposed,
            binding_projection: WorthQueryOrchestrationBindingProjection::SharedContributionBinding,
            semantic_profile,
            doc_path,
            ordinary_section: "ordinary lane",
            outcome_section: "ordinary outcome lane",
            checked_section: "checked lane",
            proof_section: "proof-visible lane",
            certification_suite: "contribution_composed_orchestration",
            certification_command,
        },
    );
}
