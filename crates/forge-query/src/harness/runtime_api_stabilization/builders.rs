use crate::harness::certification::{
    digest_parts, CanonicalCertificationRow, HostileExpectation, ParityAnchor,
    RejectionCertificationRow,
};
use crate::runtime::{
    ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimeFamilySupportStatus,
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicApiNamingContract,
    ForgeQueryRuntimeSupportProfile,
};

use super::{
    transcripts, RuntimeApiStabilizationBundle, RuntimeApiStabilizationCertificationMatrix,
    RuntimeApiStabilizationFailureClass, RuntimeApiStabilizationPerturbationClass,
    RuntimeApiStabilizationRejectionBundle,
};

pub(super) fn canonical_rows() -> Vec<
    CanonicalCertificationRow<
        RuntimeApiStabilizationPerturbationClass,
        RuntimeApiStabilizationBundle,
    >,
> {
    vec![
        canonical_row(
            "workflow-editor-golden-transcript",
            RuntimeApiStabilizationPerturbationClass::WorkflowEditorGoldenTranscript,
            transcripts::workflow_editor_transcript,
            [
                "live",
                "computed",
                "effect",
                "branch-preview",
                "branch-intent",
                "inspect",
            ],
            9,
        ),
        canonical_row(
            "geometry-kernel-golden-transcript",
            RuntimeApiStabilizationPerturbationClass::GeometryKernelGoldenTranscript,
            transcripts::geometry_kernel_transcript,
            [
                "topology-live",
                "derived-surface",
                "fallback",
                "intent",
                "inspect",
            ],
            8,
        ),
        canonical_row(
            "table-spreadsheet-golden-transcript",
            RuntimeApiStabilizationPerturbationClass::TableSpreadsheetGoldenTranscript,
            transcripts::table_spreadsheet_transcript,
            [
                "visible-rows",
                "formula",
                "dropdown",
                "layout",
                "batched-intent",
            ],
            8,
        ),
        adversarial_composed_row(),
    ]
}

pub(super) fn rejection_rows() -> Vec<
    RejectionCertificationRow<
        RuntimeApiStabilizationPerturbationClass,
        RuntimeApiStabilizationBundle,
        RuntimeApiStabilizationRejectionBundle,
    >,
> {
    vec![
        rejection_row(
            "temporal-basis-deferred-gate",
            RuntimeApiStabilizationPerturbationClass::TemporalBasisDeferredGate,
            ForgeQueryRuntimeFacadeFamily::Temporal,
            RuntimeApiStabilizationFailureClass::DeferredTemporalAsyncGate,
        ),
        rejection_row(
            "async-resource-deferred-gate",
            RuntimeApiStabilizationPerturbationClass::AsyncResourceDeferredGate,
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            RuntimeApiStabilizationFailureClass::DeferredTemporalAsyncGate,
        ),
        rejection_row(
            "mixed-cause-delivery-deferred-gate",
            RuntimeApiStabilizationPerturbationClass::MixedCauseDeliveryDeferredGate,
            ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery,
            RuntimeApiStabilizationFailureClass::DeferredTemporalAsyncGate,
        ),
        rejection_row(
            "store-backed-parity-deferred-gate",
            RuntimeApiStabilizationPerturbationClass::StoreBackedParityDeferredGate,
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
            RuntimeApiStabilizationFailureClass::DeferredStoreDurableGate,
        ),
        rejection_row(
            "durable-restart-deferred-gate",
            RuntimeApiStabilizationPerturbationClass::DurableRestartDeferredGate,
            ForgeQueryRuntimeFacadeFamily::DurableArtifacts,
            RuntimeApiStabilizationFailureClass::DeferredStoreDurableGate,
        ),
    ]
}

pub(super) fn bundle_digest_parts(
    matrix: &RuntimeApiStabilizationCertificationMatrix,
) -> Vec<String> {
    matrix
        .rows
        .iter()
        .flat_map(|row| {
            [
                row.control_lane.semantic_signature(),
                row.hostile_lane.semantic_signature(),
                row.parity_lane.semantic_signature(),
            ]
        })
        .chain(matrix.rejection_rows.iter().map(|row| {
            digest_parts(&[
                row.row_name.to_string(),
                row.hostile_lane.failure_digest.clone(),
                row.hostile_lane.deferred_temporal_async_gate_digest.clone(),
            ])
        }))
        .collect()
}

pub(super) fn coverage_digest_parts(
    matrix: &RuntimeApiStabilizationCertificationMatrix,
) -> Vec<String> {
    matrix
        .rows
        .iter()
        .map(|row| format!("canonical:{}", row.row_name))
        .chain(
            matrix
                .rejection_rows
                .iter()
                .map(|row| format!("rejection:{}", row.row_name)),
        )
        .collect()
}

