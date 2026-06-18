use forge_query::facade::consumer_kit::{
    ForgeQueryProhibitedSeam, ForgeQueryProhibitionEnforcementTier,
    ForgeQueryProhibitionRegistryRow,
};

fn main() {
    let _ = ForgeQueryProhibitionRegistryRow::new(
        ForgeQueryProhibitedSeam::WorkspaceDirectWrite,
        ForgeQueryProhibitionEnforcementTier::PhaseThreeAuditResidue,
        "consumer-owned bypass",
        "external crates must not mint prohibition registry rows",
    );
}
