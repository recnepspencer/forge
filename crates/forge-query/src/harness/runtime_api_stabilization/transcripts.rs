use serde_json::{json, Value};

use crate::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryDerivedPatch,
    ForgeQueryDerivedView, ForgeQueryDerivedViewMaintainer, ForgeQueryDerivedViewMaterialization,
    ForgeQueryEffectHandle, ForgeQueryInspection, ForgeQueryIntentDeclaration, ForgeQueryLiveView,
    ForgeQueryMutationDelta, ForgeQueryPreviewOptions, ForgeQueryRuntimeFacadeFamily,
    ForgeQueryRuntimePublicApiContract, ForgeQueryRuntimePublicApiTranscriptEvidence,
    ForgeQueryWorkspace,
};
use crate::identity::hash_parts;

use super::transcript_runtime::transcript_runtime;

pub(super) fn workflow_editor_transcript() -> ForgeQueryRuntimePublicApiTranscriptEvidence {
    execute_transcript(TranscriptSpec {
        family: "workflow-editor",
        collection: "WorkflowNode",
        view_name: "workflow.editor.canvas",
        first_computed: "workflow.editor.section-readiness",
        second_computed: "workflow.editor.workflow-readiness",
        effect_name: "workflow.editor.publish-readiness",
        intent_name: "workflow.editor.commit-intent",
        produced_aspects: &["validation.state", "layout.frame", "runtimeValue.preview"],
        neighbor_families: &[ForgeQueryRuntimeFacadeFamily::AsyncResource],
        assertion_floor: 12,
    })
}

pub(super) fn geometry_kernel_transcript() -> ForgeQueryRuntimePublicApiTranscriptEvidence {
    execute_transcript(TranscriptSpec {
        family: "geometry-kernel",
        collection: "ModelEntity",
        view_name: "geometry.kernel.topology",
        first_computed: "geometry.kernel.nurbs-evaluation",
        second_computed: "geometry.kernel.fillet-solver-readiness",
        effect_name: "geometry.kernel.persist-solver-output",
        intent_name: "geometry.kernel.commit-solve",
        produced_aspects: &["topology.edge", "surface.trim", "solver.residue"],
        neighbor_families: &[ForgeQueryRuntimeFacadeFamily::Temporal],
        assertion_floor: 12,
    })
}

pub(super) fn table_spreadsheet_transcript() -> ForgeQueryRuntimePublicApiTranscriptEvidence {
    execute_transcript(TranscriptSpec {
        family: "table-spreadsheet",
        collection: "SheetCell",
        view_name: "table.sheet.visible-window",
        first_computed: "table.sheet.formula-values",
        second_computed: "table.sheet.dropdown-domain",
        effect_name: "table.sheet.persist-batched-edit",
        intent_name: "table.sheet.commit-edit",
        produced_aspects: &["formula.value", "dropdown.domain", "layout.width"],
        neighbor_families: &[ForgeQueryRuntimeFacadeFamily::MixedCauseDelivery],
        assertion_floor: 12,
    })
}

pub(super) fn composed_runtime_transcript() -> ForgeQueryRuntimePublicApiTranscriptEvidence {
    execute_transcript(TranscriptSpec {
        family: "composed-runtime",
        collection: "ComposedNode",
        view_name: "composed.runtime.surface",
        first_computed: "composed.runtime.derived-a",
        second_computed: "composed.runtime.derived-b",
        effect_name: "composed.runtime.pending-intent-effect",
        intent_name: "composed.runtime.commit-intent",
        produced_aspects: &[
            "invariant.state",
            "branch.preview",
            "expression.output",
            "intent.commit",
        ],
        neighbor_families: &[
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
        ],
        assertion_floor: 16,
    })
}

pub(super) fn composed_runtime_hostile_transcript() -> ForgeQueryRuntimePublicApiTranscriptEvidence
{
    execute_transcript(TranscriptSpec {
        family: "composed-runtime-hostile",
        collection: "ComposedNode",
        view_name: "composed.runtime.surface.hostile",
        first_computed: "composed.runtime.derived-a.hostile",
        second_computed: "composed.runtime.derived-b.hostile",
        effect_name: "composed.runtime.pending-intent-effect.hostile",
        intent_name: "composed.runtime.commit-intent.hostile",
        produced_aspects: &[
            "invariant.state",
            "branch.preview",
            "expression.output",
            "intent.commit",
            "temporal.denial",
            "async.denial",
        ],
        neighbor_families: &[
            ForgeQueryRuntimeFacadeFamily::Temporal,
            ForgeQueryRuntimeFacadeFamily::AsyncResource,
            ForgeQueryRuntimeFacadeFamily::StoreBackedExecution,
        ],
        assertion_floor: 18,
    })
}

