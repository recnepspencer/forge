use schema::facade::platform::aspects::Aspect;
use schema::facade::platform::authority::DerivedInvalidationTarget;
use schema::facade::topology_authoring::DerivedTopologyReadBasis;
use serde::Serialize;

use super::consumer::TopologyCompiledProductConsumer;
use super::family_identity::TopologyCompiledProductFamilyIdentity;
use crate::derived_topology::invalidation_plan::compiled_product_admission::{
    TopologyCompiledProductLocalityBasis, TopologyCompiledProductSourceAuthorityBasis,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
pub struct TopologyCompiledProductFamilyAdmittedInput {
    consumer: TopologyCompiledProductConsumer,
    family_identity: TopologyCompiledProductFamilyIdentity,
    authority_snapshot_id: u64,
    authority_branch_id: String,
    truth_basis_digest_hex: String,
    touched_aspect_count: usize,
    triggered_invalidation_targets: Vec<DerivedInvalidationTarget>,
    locality_digest: String,
    precision_fallback_count: usize,
    precision_budget_fallback_count: usize,
}

impl TopologyCompiledProductFamilyAdmittedInput {
    pub const fn consumer(&self) -> TopologyCompiledProductConsumer {
        self.consumer
    }

    pub const fn family_identity(&self) -> TopologyCompiledProductFamilyIdentity {
        self.family_identity
    }

    pub const fn authority_snapshot_id(&self) -> u64 {
        self.authority_snapshot_id
    }

    pub fn authority_branch_id(&self) -> &str {
        &self.authority_branch_id
    }

    pub fn truth_basis_digest_hex(&self) -> &str {
        &self.truth_basis_digest_hex
    }

    pub const fn touched_aspect_count(&self) -> usize {
        self.touched_aspect_count
    }

    pub fn triggered_invalidation_targets(&self) -> &[DerivedInvalidationTarget] {
        &self.triggered_invalidation_targets
    }

    pub fn locality_digest(&self) -> &str {
        &self.locality_digest
    }

    pub const fn precision_fallback_count(&self) -> usize {
        self.precision_fallback_count
    }

    pub const fn precision_budget_fallback_count(&self) -> usize {
        self.precision_budget_fallback_count
    }

    pub(crate) fn from_admission_bases(
        consumer: TopologyCompiledProductConsumer,
        family_identity: TopologyCompiledProductFamilyIdentity,
        source_authority_basis: &TopologyCompiledProductSourceAuthorityBasis,
        locality_basis: &TopologyCompiledProductLocalityBasis,
    ) -> Self {
        Self {
            consumer,
            family_identity,
            authority_snapshot_id: source_authority_basis.authority_snapshot_id(),
            authority_branch_id: source_authority_basis.authority_branch_id().to_string(),
            truth_basis_digest_hex: source_authority_basis.truth_basis_digest_hex().to_string(),
            touched_aspect_count: source_authority_basis.touched_aspect_count(),
            triggered_invalidation_targets: locality_basis
                .triggered_invalidation_targets()
                .to_vec(),
            locality_digest: locality_basis.locality_digest().to_string(),
            precision_fallback_count: source_authority_basis.precision_fallback_count(),
            precision_budget_fallback_count: source_authority_basis
                .precision_budget_fallback_count(),
        }
    }
}

pub(crate) fn triggered_invalidation_targets_from_read_basis(
    read_basis: &DerivedTopologyReadBasis,
) -> Vec<DerivedInvalidationTarget> {
    triggered_invalidation_targets_from_touched_aspects(
        read_basis.touched_aspects().iter().copied(),
    )
}

pub(crate) fn triggered_invalidation_targets_from_touched_aspects(
    touched_aspects: impl IntoIterator<Item = Aspect>,
) -> Vec<DerivedInvalidationTarget> {
    let mut targets = Vec::new();
    for aspect in touched_aspects {
        match aspect {
            Aspect::Topology(topology) => match topology {
                schema::facade::platform::aspects::TopologyAspect::Structure => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyStructure);
                }
                schema::facade::platform::aspects::TopologyAspect::Ownership => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyOwnership);
                }
                schema::facade::platform::aspects::TopologyAspect::Boundary => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyBoundary);
                }
                schema::facade::platform::aspects::TopologyAspect::Radial => {
                    push_unique_target(&mut targets, DerivedInvalidationTarget::TopologyRadial);
                }
            },
            Aspect::Naming(schema::facade::platform::aspects::NamingAspect::PersistentName) => {
                push_unique_target(
                    &mut targets,
                    DerivedInvalidationTarget::NamingPersistentName,
                );
            }
            _ => {}
        }
    }
    targets
}

fn push_unique_target(
    targets: &mut Vec<DerivedInvalidationTarget>,
    target: DerivedInvalidationTarget,
) {
    if !targets.contains(&target) {
        targets.push(target);
    }
}
