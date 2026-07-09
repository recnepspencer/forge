use worth_query::facade::consumer_kit::{
    WorthQueryProhibitedSeam, WorthQueryProhibitionEnforcementTier,
    WorthQueryProhibitionRegistryRow,
};

fn main() {
    let _ = WorthQueryProhibitionRegistryRow::new(
        WorthQueryProhibitedSeam::WorkspaceDirectWrite,
        WorthQueryProhibitionEnforcementTier::PhaseThreeAuditResidue,
        "consumer-owned bypass",
        "external crates must not mint prohibition registry rows",
    );
}