struct TranscriptSpec {
    family: &'static str,
    collection: &'static str,
    view_name: &'static str,
    first_computed: &'static str,
    second_computed: &'static str,
    effect_name: &'static str,
    intent_name: &'static str,
    produced_aspects: &'static [&'static str],
    neighbor_families: &'static [ForgeQueryRuntimeFacadeFamily],
    assertion_floor: usize,
}

fn execute_transcript(spec: TranscriptSpec) -> ForgeQueryRuntimePublicApiTranscriptEvidence {
    let mut workspace = transcript_runtime()
        .workspace(format!("{}.workspace", spec.family))
        .expect("transcript runtime should expose a named workspace");
    let live = workspace
        .live_view::<Value>(spec.view_name, |q| {
            q.from(spec.collection)
                .select(
                    std::iter::once("identity.id".to_string()).chain(
                        spec.produced_aspects
                            .iter()
                            .map(|aspect| (*aspect).to_string()),
                    ),
                )
                .order_by("identity.id")
                .schema_basis(format!("runtime-transcript-{}", spec.family))
                .as_surface(spec.view_name)
        })
        .expect("transcript live surface should declare");
    let first = workspace
        .computed::<Value>(
            spec.first_computed,
            |c| {
                c.depends_on_live(&live)
                    .reads(
                        spec.produced_aspects
                            .iter()
                            .map(|aspect| (*aspect).to_string()),
                    )
                    .produces(
                        spec.produced_aspects
                            .iter()
                            .map(|aspect| format!("{aspect}.derived")),
                    )
            },
            TranscriptMaintainer {
                prefix: spec.family,
                replace: false,
            },
        )
        .expect("first computed transcript surface should declare");
    let second = workspace
        .computed::<Value>(
            spec.second_computed,
            |c| {
                c.depends_on_computed(&first)
                    .reads(
                        spec.produced_aspects
                            .iter()
                            .map(|aspect| format!("{aspect}.derived")),
                    )
                    .produces(
                        spec.produced_aspects
                            .iter()
                            .map(|aspect| format!("{aspect}.ready")),
                    )
            },
            TranscriptMaintainer {
                prefix: spec.family,
                replace: true,
            },
        )
        .expect("nested computed transcript surface should declare");
    let effect = workspace
        .effect::<Value>(spec.effect_name, |e| {
            e.when_computed(
                &second,
                spec.produced_aspects
                    .iter()
                    .map(|aspect| format!("{aspect}.ready")),
            )
            .condition_expression(
                format!("{}.custom-expression", spec.family),
                spec.produced_aspects
                    .iter()
                    .map(|aspect| format!("{aspect}.ready")),
                [format!("{}.effect-output", spec.family)],
            )
            .write_intent("strategy.intent.transcript")
            .meaningful_change_suppression()
        })
        .expect("write-intent effect transcript surface should declare");

    let (preview_digest, preview_residue_count) =
        preview_proof(&mut workspace, &live, &second, &effect, &spec);
    let intent = workspace
        .intent(intent_declaration(spec.intent_name, spec.collection))
        .expect("authoritative transcript intent should commit");
    let write = workspace
        .insert(spec.collection, |entity| {
            let entity = entity.aspect("identity.id", format!("{}-entity-1", spec.family));
            spec.produced_aspects
                .iter()
                .enumerate()
                .fold(entity, |builder, (index, aspect)| {
                    builder.aspect(*aspect, format!("{}-value-{index}", spec.family))
                })
        })
        .expect("transcript write should route live, computed, and effect artifacts");
    assert!(
        write.pending_write_intent_count() >= 1,
        "transcript effect should leave a pending write intent"
    );
    assert!(
        write
            .affected_live_view_ids()
            .iter()
            .any(|view| view == spec.view_name),
        "transcript write should affect its durable live view"
    );
    let patches = workspace.observe(&live);
    assert!(
        !patches.query_delivery_batches.is_empty(),
        "transcript live surface should emit query delivery batches"
    );
    let live_inspection = match workspace
        .inspect(&live)
        .expect("live transcript surface should inspect")
    {
        ForgeQueryInspection::LiveView(inspection) => inspection,
        other => panic!("expected live view inspection, got {other:?}"),
    };
    let live_installation_digest = live_inspection.installation_digest().to_string();
    let live_active_lane_digest = live_inspection.active_lane_digest().to_string();
    let computed_inspection = match workspace
        .inspect(&second)
        .expect("nested computed transcript surface should inspect")
    {
        ForgeQueryInspection::DerivedView(inspection) => inspection,
        other => panic!("expected derived view inspection, got {other:?}"),
    };
    assert!(
        computed_inspection.pending_incremental_patch_count() >= 1,
        "nested computed should retain incremental patch evidence"
    );
    let effect_inspection = match workspace
        .inspect(&effect)
        .expect("effect transcript surface should inspect")
    {
        ForgeQueryInspection::Effect(inspection) => inspection,
        other => panic!("expected effect inspection, got {other:?}"),
    };
    assert!(
        effect_inspection.pending_write_intent_count() >= 1,
        "effect inspection should expose pending intent residue"
    );
    let effect_intent = workspace
        .next_effect_intent(&effect, "1.0", "transcript.effect.intent.v1")
        .expect("effect pending intent should execute through intent authority");
    let effect_intent_inspection = match workspace
        .inspect(&effect_intent)
        .expect("effect intent receipt should inspect")
    {
        ForgeQueryInspection::EffectIntentReceipt(inspection) => inspection,
        other => panic!("expected effect intent receipt inspection, got {other:?}"),
    };
    let branch_digest = branch_proof(&mut workspace, &spec);
    let intent_inspection = match workspace
        .inspect(&intent)
        .expect("authoritative intent receipt should inspect")
    {
        ForgeQueryInspection::IntentReceipt(inspection) => inspection,
        other => panic!("expected intent receipt inspection, got {other:?}"),
    };
    let support_contract = workspace.public_api_contract();
    let denial_digests = unsupported_neighbor_denials(&support_contract, spec.neighbor_families);
    let state = workspace
        .state(&second)
        .expect("golden transcript should use the public state boundary");
    let authority_lane_digest = hash_parts(&[
        format!("live:{live_active_lane_digest}"),
        format!("computed:{}", computed_inspection.produced_aspect_digest()),
        format!("effect:{}", effect_inspection.trigger_digest()),
        format!("preview:{preview_digest}"),
        format!("branch:{branch_digest}"),
        format!(
            "effect_intent:{}",
            effect_intent_inspection.inspection_digest()
        ),
    ]);
    ForgeQueryRuntimePublicApiTranscriptEvidence::new(
        spec.family,
        support_contract.contract_digest(),
        state.state_digest(),
        live_installation_digest,
        computed_inspection.inspection_digest(),
        effect_inspection.inspection_digest(),
        intent.receipt_digest(),
        hash_parts(&[
            intent_inspection.inspection_digest().to_string(),
            effect_intent_inspection.inspection_digest().to_string(),
            branch_digest,
        ]),
        denial_digests,
        preview_residue_count + write.pending_write_intent_count(),
        authority_lane_digest,
        spec.assertion_floor,
    )
}

