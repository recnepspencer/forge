use worth_query::facade::domain::WorthQuerySharedLiveProjectionLease;
use worth_query::facade::foundation::ObservationLaneWitness;

type Lease = WorthQuerySharedLiveProjectionLease<(), (), (), ObservationLaneWitness>;

fn require_clone<T: Clone>() {}

fn main() {
    require_clone::<Lease>();
}
