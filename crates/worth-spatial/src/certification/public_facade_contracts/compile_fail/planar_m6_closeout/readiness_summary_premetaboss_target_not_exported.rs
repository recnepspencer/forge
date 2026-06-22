use worth_spatial::facade::planar_contract_bundle::PlanarM7ReadinessReceipt;
use worth_spatial::facade::planar_m6_closeout::{
    M6PremetabossFamily, M6PremetabossPlatformTarget,
};

fn main() {
}

fn cannot_target_readiness_summary(readiness: &PlanarM7ReadinessReceipt) {
    let _ = M6PremetabossPlatformTarget::from_m7_readiness_receipt(
        M6PremetabossFamily::BooleanReadinessFinalBoss,
        readiness,
    );
}
