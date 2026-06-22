use crate::harness::certification::{
    CanonicalCertificationRow, HostileExpectation, ParityAnchor, RejectionCertificationRow,
};
use crate::runtime::{
    ForgeQueryAspectTouch, ForgeQueryAuthorityLane, ForgeQueryLiveView, ForgeQueryNativeRow,
    ForgeQueryPreviewOptions, ForgeQueryWriteReceipt,
};
use crate::ForgeQuerySessionLabel;
use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};
use forge_foundational::facade::AspectValue;

use super::digests::{inspection_digest, touched_aspect_digest};
use super::fixture::{
    stateful_bridge_task_runtime, task_live_view, CertificationTitleListMaintainer,
};
use super::rejections::{duplicate_aspect_authoring_rejection, unsupported_intent_rejection};
use super::{
    AspectApiFinalizationCertificationBundle, AspectApiFinalizationPerturbationClass,
    AspectApiFinalizationRejectionBundle,
};

pub(super) fn canonical_rows() -> Vec<
    CanonicalCertificationRow<
        AspectApiFinalizationPerturbationClass,
        AspectApiFinalizationCertificationBundle,
    >,
> {
    vec![
        CanonicalCertificationRow {
            row_name: "authoritative-insert-update-delete-surface",
            perturbation_class: AspectApiFinalizationPerturbationClass::AuthoritativeCrudSurface,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: authoritative_crud_lane("task-1", "Buy milk", "Buy oat milk"),
            hostile_lane: authoritative_crud_lane("task-2", "Read docs", "Read better docs"),
            parity_lane: authoritative_crud_lane("task-3", "Pay bills", "Pay rent"),
        },
        CanonicalCertificationRow {
            row_name: "typed-clear-narrows-by-touched-meaning",
            perturbation_class: AspectApiFinalizationPerturbationClass::TypedClearNarrowing,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: clear_lane("description.value"),
            hostile_lane: clear_lane("title.value"),
            parity_lane: clear_lane("title.value"),
        },
        CanonicalCertificationRow {
            row_name: "preview-batch-lane-isolation",
            perturbation_class: AspectApiFinalizationPerturbationClass::PreviewBatchIsolation,
            hostile_expectation: HostileExpectation::DistinctFromControl,
            parity_anchor: ParityAnchor::Hostile,
            control_lane: authoritative_batch_lane(),
            hostile_lane: preview_batch_lane(),
            parity_lane: preview_batch_lane(),
        },
        CanonicalCertificationRow {
            row_name: "mutation-surface-closeout-contract-sync",
            perturbation_class: AspectApiFinalizationPerturbationClass::MutationSurfaceCloseoutSync,
            hostile_expectation: HostileExpectation::EquivalentToControl,
            parity_anchor: ParityAnchor::Control,
            control_lane: mutation_surface_contract_lane(),
            hostile_lane: mutation_surface_contract_lane(),
            parity_lane: mutation_surface_contract_lane(),
        },
    ]
}

pub(super) fn rejection_rows() -> Vec<
    RejectionCertificationRow<
        AspectApiFinalizationPerturbationClass,
        AspectApiFinalizationCertificationBundle,
        AspectApiFinalizationRejectionBundle,
    >,
> {
    vec![
        RejectionCertificationRow {
            row_name: "unsupported-intent-family-fails-typed-and-early",
            perturbation_class:
                AspectApiFinalizationPerturbationClass::UnsupportedIntentFamilyDenied,
            control_lane: mutation_surface_contract_lane(),
            hostile_lane: unsupported_intent_rejection(),
            parity_lane: mutation_surface_contract_lane(),
        },
        RejectionCertificationRow {
            row_name: "duplicate-clear-and-set-denied-before-routing",
            perturbation_class:
                AspectApiFinalizationPerturbationClass::DuplicateAspectAuthoringDenied,
            control_lane: mutation_surface_contract_lane(),
            hostile_lane: duplicate_aspect_authoring_rejection(),
            parity_lane: mutation_surface_contract_lane(),
        },
    ]
}

