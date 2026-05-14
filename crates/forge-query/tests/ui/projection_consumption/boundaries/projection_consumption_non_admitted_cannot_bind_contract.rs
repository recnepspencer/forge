use forge_query::facade::foundation::{
    DeferredProjectionConsumption, ProjectionConsumptionEligibility,
};

fn impossible<T>() -> T {
    panic!("fixture should fail before construction")
}

fn main() {
    let eligibility = ProjectionConsumptionEligibility::Deferred(impossible::<
        DeferredProjectionConsumption,
    >());
    let _ = eligibility.bind_contract();
}
