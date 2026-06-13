use worth_spatial::facade::nmt_certification_context::{NmtBossCloseoutReceipt, NmtBossId};

fn main() {
    let _receipt = NmtBossCloseoutReceipt {
        boss: NmtBossId::OpenRadialFan,
        certified_scope_set_identity: "scope-set".to_string(),
        outcome_count: 5,
    };
}
