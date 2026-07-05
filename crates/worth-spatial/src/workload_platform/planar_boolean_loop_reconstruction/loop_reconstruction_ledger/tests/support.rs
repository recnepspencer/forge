use crate::workload_platform::planar_boolean_loop_reconstruction::test_support::{
    admitted_phase_fourteen_identity_products, PreparedPhaseFourteenSubject,
};
use crate::workload_platform::planar_boolean_loop_reconstruction::{
    PlanarBooleanLoopIdentityMap, PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopSubshapeSignatureMap,
};

pub(super) fn admitted_identity_products(
    fixture: &PreparedPhaseFourteenSubject,
) -> (
    PlanarBooleanLoopIdentityMap,
    PlanarBooleanLoopPersistentNamePropagationMap,
    PlanarBooleanLoopSubshapeSignatureMap,
) {
    admitted_phase_fourteen_identity_products(fixture)
}
