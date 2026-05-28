use super::aspect::ForgeQueryOrchestrationAspectPosture;
use super::authority::{
    ForgeQueryOrchestrationBasisPosture, ForgeQueryOrchestrationCollaborativeExtensionPosture,
    ForgeQueryOrchestrationLowerAuthorityAttachment, ForgeQueryOrchestrationPolicyTenantPosture,
};
use super::contribution::ForgeQueryOrchestrationContributionCompatibility;
use super::current_helpers::forge_query_family_helper_orchestration_rows;
use super::current_recovery::forge_query_recovery_orchestration_rows;
use super::current_support::{push_four_lane_rows, FourLaneSpec, RowSpec};
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::{ForgeQueryOrchestrationSemanticProfile, ForgeQueryOrchestrationSurfaceInventory};
use super::strategy::ForgeQueryOrchestrationStrategyAttachment;

pub(crate) fn forge_query_current_orchestration_surface_inventory(
) -> ForgeQueryOrchestrationSurfaceInventory {
    let mut rows = current_row_specs()
        .into_iter()
        .map(RowSpec::into_row)
        .collect::<Vec<_>>();
    rows.extend(
        forge_query_recovery_orchestration_rows()
            .into_iter()
            .map(RowSpec::into_row),
    );
    rows.extend(forge_query_family_helper_orchestration_rows());
    ForgeQueryOrchestrationSurfaceInventory::new(rows)
}

