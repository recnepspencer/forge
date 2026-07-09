use worth_query::facade::{WorthQueryDerivedPatch, WorthQueryDerivedPatchPayload};

fn payload_projection(payload: WorthQueryDerivedPatchPayload) {
    let _ = payload.terminal_json_projection();
}

fn patch_projection(patch: WorthQueryDerivedPatch) {
    let _ = patch.terminal_json_payload_projection();
}

fn main() {}
