use super::certification::ForgeQueryOrchestrationSurfaceCertificationReference;
use super::docs::ForgeQueryOrchestrationSurfaceDocReference;
use super::family::{
    ForgeQueryOrchestrationBindingProjection, ForgeQueryOrchestrationCheckedTopologyKind,
    ForgeQueryOrchestrationSupportSurface, ForgeQueryOrchestrationSurfaceFamily,
    ForgeQueryOrchestrationSurfaceVisibility, ForgeQueryOrchestrationTranscriptFamily,
};
use super::row::{ForgeQueryOrchestrationSurfaceInventory, ForgeQueryOrchestrationSurfaceRow};
use super::transcript::ForgeQueryOrchestrationProofContract;

struct RowSpec {
    public_name: &'static str,
    canonical_base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    visibility: ForgeQueryOrchestrationSurfaceVisibility,
    ordinary_outcome_supported: bool,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    support_surface: ForgeQueryOrchestrationSupportSurface,
    doc_path: &'static str,
    doc_section: &'static str,
    certification_suite: &'static str,
    certification_command: &'static str,
}

pub(crate) fn forge_query_current_orchestration_surface_inventory(
) -> ForgeQueryOrchestrationSurfaceInventory {
    ForgeQueryOrchestrationSurfaceInventory::new(
        current_row_specs()
            .iter()
            .map(|spec| {
                ForgeQueryOrchestrationSurfaceRow::new(
                    spec.public_name,
                    spec.canonical_base_name,
                    spec.family,
                    spec.visibility,
                    spec.ordinary_outcome_supported,
                    spec.binding_projection,
                    ForgeQueryOrchestrationProofContract::new(
                        spec.checked_type_name,
                        spec.proof_type_name,
                        spec.transcript_family,
                        spec.checked_topology_kind,
                        spec.support_surface,
                    ),
                    ForgeQueryOrchestrationSurfaceDocReference::new(
                        spec.doc_path,
                        spec.doc_section,
                    ),
                    ForgeQueryOrchestrationSurfaceCertificationReference::new(
                        spec.certification_suite,
                        spec.certification_command,
                    ),
                )
            })
            .collect(),
    )
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

    let mut rows = Vec::new();

    push_declaration_entry_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        "orchestrate_declaration_entry",
        ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry,
        ForgeQueryOrchestrationTranscriptFamily::DeclarationEntry,
        "ForgeQueryDeclarationEntryOrchestrationChecked",
        "ForgeQueryDeclarationEntryOrchestrationProof",
        ForgeQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
        true,
    );
    rows.push(RowSpec {
        public_name: "orchestrate_declaration_entry_outcome",
        canonical_base_name: "orchestrate_declaration_entry",
        family: ForgeQueryOrchestrationSurfaceFamily::DeclarationEntry,
        visibility: ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome,
        ordinary_outcome_supported: true,
        binding_projection: ForgeQueryOrchestrationBindingProjection::None,
        checked_type_name: "ForgeQueryDeclarationEntryOrchestrationChecked",
        proof_type_name: "ForgeQueryDeclarationEntryOrchestrationProof",
        transcript_family: ForgeQueryOrchestrationTranscriptFamily::DeclarationEntry,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
        support_surface: ForgeQueryOrchestrationSupportSurface::DeclarationEntryReadiness,
        doc_path: declaration_doc,
        doc_section: "ordinary-outcome lane",
        certification_suite: "application::declaration_entry_orchestration",
        certification_command: declaration_cert,
    });

    push_progressed_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        "orchestrate_routes_from_progressed",
        ForgeQueryOrchestrationSurfaceFamily::RouteFromProgressed,
        ForgeQueryOrchestrationTranscriptFamily::DeclarationRoute,
        "ForgeQueryDeclarationRoutePlanChecked",
        "ForgeQueryDeclarationRouteOrchestrationProof",
    );
    push_progressed_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        "orchestrate_receipt_from_progressed",
        ForgeQueryOrchestrationSurfaceFamily::ReceiptFromProgressed,
        ForgeQueryOrchestrationTranscriptFamily::DeclarationReceipt,
        "ForgeQueryDeclarationReceiptChecked",
        "ForgeQueryDeclarationReceiptOrchestrationProof",
    );
    push_progressed_rows(
        &mut rows,
        declaration_doc,
        declaration_cert,
        "orchestrate_envelope_from_progressed",
        ForgeQueryOrchestrationSurfaceFamily::EnvelopeFromProgressed,
        ForgeQueryOrchestrationTranscriptFamily::DeclarationEnvelope,
        "ForgeQueryDeclarationEnvelopeChecked",
        "ForgeQueryDeclarationEnvelopeOrchestrationProof",
    );

    push_four_lane_rows(
        &mut rows,
        continuation_doc,
        continuation_cert,
        "prepare_continuation_from_target",
        ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareTarget,
        ForgeQueryOrchestrationTranscriptFamily::PreparedContinuation,
        "ForgeQueryPreparedContinuationChecked",
        "ForgeQueryPreparedContinuationTranscript",
        ForgeQueryOrchestrationSupportSurface::ContinuationPreparedContract,
        ForgeQueryOrchestrationCheckedTopologyKind::Continuation,
        ForgeQueryOrchestrationBindingProjection::SharedContinuationBinding,
    );
    push_four_lane_rows(
        &mut rows,
        continuation_doc,
        continuation_cert,
        "prepare_continuation_from_context",
        ForgeQueryOrchestrationSurfaceFamily::ContinuationPrepareContext,
        ForgeQueryOrchestrationTranscriptFamily::PreparedContinuation,
        "ForgeQueryPreparedContinuationChecked",
        "ForgeQueryPreparedContinuationTranscript",
        ForgeQueryOrchestrationSupportSurface::ContinuationPreparedContract,
        ForgeQueryOrchestrationCheckedTopologyKind::Continuation,
        ForgeQueryOrchestrationBindingProjection::SharedContinuationBinding,
    );
    push_four_lane_rows(
        &mut rows,
        continuation_doc,
        continuation_cert,
        "execute_prepared_continuation",
        ForgeQueryOrchestrationSurfaceFamily::ContinuationExecute,
        ForgeQueryOrchestrationTranscriptFamily::ContinuationExecution,
        "ForgeQueryContinuationExecutionChecked",
        "ForgeQueryContinuationExecutionTranscript",
        ForgeQueryOrchestrationSupportSurface::ContinuationPreparedContract,
        ForgeQueryOrchestrationCheckedTopologyKind::Continuation,
        ForgeQueryOrchestrationBindingProjection::SharedContinuationBinding,
    );

    push_four_lane_rows(
        &mut rows,
        signal_doc,
        signal_cert,
        "orchestrate_signal_compatibility",
        ForgeQueryOrchestrationSurfaceFamily::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::SignalCompatibilityOrchestration,
        "ForgeQuerySignalCompatibilityOrchestrationChecked",
        "ForgeQuerySignalCompatibilityOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::SignalCompatibilityOrchestration,
        ForgeQueryOrchestrationBindingProjection::SharedSignalCompatibilityBinding,
    );
    push_four_lane_rows(
        &mut rows,
        contribution_doc,
        contribution_cert,
        "orchestrate_declaration_with_contributions",
        ForgeQueryOrchestrationSurfaceFamily::ContributionComposedOrchestration,
        ForgeQueryOrchestrationTranscriptFamily::ContributionComposedOrchestration,
        "ForgeQueryContributionComposedOrchestrationChecked",
        "ForgeQueryContributionComposedOrchestrationTranscript",
        ForgeQueryOrchestrationSupportSurface::ContributionComposedOrchestration,
        ForgeQueryOrchestrationCheckedTopologyKind::ContributionComposed,
        ForgeQueryOrchestrationBindingProjection::SharedContributionBinding,
    );

    rows
}