fn canonical_row(
    row_name: &'static str,
    perturbation_class: RuntimeApiStabilizationPerturbationClass,
    transcript: fn() -> crate::runtime::ForgeQueryRuntimePublicApiTranscriptEvidence,
    surfaces: impl IntoIterator<Item = &'static str> + Clone,
    meaningful_assertion_count: usize,
) -> CanonicalCertificationRow<
    RuntimeApiStabilizationPerturbationClass,
    RuntimeApiStabilizationBundle,
> {
    let transcript_evidence = transcript();
    CanonicalCertificationRow {
        row_name,
        perturbation_class,
        hostile_expectation: HostileExpectation::EquivalentToControl,
        parity_anchor: ParityAnchor::Control,
        control_lane: bundle(
            transcript_evidence.clone(),
            surfaces.clone(),
            meaningful_assertion_count,
        ),
        hostile_lane: bundle(
            transcript_evidence.clone(),
            surfaces.clone(),
            meaningful_assertion_count,
        ),
        parity_lane: bundle(transcript_evidence, surfaces, meaningful_assertion_count),
    }
}

fn adversarial_composed_row() -> CanonicalCertificationRow<
    RuntimeApiStabilizationPerturbationClass,
    RuntimeApiStabilizationBundle,
> {
    let control_lane = bundle(
        transcripts::composed_runtime_transcript(),
        [
            "live-subscription",
            "nested-computed",
            "pending-intent-effect",
            "authoritative-intent",
            "effect-intent",
            "branch-intent",
            "preview-isolation",
            "feedback-graph",
        ],
        14,
    );
    let hostile_lane = bundle(
        transcripts::composed_runtime_hostile_transcript(),
        [
            "live-subscription",
            "nested-computed",
            "pending-intent-effect",
            "authoritative-intent",
            "effect-intent",
            "branch-intent",
            "preview-isolation",
            "feedback-graph",
            "temporal-neighbor-denial",
            "async-resource-neighbor-denial",
        ],
        18,
    );
    CanonicalCertificationRow {
        row_name: "composed-runtime-adversarial-transcript",
        perturbation_class:
            RuntimeApiStabilizationPerturbationClass::ComposedRuntimeAdversarialTranscript,
        hostile_expectation: HostileExpectation::DistinctFromControl,
        parity_anchor: ParityAnchor::Hostile,
        control_lane,
        parity_lane: hostile_lane.clone(),
        hostile_lane,
    }
}

fn rejection_row(
    row_name: &'static str,
    perturbation_class: RuntimeApiStabilizationPerturbationClass,
    family: ForgeQueryRuntimeFacadeFamily,
    failure_class: RuntimeApiStabilizationFailureClass,
) -> RejectionCertificationRow<
    RuntimeApiStabilizationPerturbationClass,
    RuntimeApiStabilizationBundle,
    RuntimeApiStabilizationRejectionBundle,
> {
    let control_lane = bundle(
        transcripts::workflow_editor_transcript(),
        ["read", "live", "inspect"],
        6,
    );
    let contract = contract();
    let row = contract
        .family(family)
        .expect("deferred public API family should have support row");
    let failure_digest = digest_parts(&[
        format!("failure_class:{failure_class:?}"),
        format!("family:{}", family.as_str()),
        format!("status:{}", row.status().as_str()),
        format!("reason:{}", row.reason().unwrap_or("none")),
    ]);
    RejectionCertificationRow {
        row_name,
        perturbation_class,
        control_lane: control_lane.clone(),
        hostile_lane: RuntimeApiStabilizationRejectionBundle {
            failure_class,
            family,
            status: row.status(),
            failure_digest,
            deferred_temporal_async_gate_digest: row.contract_digest().to_string(),
            counter_snapshot: "forbidden_delivery_residue=0;authority_residue=0".to_string(),
            compile_fail_boundary_digest: compile_fail_boundary_digest(),
        },
        parity_lane: control_lane,
    }
}

