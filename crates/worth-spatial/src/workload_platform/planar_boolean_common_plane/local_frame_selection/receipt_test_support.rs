#[path = "receipt_test_support/geometry_receipts.rs"]
mod geometry_receipts;
#[path = "receipt_test_support/handles.rs"]
mod handles;
#[path = "receipt_test_support/readiness_bundle.rs"]
mod readiness_bundle;
#[path = "receipt_test_support/shared_plane_identity.rs"]
mod shared_plane_identity;

pub(super) use readiness_bundle::readiness_receipt;
pub(super) use shared_plane_identity::shared_plane_identity_receipt;

const WORLD: &str = "planar-boolean-common-plane-local-frame-selection-tests";
const TOPOLOGY: &str = "topology:local-frame-test";
const MOVEMENT: &str = "movement:local-frame-test";
const NEIGHBORHOOD: &str = "neighborhood:local-frame-test";