fn authoritative_crud_lane(
    entity_id: &str,
    initial_title: &str,
    renamed_title: &str,
) -> AspectApiFinalizationCertificationBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.crud")
        .expect("workspace should open");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> =
        task_live_view(&mut workspace, "tasks.cert-crud");

    let seed = workspace
        .insert("Task", |task| {
            task.aspect(aspect_touch("identity.id"), string_aspect_value(entity_id))
                .aspect(
                    aspect_touch("title.value"),
                    string_aspect_value(initial_title),
                )
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);
    let update = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.aspect(
                aspect_touch("title.value"),
                string_aspect_value(renamed_title),
            )
        })
        .expect("rename should execute");
    let _patches = workspace.observe(&live);
    let delete = workspace
        .delete_with(seed.deltas()[0].entity_identity.clone(), |task| {
            task.touch(aspect_touch("identity.id"))
                .touch(aspect_touch("title.value"))
        })
        .expect("delete should execute");
    let state = workspace.state(&delete).expect("state should resolve");
    let inspection = workspace
        .inspect(&delete)
        .expect("inspection should resolve");

    AspectApiFinalizationCertificationBundle {
        mutation_surface_label: "workspace.insert/update/delete".to_string(),
        authority_lane_label: delete.authority_lane().to_string(),
        mutation_family_label: delete.mutation_family().to_string(),
        support_matrix_digest: workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        mutation_surface_report_digest: workspace
            .public_mutation_surface_report()
            .report_digest()
            .to_string(),
        closeout_digest: workspace
            .public_aspect_api_finalization_closeout()
            .closeout_digest()
            .to_string(),
        receipt_digest: write_receipt_digest(&delete),
        state_digest: state
            .state_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        inspection_digest: inspection_digest(&inspection),
        touched_aspect_digest: touched_aspect_digest(update.deltas()[0].admitted_touched_aspects()),
        affected_live_view_count: delete.affected_live_view_targets().len(),
        affected_derived_view_count: delete.affected_derived_view_targets().len(),
        routed_patch_count: 1,
        materialized_row_count: 0,
        preview_residue_count: 0,
    }
}

fn clear_lane(aspect_path: &str) -> AspectApiFinalizationCertificationBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.clear")
        .expect("workspace should open");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> =
        task_live_view(&mut workspace, "tasks.cert-clear");
    let seed = workspace
        .insert("Task", |task| {
            task.aspect(aspect_touch("identity.id"), string_aspect_value("task-1"))
                .aspect(aspect_touch("title.value"), string_aspect_value("Buy milk"))
                .aspect(
                    aspect_touch("description.value"),
                    string_aspect_value("whole milk"),
                )
        })
        .expect("seed insert should execute");
    let _ = workspace.observe(&live);

    let receipt = workspace
        .update(seed.deltas()[0].entity_identity.clone(), |task| {
            task.clear(aspect_touch(aspect_path))
        })
        .expect("clear should execute");
    let patches = workspace.observe(&live);
    let state = workspace.state(&receipt).expect("state should resolve");
    let inspection = workspace
        .inspect(&receipt)
        .expect("inspection should resolve");

    AspectApiFinalizationCertificationBundle {
        mutation_surface_label: "workspace.update.clear".to_string(),
        authority_lane_label: receipt.authority_lane().to_string(),
        mutation_family_label: receipt.mutation_family().to_string(),
        support_matrix_digest: workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        mutation_surface_report_digest: workspace
            .public_mutation_surface_report()
            .report_digest()
            .to_string(),
        closeout_digest: workspace
            .public_aspect_api_finalization_closeout()
            .closeout_digest()
            .to_string(),
        receipt_digest: write_receipt_digest(&receipt),
        state_digest: state
            .state_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        inspection_digest: inspection_digest(&inspection),
        touched_aspect_digest: touched_aspect_digest(
            receipt.deltas()[0].admitted_touched_aspects(),
        ),
        affected_live_view_count: receipt.affected_live_view_targets().len(),
        affected_derived_view_count: receipt.affected_derived_view_targets().len(),
        routed_patch_count: patches.query_delivery_batches.len(),
        materialized_row_count: workspace.read(&live).len(),
        preview_residue_count: 0,
    }
}

