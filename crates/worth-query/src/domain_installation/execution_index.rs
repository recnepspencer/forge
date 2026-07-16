use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::application::WorthQueryDeclarationEntryContributionCategoryFamily;
use crate::authoring::{
    WorthQueryGraphReadDomainOperationDeclaration, WorthQueryGraphReadOperationKey,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphReadOperationLookup, WorthQueryGraphReadOperationRegistration,
    WorthQueryGraphReadOperationUnsupportedShapeDeclaration,
};

use super::installation_state::WorthQueryInstalledDomainArtifact;

mod semantic_keys;

use semantic_keys::{
    InstalledDeclarationFamilyKey, InstalledDeclarationFamilySlot, InstalledDomainOwner,
    InstalledInvariantSlot,
};

#[derive(Debug, Eq, PartialEq)]
pub(crate) struct WorthQueryInstalledDomainSemantics {
    declaration_families: BTreeMap<InstalledDeclarationFamilyKey, u32>,
    contribution_policy: Vec<WorthQueryDeclarationEntryContributionCategoryFamily>,
    identity: WorthQueryEvidenceIdentity,
}

impl WorthQueryInstalledDomainSemantics {
    fn from_artifact(artifact: &WorthQueryInstalledDomainArtifact) -> Self {
        let declaration_families = artifact
            .declaration_families
            .iter()
            .map(|family| {
                (
                    InstalledDeclarationFamilyKey::new(family.family_key()),
                    family.version(),
                )
            })
            .collect::<BTreeMap<_, _>>();
        let contribution_policy = artifact.contribution_policy.clone();
        let identity =
            worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecutionIndex)
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("package"),
                    artifact.package_identity.evidence_identity(),
                )
                .field_value_sequence(
                    WorthQueryEvidenceTag::new("declaration_family"),
                    declaration_families
                        .iter()
                        .map(|(family, version)| format!("{}:{version}", family.as_str())),
                )
                .field_value_sequence(
                    WorthQueryEvidenceTag::new("contribution_policy"),
                    contribution_policy.iter().map(|category| category.as_str()),
                )
                .seal();
        Self {
            declaration_families,
            contribution_policy,
            identity,
        }
    }

    pub(crate) fn declaration_family_version(&self, family_key: &str) -> Option<u32> {
        self.declaration_families.get(family_key).copied()
    }

    pub(crate) fn contribution_policy(
        &self,
    ) -> &[WorthQueryDeclarationEntryContributionCategoryFamily] {
        &self.contribution_policy
    }

    pub(crate) fn permits_contribution(
        &self,
        category: WorthQueryDeclarationEntryContributionCategoryFamily,
    ) -> bool {
        self.contribution_policy.contains(&category)
    }

    pub(crate) fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub(crate) struct WorthQueryInstalledDomainExecutionIndexShape {
    pub(crate) invariant_count: usize,
    pub(crate) graph_obligation_count: usize,
    pub(crate) operation_count: usize,
    pub(crate) declaration_family_count: usize,
    pub(crate) contribution_policy_count: usize,
}

pub(crate) struct WorthQueryInstalledDomainExecutionIndex {
    graph_read_operations:
        BTreeMap<WorthQueryGraphReadOperationKey, WorthQueryGraphReadOperationRegistration>,
    semantics_by_marker: HashMap<TypeId, Arc<WorthQueryInstalledDomainSemantics>>,
    identity: WorthQueryEvidenceIdentity,
    shape: WorthQueryInstalledDomainExecutionIndexShape,
    runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
    indexed_operation_lookups: AtomicUsize,
}

