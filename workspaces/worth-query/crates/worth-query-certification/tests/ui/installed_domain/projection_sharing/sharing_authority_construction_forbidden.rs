use worth_query::facade::domain::WorthQueryAdmittedProjectionSharing;
use worth_query::runtime::{
    WorthQuerySharedExecutionOwnerIdentity, WorthQuerySharedProjectionLeaseIdentity,
    WorthQuerySharedProjectionLeaseToken,
};

fn main() {
    let owner = WorthQuerySharedExecutionOwnerIdentity::new(1, 2, 3);
    let lease = WorthQuerySharedProjectionLeaseIdentity::new(1, 4, 5);
    let _token = WorthQuerySharedProjectionLeaseToken::new(owner, lease);
    let _admission = WorthQueryAdmittedProjectionSharing {};
}