fn authoritative_batch_lane() -> AspectApiFinalizationCertificationBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.batch")
        .expect("workspace should open");
    let live: ForgeQueryLiveView<ForgeQueryNativeRow> =
        task_live_view(&mut workspace, "tasks.cert-batch");
    let computed = workspace
        .computed::<ForgeQueryNativeRow>(
            "tasks.cert-batch-summary",
            |c| {
                c.depends_on_live(&live)
                    .reads([aspect_touch("title.value")])
                    .produces([aspect_touch("ui.batch_summary")])
            },
            CertificationTitleListMaintainer,
        )
        .expect("computed should declare");
    let receipt = workspace
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect(aspect_touch("identity.id"), string_aspect_value("task-1"))
                        .aspect(aspect_touch("title.value"), string_aspect_value("Buy milk"))
                })
                .insert("Task", |task| {
                    task.aspect(aspect_touch("identity.id"), string_aspect_value("task-2"))
                        .aspect(
                            aspect_touch("title.value"),
                            string_aspect_value("Buy bread"),
                        )
                })
        })
        .expect("batch should execute");
    let patches = workspace.observe(&live);
    let materialization = workspace
        .materialize_result(&computed)
        .expect("aspect API certification materialization should execute");
    let state = workspace.state(&receipt).expect("state should resolve");
    let inspection = workspace
        .inspect(&receipt)
        .expect("inspection should resolve");

    AspectApiFinalizationCertificationBundle {
        mutation_surface_label: "workspace.batch".to_string(),
        authority_lane_label: receipt.authority_lane().to_string(),
        mutation_family_label: "batch".to_string(),
        support_matrix_digest: workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        mutation_surface_report_digest: workspace
            .public_mutation_surface_report()
            .report_digest()
            .to_string(),
        closeout_digest: workspace
            .public_aspect_api_finalization_closeout()
            .closeout_digest()
            .to_string(),
        receipt_digest: receipt.batch_digest().to_string(),
        state_digest: state
            .state_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        inspection_digest: inspection_digest(&inspection),
        touched_aspect_digest: touched_aspect_digest(receipt.admitted_touched_aspects()),
        affected_live_view_count: receipt.affected_live_view_targets().len(),
        affected_derived_view_count: receipt.affected_derived_view_targets().len(),
        routed_patch_count: patches.query_delivery_batches.len(),
        materialized_row_count: materialization.row_count(),
        preview_residue_count: 0,
    }
}

fn aspect_touch(aspect_path: &str) -> ForgeQueryAspectTouch {
    ForgeQueryAspectTouch::from_authoring_path(aspect_path)
        .expect("aspect API certification aspect path should admit")
}

fn string_aspect_value(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}