impl WorthQueryInstalledDomainExecutionIndex {
    pub(crate) fn build(
        artifacts: &[WorthQueryInstalledDomainArtifact],
        runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
    ) -> Self {
        let graph_read_operations = graph_read_operation_index(artifacts);
        let semantics_by_marker = installed_domain_semantics_index(artifacts);
        let declaration_families = declaration_family_index(artifacts);
        let contribution_policies = contribution_policy_index(artifacts);
        let invariant_slots = invariant_slot_index(artifacts);
        let graph_obligation_digests = graph_obligation_identity_parts(artifacts);
        let package_provenance = package_provenance_identity_parts(artifacts);
        let identity = execution_index_identity(
            &package_provenance,
            &graph_read_operations,
            &declaration_families,
            &contribution_policies,
            &invariant_slots,
            &graph_obligation_digests,
        );
        let shape = execution_index_shape(
            &graph_read_operations,
            &declaration_families,
            &contribution_policies,
            &invariant_slots,
            &graph_obligation_digests,
        );
        Self {
            graph_read_operations,
            semantics_by_marker,
            identity,
            shape,
            runtime_authority,
            indexed_operation_lookups: AtomicUsize::new(0),
        }
    }

    pub(crate) fn domain_semantics(
        &self,
        marker_type: TypeId,
    ) -> Option<Arc<WorthQueryInstalledDomainSemantics>> {
        self.semantics_by_marker.get(&marker_type).map(Arc::clone)
    }

    pub(crate) fn identity(&self) -> &WorthQueryEvidenceIdentity {
        &self.identity
    }

    pub(crate) fn shape(&self) -> WorthQueryInstalledDomainExecutionIndexShape {
        self.shape
    }

    pub(crate) fn indexed_operation_lookups(&self) -> usize {
        self.indexed_operation_lookups.load(Ordering::Relaxed)
    }
}

fn graph_read_operation_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> BTreeMap<WorthQueryGraphReadOperationKey, WorthQueryGraphReadOperationRegistration> {
    artifacts
        .iter()
        .flat_map(|artifact| artifact.graph_read_operations.iter())
        .map(|operation| {
            let key = operation
                .operation_key()
                .expect("admitted package operations have canonical keys");
            (key, operation.clone())
        })
        .collect()
}

fn installed_domain_semantics_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> HashMap<TypeId, Arc<WorthQueryInstalledDomainSemantics>> {
    artifacts
        .iter()
        .map(|artifact| {
            (
                artifact.marker_type,
                Arc::new(WorthQueryInstalledDomainSemantics::from_artifact(artifact)),
            )
        })
        .collect()
}

fn declaration_family_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> BTreeMap<InstalledDeclarationFamilySlot, String> {
    artifacts
        .iter()
        .flat_map(|artifact| {
            artifact.declaration_families.iter().map(|family| {
                (
                    InstalledDeclarationFamilySlot::new(
                        &artifact.domain_owner,
                        family.family_key(),
                    ),
                    family.canonical_part(),
                )
            })
        })
        .collect()
}

fn contribution_policy_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> BTreeMap<InstalledDomainOwner, Vec<String>> {
    artifacts
        .iter()
        .map(|artifact| {
            (
                InstalledDomainOwner::new(&artifact.domain_owner),
                artifact
                    .contribution_policy
                    .iter()
                    .map(|category| category.as_str().to_string())
                    .collect(),
            )
        })
        .collect()
}