fn bundle(
    transcript_evidence: crate::runtime::ForgeQueryRuntimePublicApiTranscriptEvidence,
    surfaces: impl IntoIterator<Item = &'static str>,
    meaningful_assertion_count: usize,
) -> RuntimeApiStabilizationBundle {
    let contract = contract();
    let naming_contract = ForgeQueryRuntimePublicApiNamingContract::standard();
    let surface_list: Vec<_> = surfaces.into_iter().collect();
    let transcript_family = transcript_evidence.transcript_family().to_string();
    let golden_transcript_digest = digest_parts(
        &surface_list
            .iter()
            .map(|surface| format!("surface:{surface}"))
            .chain([format!("transcript:{transcript_family}")])
            .collect::<Vec<_>>(),
    );
    RuntimeApiStabilizationBundle {
        public_api_surface_digest: contract.contract_digest().to_string(),
        public_api_naming_contract_digest: naming_contract.contract_digest().to_string(),
        golden_transcript_digest,
        executable_transcript_digest: transcript_evidence.transcript_digest().to_string(),
        handle_contract_digest: digest_parts(&[
            "handle:named-durable-surface".to_string(),
            "handle:dependency-digests".to_string(),
            "handle:authority-lane".to_string(),
            "handle:inspectable".to_string(),
        ]),
        state_contract_digest: digest_parts(&[
            "state:ready".to_string(),
            "state:pending".to_string(),
            "state:stale".to_string(),
            "state:failed".to_string(),
            "state:cancelled".to_string(),
            "state:superseded".to_string(),
            "state:denied".to_string(),
            "state:unsupported".to_string(),
        ]),
        aspect_contract_digest: digest_parts(&[
            "aspect:reads".to_string(),
            "aspect:produces".to_string(),
            "aspect:trigger".to_string(),
            "aspect:condition-inputs".to_string(),
        ]),
        authority_lane_digest: digest_parts(&[
            "lane:authoritative-truth".to_string(),
            "lane:branch-local-truth".to_string(),
            "lane:preview-truth".to_string(),
            "lane:derived-runtime-state".to_string(),
            "lane:effect-delivery-state".to_string(),
            "lane:pending-write-intent".to_string(),
            "lane:bridge-external-state".to_string(),
            "lane:temporal-execution-state".to_string(),
            "lane:async-resource-state".to_string(),
        ]),
        inspection_contract_digest: digest_parts(&[
            "inspection:declaration".to_string(),
            "inspection:dependency".to_string(),
            "inspection:authority-lane".to_string(),
            "inspection:basis-lane".to_string(),
            "inspection:feedback-phase-graph".to_string(),
            "inspection:deferred-temporal-async".to_string(),
        ]),
        support_matrix_digest: support_matrix_digest(&contract),
        deferred_temporal_async_gate_digest: deferred_gate_digest(&contract),
        failure_digest: "none".to_string(),
        counter_snapshot: format!(
            "stable={};deferred={};unsupported={};preferred_names={};compat_names={};assertions={meaningful_assertion_count};plumbing=0;denials={};residue={}",
            contract.stable_family_count(),
            contract.deferred_family_count(),
            contract.unsupported_family_count(),
            naming_contract.preferred_entrypoint_count(),
            naming_contract.compatibility_name_count(),
            transcript_evidence.unsupported_neighbor_denial_digests().len(),
            transcript_evidence.delivery_residue_count()
        ),
        compile_fail_boundary_digest: compile_fail_boundary_digest(),
        transcript_family,
        public_facade_only: true,
        lower_runtime_plumbing_count: 0,
        meaningful_assertion_count: meaningful_assertion_count
            .max(transcript_evidence.meaningful_assertion_count()),
        unsupported_neighbor_denial_count: transcript_evidence
            .unsupported_neighbor_denial_digests()
            .len(),
        delivery_residue_count: transcript_evidence.delivery_residue_count(),
        stable_family_count: contract.stable_family_count(),
        deferred_family_count: contract.deferred_family_count(),
        unsupported_family_count: contract.unsupported_family_count(),
    }
}

fn contract() -> ForgeQueryRuntimePublicApiContract {
    ForgeQueryRuntimePublicApiContract::from_support_profile(
        &ForgeQueryRuntimeSupportProfile::compatibility_backend(),
    )
}

fn support_matrix_digest(contract: &ForgeQueryRuntimePublicApiContract) -> String {
    digest_parts(
        &contract
            .families()
            .iter()
            .map(|row| {
                format!(
                    "{}:{}:{}",
                    row.family().as_str(),
                    row.status().as_str(),
                    row.contract_digest()
                )
            })
            .collect::<Vec<_>>(),
    )
}

fn deferred_gate_digest(contract: &ForgeQueryRuntimePublicApiContract) -> String {
    digest_parts(
        &contract
            .families()
            .iter()
            .filter(|row| row.status() == ForgeQueryRuntimeFamilySupportStatus::DeferredDebt)
            .map(|row| format!("{}:{}", row.family().as_str(), row.contract_digest()))
            .collect::<Vec<_>>(),
    )
}

fn compile_fail_boundary_digest() -> String {
    digest_parts(&[
        "runtime-public-api-contract-private-fields".to_string(),
        "runtime-public-api-naming-contract-private-fields".to_string(),
        "runtime-state-snapshot-private-fields".to_string(),
        "runtime-public-api-transcript-evidence-private-fields".to_string(),
        "runtime-handle-contract-private-fields".to_string(),
        "runtime-workspace-dynamic-surface-shortcut-forbidden".to_string(),
        "runtime-workspace-handle-value-shortcut-forbidden".to_string(),
        "lower-runtime-plumbing-shortcut-forbidden".to_string(),
    ])
}
