use worth_query::facade::domain::WorthQuerySharedLiveProjectionLease;
use worth_query::facade::foundation::ObservationLaneWitness;

type Lease = WorthQuerySharedLiveProjectionLease<(), (), (), ObservationLaneWitness>;

#[allow(unreachable_code)]
fn forge_lease() -> Lease {
    Lease {
        source: panic!(),
        proof: panic!(),
        workspace_capability: panic!(),
        token: panic!(),
    }
}

fn main() {}
