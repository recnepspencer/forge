use worth_spatial::facade::nmt_certification_context::NmtScopeProjectionReceipt;

fn main() {
    let _projection = NmtScopeProjectionReceipt {
        parent_projection_identity: "aggregate-projection".to_string(),
        scope_identity: "scope".to_string(),
        scope_projection_identity: "scope-projection".to_string(),
        local_frame_identity: "frame".to_string(),
        consumed_projected_entities: vec!["raw-coordinate-row".to_string()],
        counters: todo!(),
    };
}
