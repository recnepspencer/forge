use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryEffectPolicy, ForgeQueryPreviewCloseoutKind,
    ForgeQueryPreviewOutcomeInspection,
};

fn main() {
    let _forged = ForgeQueryPreviewOutcomeInspection {
        label: String::new(),
        closeout_kind: ForgeQueryPreviewCloseoutKind::Discarded,
        effect_policy: ForgeQueryEffectPolicy::DeriveOnly,
        promoted: false,
        discarded: true,
        write_count: 0,
        preview_binding_count: 0,
        live_binding_count: 0,
        computed_binding_count: 0,
        effect_binding_count: 0,
        subscription_residue_count: 0,
        derived_runtime_residue_count: 0,
        effect_delivery_residue_count: 0,
        pending_write_intent_residue_count: 0,
        preview_write_staging_count: 0,
        promoted_write_count: 0,
        authoritative_residue_count: 0,
        source_lane: ForgeQueryAuthorityLane::PreviewTruth,
        target_lane: ForgeQueryAuthorityLane::PreviewTruth,
        basis_evidence: Vec::new(),
        basis_digest: String::new(),
        closeout_digest: String::new(),
        residue_digest: String::new(),
        inspection_digest: String::new(),
    };
}