fn current_row_specs() -> Vec<RowSpec> {
    let declaration_doc =
        "crates/forge-query/docs/domain-capabilities/declaration-entry-orchestration.md";
    let continuation_doc = "crates/forge-query/docs/domain-capabilities/continuation-pipeline.md";
    let signal_doc =
        "crates/forge-query/docs/domain-capabilities/signal-compatibility-orchestration.md";
    let contribution_doc =
        "crates/forge-query/docs/domain-capabilities/contribution-composed-orchestration.md";

    let declaration_cert =
        "cargo test -p forge-query application::declaration_entry_orchestration -- --nocapture";
    let continuation_cert = "cargo test -p forge-query continuation_pipeline -- --nocapture";
    let signal_cert = "cargo test -p forge-query signal_compatibility_orchestration -- --nocapture";
    let contribution_cert =
        "cargo test -p forge-query contribution_composed_orchestration -- --nocapture";

    let declaration_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RequiredContract,
        ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal_foundational(),
        ForgeQueryOrchestrationStrategyAttachment::declaration_entry_foundational(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let progression_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RequiredContract,
        ForgeQueryOrchestrationBasisPosture::DeclarationEntry,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::none(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let continuation_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::AspectSensitiveReadmission,
        ForgeQueryOrchestrationBasisPosture::ReadmissionAware,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::none(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let signal_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::RetainedContractAndCoverage,
        ForgeQueryOrchestrationBasisPosture::SignalCompatibilityRetained,
        ForgeQueryOrchestrationPolicyTenantPosture::WorkflowPreviewAware,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::signal_and_bridge(),
        ForgeQueryOrchestrationContributionCompatibility::none(),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
    );
    let contribution_profile = ForgeQueryOrchestrationSemanticProfile::new(
        ForgeQueryOrchestrationAspectPosture::CategoryScopedAspectComposition,
        ForgeQueryOrchestrationBasisPosture::DeclarationScopedContribution,
        ForgeQueryOrchestrationPolicyTenantPosture::InheritedPlatformPolicy,
        ForgeQueryOrchestrationLowerAuthorityAttachment::relational_bridge_signal(),
        ForgeQueryOrchestrationStrategyAttachment::foundational_materialization_profile(),
        ForgeQueryOrchestrationContributionCompatibility::declaration_scoped(vec![
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::Admission,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::SupportTraceability,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::ExplanationInspection,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::WorkflowPreview,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::ContinuityLineage,
            crate::application::ForgeQueryDeclarationEntryContributionCategoryFamily::ConsequenceAftermath,
        ]),
        ForgeQueryOrchestrationCollaborativeExtensionPosture::CollaborativePhasesReady,
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
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_declaration_entry",
            family: ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry,
            transcript_family: ForgeQueryOrchestrationTranscriptFamily::DeclarationEntry,
            checked_type_name: "ForgeQueryDeclarationEntryOrchestrationChecked",
            proof_type_name: "ForgeQueryDeclarationEntryOrchestrationProof",
            support_surface: ForgeQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
            checked_topology_kind:
                ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
            binding_projection: ForgeQueryOrchestrationBindingProjection::None,
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
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
) {
    for (name, family, transcript, checked_type, proof_type) in [
        (
            "orchestrate_routes_from_progressed",
            ForgeQueryOrchestrationSurfaceFamily::RouteFromProgressed,
            ForgeQueryOrchestrationTranscriptFamily::DeclarationRoute,
            "ForgeQueryDeclarationRoutePlanChecked",
            "ForgeQueryDeclarationRouteOrchestrationProof",
        ),
        (
            "orchestrate_receipt_from_progressed",
            ForgeQueryOrchestrationSurfaceFamily::ReceiptFromProgressed,
            ForgeQueryOrchestrationTranscriptFamily::DeclarationReceipt,
            "ForgeQueryDeclarationReceiptChecked",
            "ForgeQueryDeclarationReceiptOrchestrationProof",
        ),
        (
            "orchestrate_envelope_from_progressed",
            ForgeQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed,
            ForgeQueryOrchestrationTranscriptFamily::DeclarationEnvelope,
            "ForgeQueryDeclarationEnvelopeChecked",
            "ForgeQueryDeclarationEnvelopeOrchestrationProof",
        ),
    ] {
        for (public_name, visibility, doc_section) in [
            (
                name,
                ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
                "product ordinary lane",
            ),
            (
                super::current_support::leak(format!("{name}_with_intent")),
                ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
                "product ordinary lane",
            ),
            (
                super::current_support::leak(format!("{name}_checked")),
                ForgeQueryOrchestrationSurfaceVisibility::Checked,
                "product checked lane",
            ),
            (
                super::current_support::leak(format!("{name}_checked_with_intent")),
                ForgeQueryOrchestrationSurfaceVisibility::Checked,
                "product checked lane",
            ),
            (
                super::current_support::leak(format!("{name}_proof")),
                ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
                "product proof lane",
            ),
            (
                super::current_support::leak(format!("{name}_proof_with_intent")),
                ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
                "product proof lane",
            ),
        ] {
            rows.push(RowSpec {
                public_name,
                canonical_base_name: name,
                family,
                visibility,
                ordinary_outcome_supported: false,
                binding_projection: ForgeQueryOrchestrationBindingProjection::None,
                checked_type_name: checked_type,
                proof_type_name: proof_type,
                transcript_family: transcript,
                checked_topology_kind:
                    ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
                support_surface:
                    ForgeQueryOrchestrationSupportSurface::DeclarationEntryCrossingInventory,
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
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
) {
    for (base_name, family, transcript) in [
        (
            "prepare_continuation_from_target",
            ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget,
            ForgeQueryOrchestrationTranscriptFamily::PreparedContinuation,
        ),
        (
            "prepare_continuation_from_context",
            ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext,
            ForgeQueryOrchestrationTranscriptFamily::PreparedContinuation,
        ),
        (
            "execute_prepared_continuation",
            ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute,
            ForgeQueryOrchestrationTranscriptFamily::ContinuationExecution,
        ),
    ] {
        push_four_lane_rows(
            rows,
            FourLaneSpec {
                base_name,
                family,
                transcript_family: transcript,
                checked_type_name: if base_name == "execute_prepared_continuation" {
                    "ForgeQueryContinuationExecutionChecked"
                } else {
                    "ForgeQueryPreparedContinuationChecked"
                },
                proof_type_name: if base_name == "execute_prepared_continuation" {
                    "ForgeQueryContinuationExecutionTranscript"
                } else {
                    "ForgeQueryPreparedContinuationTranscript"
                },
                support_surface:
                    ForgeQueryOrchestrationSupportSurface::ContinuationPreparedContract,
                checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::Continuation,
                binding_projection:
                    ForgeQueryOrchestrationBindingProjection::SharedContinuationBinding,
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
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_signal_compatibility",
            family: ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
            transcript_family:
                ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
            checked_type_name: "ForgeQuerySignalCompatibilityOrchestrationChecked",
            proof_type_name: "ForgeQuerySignalCompatibilityOrchestrationTranscript",
            support_surface:
                ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
            checked_topology_kind:
                ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
            binding_projection:
                ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
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
    semantic_profile: ForgeQueryOrchestrationSemanticProfile,
) {
    push_four_lane_rows(
        rows,
        FourLaneSpec {
            base_name: "orchestrate_declaration_with_contributions",
            family: ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
            transcript_family:
                ForgeQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
            checked_type_name: "ForgeQueryContributionComposedOrchestrationChecked",
            proof_type_name: "ForgeQueryContributionComposedOrchestrationTranscript",
            support_surface:
                ForgeQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
            checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::ContributionComposed,
            binding_projection: ForgeQueryOrchestrationBindingProjection::SharedContributionBinding,
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