fn preview_proof(
    workspace: &mut ForgeQueryWorkspace,
    live: &ForgeQueryLiveView<Value>,
    computed: &crate::facade::ForgeQueryDerivedViewHandle<Value>,
    effect: &ForgeQueryEffectHandle<Value>,
    spec: &TranscriptSpec,
) -> (String, usize) {
    let mut preview = workspace
        .preview_with_options(
            format!("{}.preview", spec.family),
            ForgeQueryPreviewOptions::sandboxed_write_intent(),
        )
        .expect("transcript preview should open");
    let live_binding = preview.use_view(live);
    let computed_binding = preview.use_computed(computed);
    let effect_binding = preview
        .use_effect(effect)
        .expect("effect should bind in preview");
    let preview_write = preview
        .insert(spec.collection, |entity| {
            let entity = entity.aspect("identity.id", format!("{}-preview-1", spec.family));
            spec.produced_aspects
                .iter()
                .enumerate()
                .fold(entity, |builder, (index, aspect)| {
                    builder.aspect(*aspect, format!("{}-preview-{index}", spec.family))
                })
        })
        .expect("preview write should stage");
    let preview_intent = preview
        .execute_intent(intent_declaration(
            format!("{}.preview-intent", spec.intent_name),
            spec.collection,
        ))
        .expect("preview intent should stage");
    let outcome = preview.discard();
    assert_eq!(
        preview_write.authority_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        preview_intent.target_lane(),
        ForgeQueryAuthorityLane::PreviewTruth
    );
    assert_eq!(
        outcome.authoritative_residue_count(),
        0,
        "preview transcript must not leak authoritative writes"
    );
    assert!(
        outcome.preview_binding_count() >= 3,
        "preview should bind live, computed, and effect handles"
    );
    (
        hash_parts(&[
            binding_digest(&live_binding),
            binding_digest(&computed_binding),
            binding_digest(&effect_binding),
            preview_intent.receipt_digest().to_string(),
            outcome.closeout_evidence().closeout_digest().to_string(),
        ]),
        outcome.write_count() + outcome.pending_write_intent_residue_count(),
    )
}

