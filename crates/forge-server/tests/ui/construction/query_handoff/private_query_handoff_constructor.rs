use forge_server::{ForgeServerQueryHandoff, ForgeServerQueryHandoffOperation, ForgeServerQuerySupportPosture};

fn main() {
    let _ = ForgeServerQueryHandoff {
        admission: loop {},
        operation: ForgeServerQueryHandoffOperation::query_read("users.profile"),
        workspace: loop {},
        downstream_delivery_contract: loop {},
        support_posture: ForgeServerQuerySupportPosture::QueryReadSupported {
            family_contract: loop {},
        },
        canonical_digest: "forged".to_string(),
    };
}
