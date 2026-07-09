use worth_query::facade::runtime::{
    WorthQueryGraphReadAccessCase, WorthQueryGraphReadAccessRequirementKind,
};

fn main() {
    let _ = WorthQueryGraphReadAccessCase::for_requirement_kind(
        WorthQueryGraphReadAccessRequirementKind::ResultBuffer,
    );
}