fn leak(text: String) -> &'static str {
    text.leak()
}

fn push_declaration_entry_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    support_surface: ForgeQueryOrchestrationSupportSurface,
    ordinary_outcome_supported: bool,
) {
    rows.push(RowSpec {
        public_name: base_name,
        canonical_base_name: base_name,
        family,
        visibility: ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
        ordinary_outcome_supported,
        binding_projection: ForgeQueryOrchestrationBindingProjection::None,
        checked_type_name,
        proof_type_name,
        transcript_family,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
        support_surface,
        doc_path,
        doc_section: "admitted-handle entry points",
        certification_suite: "application::declaration_entry_orchestration",
        certification_command,
    });
    rows.push(RowSpec {
        public_name: leak(format!("{base_name}_checked")),
        canonical_base_name: base_name,
        family,
        visibility: ForgeQueryOrchestrationSurfaceVisibility::Checked,
        ordinary_outcome_supported,
        binding_projection: ForgeQueryOrchestrationBindingProjection::None,
        checked_type_name,
        proof_type_name,
        transcript_family,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
        support_surface,
        doc_path,
        doc_section: "checked lane",
        certification_suite: "application::declaration_entry_orchestration",
        certification_command,
    });
    rows.push(RowSpec {
        public_name: leak(format!("{base_name}_proof")),
        canonical_base_name: base_name,
        family,
        visibility: ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
        ordinary_outcome_supported,
        binding_projection: ForgeQueryOrchestrationBindingProjection::None,
        checked_type_name,
        proof_type_name,
        transcript_family,
        checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage,
        support_surface,
        doc_path,
        doc_section: "proof-visible lane",
        certification_suite: "application::declaration_entry_orchestration",
        certification_command,
    });
}