fn invariant_slot_index(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> BTreeMap<InstalledInvariantSlot, String> {
    artifacts
        .iter()
        .flat_map(|artifact| {
            artifact.invariant_definitions.iter().map(|invariant| {
                (
                    InstalledInvariantSlot::new(&artifact.domain_owner, invariant.slot_key()),
                    invariant.canonical_part(),
                )
            })
        })
        .collect()
}

fn graph_obligation_identity_parts(artifacts: &[WorthQueryInstalledDomainArtifact]) -> Vec<String> {
    let mut identity_parts = artifacts
        .iter()
        .flat_map(|artifact| {
            artifact
                .graph_obligation_definitions
                .iter()
                .map(|obligation| {
                    format!("{}:{}", artifact.domain_owner, obligation.canonical_part())
                })
        })
        .collect::<Vec<_>>();
    identity_parts.sort();
    identity_parts
}

fn package_provenance_identity_parts(
    artifacts: &[WorthQueryInstalledDomainArtifact],
) -> Vec<String> {
    let mut identity_parts = artifacts
        .iter()
        .map(|artifact| {
            artifact
                .substrate_provenance
                .identity()
                .as_str()
                .to_string()
        })
        .collect::<Vec<_>>();
    identity_parts.sort();
    identity_parts
}

fn execution_index_shape(
    graph_read_operations: &BTreeMap<
        WorthQueryGraphReadOperationKey,
        WorthQueryGraphReadOperationRegistration,
    >,
    declaration_families: &BTreeMap<InstalledDeclarationFamilySlot, String>,
    contribution_policies: &BTreeMap<InstalledDomainOwner, Vec<String>>,
    invariant_slots: &BTreeMap<InstalledInvariantSlot, String>,
    graph_obligation_identity_parts: &[String],
) -> WorthQueryInstalledDomainExecutionIndexShape {
    WorthQueryInstalledDomainExecutionIndexShape {
        invariant_count: invariant_slots.len(),
        graph_obligation_count: graph_obligation_identity_parts.len(),
        operation_count: graph_read_operations.len(),
        declaration_family_count: declaration_families.len(),
        contribution_policy_count: contribution_policies.values().map(Vec::len).sum(),
    }
}

impl WorthQueryGraphReadOperationLookup for WorthQueryInstalledDomainExecutionIndex {
    fn matching_declared_operation(
        &self,
        declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationRegistration> {
        self.indexed_operation_lookups
            .fetch_add(1, Ordering::Relaxed);
        self.graph_read_operations
            .get(declaration.key())
            .filter(|registration| {
                declaration_authorizes_registration(
                    declaration,
                    registration,
                    self.runtime_authority,
                )
            })
            .filter(|registration| registration.matches_declared_operation(declaration))
    }

    fn matching_unsupported_declared_operation(
        &self,
        _declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationUnsupportedShapeDeclaration> {
        None
    }
}

fn declaration_authorizes_registration(
    declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    registration: &WorthQueryGraphReadOperationRegistration,
    runtime_authority: crate::runtime::WorthQueryRuntimeAuthorityIdentity,
) -> bool {
    let Some(authority) = declaration.installed_authority() else {
        return false;
    };
    let Some(provenance) = registration.installed_provenance() else {
        return false;
    };
    authority.authority().runtime_authority() == runtime_authority
        && authority.package_identity().as_str() == provenance.package_identity()
        && authority.authority().domain_owner() == provenance.domain_owner()
}

fn execution_index_identity(
    packages: &[String],
    operations: &BTreeMap<
        WorthQueryGraphReadOperationKey,
        WorthQueryGraphReadOperationRegistration,
    >,
    families: &BTreeMap<InstalledDeclarationFamilySlot, String>,
    policies: &BTreeMap<InstalledDomainOwner, Vec<String>>,
    invariants: &BTreeMap<InstalledInvariantSlot, String>,
    obligations: &[String],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecutionIndex)
        .field_value_sequence(WorthQueryEvidenceTag::new("package"), packages)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("operation"),
            operations.values().map(|operation| operation.digest_part()),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("family"),
            families
                .iter()
                .map(|(slot, meaning)| format!("{}:{meaning}", slot.terminal_projection())),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("policy"),
            policies.iter().flat_map(|(owner, values)| {
                values
                    .iter()
                    .map(move |value| format!("{}:{value}", owner.as_str()))
            }),
        )
        .field_value_sequence(
            WorthQueryEvidenceTag::new("invariant"),
            invariants
                .iter()
                .map(|(slot, meaning)| format!("{}:{meaning}", slot.terminal_projection())),
        )
        .field_value_sequence(WorthQueryEvidenceTag::new("obligation"), obligations)
        .seal()
}
