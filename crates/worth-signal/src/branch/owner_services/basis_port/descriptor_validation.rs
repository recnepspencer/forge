use crate::branch::{
    SignalBranchBasisDescriptor, SignalBranchBasisReadmissionDenial, SignalBranchObservation,
};

pub(super) fn compare_descriptor_with_observation(
    descriptor: &SignalBranchBasisDescriptor,
    observation: &SignalBranchObservation,
) -> Result<(), SignalBranchBasisReadmissionDenial> {
    descriptor
        .observation()
        .compare(observation)
        .map_err(
            |mismatch| SignalBranchBasisReadmissionDenial::ReferenceMismatch {
                axes: mismatch.axes().to_vec(),
            },
        )
}