fn preview_batch_lane() -> AspectApiFinalizationCertificationBundle {
    let mut workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.preview-batch")
        .expect("workspace should open");
    let mut preview = workspace
        .preview_with_options(
            ForgeQuerySessionLabel::scoped_strs(
                "aspect-api-finalization-certification",
                ["preview-batch"],
            )
            .expect("preview label should build"),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("preview should open");
    let receipt = preview
        .batch(|batch| {
            batch
                .insert("Task", |task| {
                    task.aspect(
                        aspect_touch("identity.id"),
                        string_aspect_value("preview-task-1"),
                    )
                    .aspect(
                        aspect_touch("title.value"),
                        string_aspect_value("Preview title one"),
                    )
                })
                .insert("Task", |task| {
                    task.aspect(
                        aspect_touch("identity.id"),
                        string_aspect_value("preview-task-2"),
                    )
                    .aspect(
                        aspect_touch("title.value"),
                        string_aspect_value("Preview title two"),
                    )
                })
        })
        .expect("preview batch should stage");
    let outcome = preview.discard();
    let inspection = workspace
        .inspect(&receipt)
        .expect("inspection should resolve");
    let state = workspace.state(&receipt).expect("state should resolve");

    AspectApiFinalizationCertificationBundle {
        mutation_surface_label: "preview.batch".to_string(),
        authority_lane_label: receipt.authority_lane().to_string(),
        mutation_family_label: "batch".to_string(),
        support_matrix_digest: workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        mutation_surface_report_digest: workspace
            .public_mutation_surface_report()
            .report_digest()
            .to_string(),
        closeout_digest: workspace
            .public_aspect_api_finalization_closeout()
            .closeout_digest()
            .to_string(),
        receipt_digest: receipt.batch_digest().to_string(),
        state_digest: state
            .state_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        inspection_digest: inspection_digest(&inspection),
        touched_aspect_digest: touched_aspect_digest(receipt.admitted_touched_aspects()),
        affected_live_view_count: receipt.affected_live_view_targets().len(),
        affected_derived_view_count: receipt.affected_derived_view_targets().len(),
        routed_patch_count: 0,
        materialized_row_count: 0,
        preview_residue_count: outcome.pending_write_intent_residue_count(),
    }
}

fn mutation_surface_contract_lane() -> AspectApiFinalizationCertificationBundle {
    let workspace = stateful_bridge_task_runtime()
        .workspace("aspect-api.contract")
        .expect("workspace should open");
    let report = workspace.public_mutation_surface_report();
    let closeout = workspace.public_aspect_api_finalization_closeout();

    AspectApiFinalizationCertificationBundle {
        mutation_surface_label:
            "workspace.public_mutation_surface_report/public_aspect_api_finalization_closeout"
                .to_string(),
        authority_lane_label: ForgeQueryAuthorityLane::AuthoritativeTruth.to_string(),
        mutation_family_label: "contract".to_string(),
        support_matrix_digest: workspace
            .public_support_matrix()
            .matrix_digest()
            .terminal_projection_for_reporting()
            .to_string(),
        mutation_surface_report_digest: report.report_digest().to_string(),
        closeout_digest: closeout.closeout_digest().to_string(),
        receipt_digest: report.report_digest().to_string(),
        state_digest: closeout.closeout_digest().to_string(),
        inspection_digest: crate::harness::certification::digest_parts(&[
            report.report_digest().to_string(),
            closeout.closeout_digest().to_string(),
        ]),
        touched_aspect_digest: crate::harness::certification::digest_parts(
            &closeout
                .preferred_stable_surfaces()
                .iter()
                .map(|surface| format!("surface:{surface}"))
                .collect::<Vec<_>>(),
        ),
        affected_live_view_count: report.preferred_stable_count(),
        affected_derived_view_count: report.lower_level_stable_count(),
        routed_patch_count: report.lower_level_stable_count(),
        materialized_row_count: report.support_gated_count(),
        preview_residue_count: 0,
    }
}

fn write_receipt_digest(receipt: &ForgeQueryWriteReceipt) -> String {
    ForgeQueryEvidenceIdentity::compose(ForgeQueryEvidenceScope::WriteReceiptInspectionArtifact)
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("commit_evidence_identity"),
            receipt.commit_evidence_identity(),
        )
        .field_evidence_identity(
            ForgeQueryEvidenceTag::new("snapshot_evidence_identity"),
            receipt.snapshot_evidence_identity(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("mutation_family"),
            receipt.mutation_family().as_str(),
        )
        .field_shape(
            ForgeQueryEvidenceTag::new("authority_lane"),
            receipt.authority_lane().as_str(),
        )
        .seal()
        .terminal_projection_for_reporting()
        .to_string()
}
