use forge_query::facade::{
    EffectLifecyclePublicSurfaceRow, EffectPublicSurfaceAvailability, EffectPublicSurfaceKind,
    EffectReceiptArtifactKind,
};

fn main() {
    let _ = EffectLifecyclePublicSurfaceRow {
        surface_kind: EffectPublicSurfaceKind::BatchExecution,
        entrypoint: Some("effect_batch().using_basis(...).admit().lower().execute()"),
        primary_artifact_kind: Some(EffectReceiptArtifactKind::ForgeQueryBatchWriteReceipt),
        availability: EffectPublicSurfaceAvailability::Implemented,
        lower_runtime_visibility_hidden: true,
        row_digest: String::new(),
    };
}
