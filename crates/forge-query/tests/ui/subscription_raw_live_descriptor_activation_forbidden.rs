use forge_query::facade::{prepare_subscription_activation, LiveQueryAdmissionArtifact};

fn main() {
    let raw_live = Option::<LiveQueryAdmissionArtifact>::None.unwrap();
    let _activation = prepare_subscription_activation(raw_live);
}
