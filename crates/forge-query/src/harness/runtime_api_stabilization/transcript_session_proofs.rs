use crate::facade::{
    ForgeQueryAuthorityLane, ForgeQueryBranchOptions, ForgeQueryEffectHandle,
    ForgeQueryIntentDeclaration, ForgeQueryIntentInput, ForgeQueryLiveView,
    ForgeQueryPreviewOptions, ForgeQueryRuntimeFacadeFamily, ForgeQueryRuntimePublicApiContract,
    ForgeQueryWorkspace,
};
use crate::identity::hash_parts;
use crate::runtime::ForgeQueryNativeRow;
use crate::ForgeQuerySessionLabel;
use forge_foundational::facade::AspectValue;

use super::transcripts::TranscriptSpec;

pub(super) fn preview_proof(
    workspace: &mut ForgeQueryWorkspace,
    live: &ForgeQueryLiveView<ForgeQueryNativeRow>,
    computed: &crate::facade::ForgeQueryDerivedViewHandle<ForgeQueryNativeRow>,
    effect: &ForgeQueryEffectHandle<ForgeQueryNativeRow>,
    spec: &TranscriptSpec,
) -> (String, usize) {
    let mut preview = workspace
        .preview_with_options(
            ForgeQuerySessionLabel::scoped_strs(
                "runtime-api-stabilization",
                [format!("{}.preview", spec.family)],
            )
            .expect("preview label should build"),
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
            let entity = entity.aspect(
                aspect_touch("identity.id"),
                string_aspect_value(format!("{}-preview-1", spec.family)),
            );
            spec.produced_aspects
                .iter()
                .enumerate()
                .fold(entity, |builder, (index, aspect)| {
                    builder.aspect(
                        aspect_touch(*aspect),
                        string_aspect_value(format!("{}-preview-{index}", spec.family)),
                    )
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

pub(super) fn branch_proof(workspace: &mut ForgeQueryWorkspace, spec: &TranscriptSpec) -> String {
    let mut branch = workspace
        .branch_with_options(
            ForgeQuerySessionLabel::scoped_strs(
                "runtime-api-stabilization",
                [format!("{}.branch", spec.family)],
            )
            .expect("branch label should build"),
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

pub(super) fn support_gated_neighbor_denials(
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

fn binding_digest(binding: &crate::facade::ForgeQueryPreviewHandleBindingEvidence) -> String {
    hash_parts(&[
        format!("label_identity:{}", binding.label_identity().as_str()),
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

pub(super) fn intent_declaration(
    name: impl Into<String>,
    collection: &str,
) -> ForgeQueryIntentDeclaration {
    ForgeQueryIntentDeclaration::strategy_commit(
        name,
        "strategy.intent.transcript",
        "1.0",
        "transcript.intent.input.v1",
        ForgeQueryIntentInput::object([
            ("collection", ForgeQueryIntentInput::string(collection)),
            (
                "entity",
                ForgeQueryIntentInput::string("transcript-entity-1"),
            ),
        ]),
    )
}

fn aspect_touch(aspect_path: impl Into<String>) -> crate::runtime::ForgeQueryAspectTouch {
    crate::runtime::ForgeQueryAspectTouch::from_authoring_path(aspect_path)
        .expect("runtime transcript aspect should admit")
}

fn string_aspect_value(value: impl Into<String>) -> AspectValue {
    AspectValue::String(value.into().into())
}