fn push_progressed_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
) {
    let support_surface = ForgeQueryOrchestrationSupportSurface::DeclarationEntryCrossingInventory;
    let checked_kind = ForgeQueryOrchestrationCheckedTopologyKind::DeclarationEntryStage;
    for (public_name, visibility, doc_section) in [
        (
            base_name,
            ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
            "product ordinary lane",
        ),
        (
            leak(format!("{base_name}_with_intent")),
            ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
            "product ordinary lane",
        ),
        (
            leak(format!("{base_name}_checked")),
            ForgeQueryOrchestrationSurfaceVisibility::Checked,
            "product checked lane",
        ),
        (
            leak(format!("{base_name}_checked_with_intent")),
            ForgeQueryOrchestrationSurfaceVisibility::Checked,
            "product checked lane",
        ),
        (
            leak(format!("{base_name}_proof")),
            ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
            "product proof lane",
        ),
        (
            leak(format!("{base_name}_proof_with_intent")),
            ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
            "product proof lane",
        ),
    ] {
        rows.push(RowSpec {
            public_name,
            canonical_base_name: base_name,
            family,
            visibility,
            ordinary_outcome_supported: false,
            binding_projection: ForgeQueryOrchestrationBindingProjection::None,
            checked_type_name,
            proof_type_name,
            transcript_family,
            checked_topology_kind: checked_kind,
            support_surface,
            doc_path,
            doc_section,
            certification_suite: "application::declaration_entry_orchestration",
            certification_command,
        });
    }
}

fn push_four_lane_rows(
    rows: &mut Vec<RowSpec>,
    doc_path: &'static str,
    certification_command: &'static str,
    base_name: &'static str,
    family: ForgeQueryOrchestrationSurfaceFamily,
    transcript_family: ForgeQueryOrchestrationTranscriptFamily,
    checked_type_name: &'static str,
    proof_type_name: &'static str,
    support_surface: ForgeQueryOrchestrationSupportSurface,
    checked_topology_kind: ForgeQueryOrchestrationCheckedTopologyKind,
    binding_projection: ForgeQueryOrchestrationBindingProjection,
) {
    for (public_name, visibility, doc_section) in [
        (
            base_name,
            ForgeQueryOrchestrationSurfaceVisibility::Ordinary,
            "ordinary lane",
        ),
        (
            leak(format!("{base_name}_outcome")),
            ForgeQueryOrchestrationSurfaceVisibility::OrdinaryOutcome,
            "ordinary outcome lane",
        ),
        (
            leak(format!("{base_name}_checked")),
            ForgeQueryOrchestrationSurfaceVisibility::Checked,
            "checked lane",
        ),
        (
            leak(format!("{base_name}_proof")),
            ForgeQueryOrchestrationSurfaceVisibility::ProofVisible,
            "proof-visible lane",
        ),
    ] {
        rows.push(RowSpec {
            public_name,
            canonical_base_name: base_name,
            family,
            visibility,
            ordinary_outcome_supported: true,
            binding_projection,
            checked_type_name,
            proof_type_name,
            transcript_family,
            checked_topology_kind,
            support_surface,
            doc_path,
            doc_section,
            certification_suite: family.as_str(),
            certification_command,
        });
    }
}
