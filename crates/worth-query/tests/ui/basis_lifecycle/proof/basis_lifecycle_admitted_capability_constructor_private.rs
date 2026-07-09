use worth_query::facade::{AdmittedBasisCapability, ObservationLaneWitness};

fn normalized() -> worth_query::facade::NormalizedBasisIntent {
    unimplemented!()
}

fn lane() -> ObservationLaneWitness {
    unimplemented!()
}

fn main() {
    let _ = AdmittedBasisCapability::<ObservationLaneWitness> {
        normalized: normalized(),
        lane: lane(),
        capability_digest: String::new(),
    };
}
