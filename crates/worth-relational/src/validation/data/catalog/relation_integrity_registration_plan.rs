use crate::schema::data::{LoweredRelationIntegrityPlan, MinimumCardinalityEnforcement};
use crate::validation::data::{InvariantRegistration, InvariantRule};

pub(crate) fn relation_integrity_registrations_for_plan(
    plan: &LoweredRelationIntegrityPlan,
) -> Vec<InvariantRegistration> {
    let mut registrations = Vec::with_capacity(plan.contract_count());
    append_endpoint_kind_registrations(plan, &mut registrations);
    append_cardinality_maximum_registrations(plan, &mut registrations);
    append_cardinality_minimum_registrations(plan, &mut registrations);
    append_relation_shape_registrations(plan, &mut registrations);
    append_publication_registrations(plan, &mut registrations);
    registrations
}

fn append_endpoint_kind_registrations(
    plan: &LoweredRelationIntegrityPlan,
    registrations: &mut Vec<InvariantRegistration>,
) {
    registrations.extend(
        plan.endpoint_kind_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::EndpointKindContract(contract),
                )
            }),
    );
}

fn append_cardinality_maximum_registrations(
    plan: &LoweredRelationIntegrityPlan,
    registrations: &mut Vec<InvariantRegistration>,
) {
    registrations.extend(
        plan.cardinality_maximum_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::CardinalityMaximumContract(contract),
                )
            }),
    );
}

fn append_cardinality_minimum_registrations(
    plan: &LoweredRelationIntegrityPlan,
    registrations: &mut Vec<InvariantRegistration>,
) {
    registrations.extend(
        plan.cardinality_minimum_contracts
            .iter()
            .cloned()
            .map(|contract| match contract.minimum_enforcement {
                MinimumCardinalityEnforcement::CommitBoundary => {
                    InvariantRegistration::commit_boundary_blocking(
                        InvariantRule::CardinalityMinimumContract(contract),
                    )
                }
                MinimumCardinalityEnforcement::CertificationBoundary => {
                    InvariantRegistration::certification_boundary_blocking(
                        InvariantRule::CardinalityMinimumContract(contract),
                    )
                }
            }),
    );
}

fn append_relation_shape_registrations(
    plan: &LoweredRelationIntegrityPlan,
    registrations: &mut Vec<InvariantRegistration>,
) {
    registrations.extend(plan.uniqueness_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::UniquenessContract(contract))
    }));
    registrations.extend(plan.symmetry_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::SymmetryContract(contract))
    }));
    registrations.extend(
        plan.endpoint_deletion_integrity_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::EndpointDeletionIntegrityContract(contract),
                )
            }),
    );
    registrations.extend(plan.acyclicity_contracts.iter().cloned().map(|contract| {
        InvariantRegistration::commit_boundary_blocking(InvariantRule::AcyclicityContract(contract))
    }));
    registrations.extend(
        plan.partition_isolation_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::commit_boundary_blocking(
                    InvariantRule::PartitionIsolationContract(contract),
                )
            }),
    );
}

fn append_publication_registrations(
    plan: &LoweredRelationIntegrityPlan,
    registrations: &mut Vec<InvariantRegistration>,
) {
    registrations.extend(
        plan.connectivity_minimum_contracts
            .iter()
            .cloned()
            .map(|contract| {
                InvariantRegistration::snapshot_publication_blocking(
                    InvariantRule::ConnectivityMinimumContract(contract),
                )
            }),
    );
}
