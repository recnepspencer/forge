use forge_query::facade::runtime::{
    ForgeQueryGraphReadAccessCase, ForgeQueryGraphReadAccessRequirementKind,
};

fn main() {
    let _ = ForgeQueryGraphReadAccessCase::for_requirement_kind(
        ForgeQueryGraphReadAccessRequirementKind::ResultBuffer,
    );
}
