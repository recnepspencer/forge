use forge_query::facade::{ForgeQueryDerivedPatch, ForgeQueryDerivedPatchPayload};

fn payload_projection(payload: ForgeQueryDerivedPatchPayload) {
    let _ = payload.terminal_json_projection();
}

fn patch_projection(patch: ForgeQueryDerivedPatch) {
    let _ = patch.terminal_json_payload_projection();
}

fn main() {}