fn binding_digest(binding: &crate::facade::ForgeQueryPreviewHandleBindingEvidence) -> String {
    hash_parts(&[
        format!("label:{}", binding.label()),
        format!("handle:{}", binding.handle_name()),
        format!("family:{}", binding.family().as_str()),
        format!("source:{}", binding.source_lane().as_str()),
        format!("preview:{}", binding.preview_lane().as_str()),
        format!("policy:{}", binding.effect_policy().as_str()),
        format!(
            "disposition:{}",
            binding
                .effect_disposition()
                .map(|disposition| disposition.as_str())
                .unwrap_or("none")
        ),
    ])
}

fn branch_proof(workspace: &mut ForgeQueryWorkspace, spec: &TranscriptSpec) -> String {
    let mut branch = workspace
        .branch_with_options(
            format!("{}.branch", spec.family),
            ForgeQueryBranchOptions::sandboxed_write_intent(),
        )
        .expect("transcript branch should open");
    let receipt = branch
        .execute_intent(intent_declaration(
            format!("{}.branch-intent", spec.intent_name),
            spec.collection,
        ))
        .expect("branch intent should stay branch-local");
    assert_eq!(
        receipt.target_lane(),
        ForgeQueryAuthorityLane::BranchLocalTruth
    );
    assert_eq!(branch.branch_intent_receipts().len(), 1);
    receipt.receipt_digest().to_string()
}

fn unsupported_neighbor_denials(
    contract: &ForgeQueryRuntimePublicApiContract,
    families: &[ForgeQueryRuntimeFacadeFamily],
) -> Vec<String> {
    families
        .iter()
        .map(|family| {
            let row = contract
                .family(*family)
                .expect("future neighbor family should have contract row");
            assert!(
                row.status() != crate::facade::ForgeQueryRuntimeFamilySupportStatus::Supported,
                "future neighbor must not be silently admitted in public stabilization transcripts"
            );
            hash_parts(&[
                format!("family:{}", family.as_str()),
                format!("status:{}", row.status().as_str()),
                format!("reason:{}", row.reason().unwrap_or("none")),
                format!("contract:{}", row.contract_digest()),
            ])
        })
        .collect()
}

fn intent_declaration(name: impl Into<String>, collection: &str) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "strategy.intent.transcript",
        "1.0",
        "transcript.intent.input.v1",
        json!({ "collection": collection, "entity": "transcript-entity-1" }),
    )
}

struct TranscriptMaintainer {
    prefix: &'static str,
    replace: bool,
}

impl ForgeQueryDerivedViewMaintainer for TranscriptMaintainer {
    fn maintain(
        &mut self,
        view: &ForgeQueryDerivedView,
        delta: &ForgeQueryMutationDelta,
        materialization: &mut ForgeQueryDerivedViewMaterialization,
    ) -> ForgeQueryDerivedPatch {
        let row = json!({
            "family": self.prefix,
            "entity": delta.entity_identity,
            "view": view.name(),
        });
        if self.replace {
            materialization.replace_rows([row.clone()]);
        } else {
            materialization.push_row(row.clone());
        }
        ForgeQueryDerivedPatch::incremental(
            view.name(),
            format!("transcript-derived-commit:{}", self.prefix),
            delta.entity_identity.clone(),
            if view.produced_aspects().is_empty() {
                delta.aspect_paths.clone()
            } else {
                view.produced_aspects().to_vec()
            },
            row,
        )
    }
}
