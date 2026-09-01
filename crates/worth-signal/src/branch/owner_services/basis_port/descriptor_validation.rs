use worth_foundational::FoundationalBranchReferenceMismatchAxis;

use crate::branch::{
    SignalBranchBasisDescriptor, SignalBranchBasisLifecyclePosture,
    SignalBranchBasisReadmissionDenial, SignalBranchObservation,
    SignalBranchRetainedReadmissionDenial, SignalBranchRetentionLease,
    SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION,
};
use crate::state::SignalBranchId;

use super::super::SignalOwner;

pub(super) fn validate_managed_descriptor<D, I, T>(
    owner: &SignalOwner<D, I, T>,
    descriptor: &SignalBranchBasisDescriptor,
    expected_branch_id: SignalBranchId,
) -> Result<(), SignalBranchBasisReadmissionDenial>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    if descriptor.schema_version() != SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION {
        return Err(
            SignalBranchBasisReadmissionDenial::UnsupportedDescriptorVersion {
                observed: descriptor.schema_version(),
                supported: SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION,
            },
        );
    }
    if descriptor.lifecycle_posture() != SignalBranchBasisLifecyclePosture::Live {
        return Err(SignalBranchBasisReadmissionDenial::LifecycleMismatch);
    }
    if descriptor.branch_id() != expected_branch_id {
        return Err(SignalBranchBasisReadmissionDenial::ReferenceMismatch {
            axes: vec![FoundationalBranchReferenceMismatchAxis::BranchIdentity],
        });
    }
    let Some(target) = descriptor.observation().target().as_basis() else {
        return Err(SignalBranchBasisReadmissionDenial::ReferenceMismatch {
            axes: vec![FoundationalBranchReferenceMismatchAxis::TargetBasis],
        });
    };
    let runtime_graph_instance_id = owner.runtime_instance_id().to_string();
    if target.graph_instance_id() != runtime_graph_instance_id {
        return Err(SignalBranchBasisReadmissionDenial::OwnerMismatch {
            descriptor_graph_instance_id: target.graph_instance_id().to_owned(),
            runtime_graph_instance_id,
        });
    }
    if target.definition_basis() != owner.definition_basis() {
        return Err(SignalBranchBasisReadmissionDenial::DefinitionMismatch {
            descriptor_definition_basis: target.definition_basis(),
            runtime_definition_basis: owner.definition_basis(),
        });
    }
    Ok(())
}

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

pub(super) fn validate_retained_descriptor(
    descriptor: &SignalBranchBasisDescriptor,
    lease: &SignalBranchRetentionLease,
) -> Result<(), SignalBranchRetainedReadmissionDenial> {
    if !lease.retains_live_obligation() {
        return Err(SignalBranchRetainedReadmissionDenial::UnavailableRetainedTarget);
    }
    if descriptor != lease.descriptor() {
        return Err(SignalBranchRetainedReadmissionDenial::DescriptorMismatch);
    }
    if descriptor.schema_version() != SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION {
        return Err(
            SignalBranchRetainedReadmissionDenial::UnsupportedDescriptorVersion {
                observed: descriptor.schema_version(),
                supported: SIGNAL_BRANCH_BASIS_DESCRIPTOR_SCHEMA_VERSION,
            },
        );
    }
    if descriptor.lifecycle_posture() != SignalBranchBasisLifecyclePosture::Live {
        return Err(SignalBranchRetainedReadmissionDenial::LifecycleMismatch);
    }
    Ok(())
}
