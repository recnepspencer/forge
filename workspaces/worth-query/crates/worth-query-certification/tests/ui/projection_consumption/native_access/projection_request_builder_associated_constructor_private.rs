use worth_query::facade::domain::WorthQueryProjectionRequestBuilder;
use worth_query::facade::foundation::ObservationLaneWitness;

fn main() {
    let _ = WorthQueryProjectionRequestBuilder::<(), (), (), ObservationLaneWitness>::new();
}
