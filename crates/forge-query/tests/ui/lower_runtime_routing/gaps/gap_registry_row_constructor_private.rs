use forge_query::facade::runtime::{
    ForgeQueryLowerRuntimeAuthorityOwner, ForgeQueryLowerRuntimeGapRegistryRow,
    ForgeQueryLowerRuntimeSeamKey,
};

fn main() {
    let _ = ForgeQueryLowerRuntimeGapRegistryRow::new(
        ForgeQueryLowerRuntimeSeamKey::FrontierEvidenceIntake,
        "forged-seam",
        "forged-shape",
        ForgeQueryLowerRuntimeAuthorityOwner::Signal,
        "forged-contract",
        "forged-closeout",
    );
}
