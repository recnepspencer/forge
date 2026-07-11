use forge_store_security::{StoreCustodyPosture, StoreLegacySecurityPosture};

fn require_custody_posture(_: StoreCustodyPosture) {}

fn main() {
    require_custody_posture(StoreLegacySecurityPosture::LegacyUnscoped);
}
