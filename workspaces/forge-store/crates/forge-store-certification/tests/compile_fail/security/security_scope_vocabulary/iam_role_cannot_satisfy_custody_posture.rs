use forge_store_security::{StoreCustodyPosture, StoreIamRoleClaim};

fn require_custody_posture(_: StoreCustodyPosture) {}

fn main() {
    require_custody_posture(StoreIamRoleClaim::raw("arn:aws:iam::123:role/store"));
}
