use std::any::TypeId;
use std::collections::{BTreeMap, HashMap};
use std::sync::atomic::{AtomicUsize, Ordering};
use std::sync::Arc;

use crate::authoring::{
    WorthQueryGraphReadDomainOperationDeclaration, WorthQueryGraphReadOperationKey,
};
use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};
use crate::runtime::{
    WorthQueryGraphReadOperationLookup, WorthQueryGraphReadOperationRegistration,
    WorthQueryGraphReadOperationUnsupportedShapeDeclaration, WorthQueryRuntimeAuthorityIdentity,
};

use super::pending_installations::WorthQueryInstalledDomainArtifact;
use super::{
    WorthQueryDomainExecutionIndexRebuildReport, WorthQueryDomainHandleDenial,
    WorthQueryDomainHandleDenialKind, WorthQueryDomainInstallationConstructionCounters,
    WorthQueryDomainInstallationGeneration, WorthQueryDomainInstallationLookupCounters,
    WorthQueryDomainInstallationReceipt, WorthQueryDomainInstalledDefinitionCounts,
    WorthQueryInstalledDomainAuthority, WorthQueryInstalledDomainHandle,
};

struct WorthQueryInstalledDomainRecord {
    artifact: WorthQueryInstalledDomainArtifact,
    authority: Arc<WorthQueryInstalledDomainAuthority>,
    receipt: WorthQueryDomainInstallationReceipt,
}

pub(crate) struct WorthQueryInstalledDomainExecutionIndex {
    graph_read_operations:
        BTreeMap<WorthQueryGraphReadOperationKey, WorthQueryGraphReadOperationRegistration>,
    declaration_families: BTreeMap<String, String>,
    contribution_policies: BTreeMap<String, Vec<String>>,
    invariant_slots: BTreeMap<String, String>,
    graph_obligation_digests: Vec<String>,
    identity: WorthQueryEvidenceIdentity,
    indexed_operation_lookups: AtomicUsize,
}

pub(crate) struct WorthQueryDomainInstallationRegistry {
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    generation: WorthQueryDomainInstallationGeneration,
    records: Vec<WorthQueryInstalledDomainRecord>,
    by_marker_type: HashMap<TypeId, usize>,
    execution_index: WorthQueryInstalledDomainExecutionIndex,
    handle_lookups: AtomicUsize,
}

impl WorthQueryDomainInstallationRegistry {
    pub(crate) fn from_artifacts(
        artifacts: Vec<WorthQueryInstalledDomainArtifact>,
        runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    ) -> Self {
        let generation = WorthQueryDomainInstallationGeneration::initial();
        let execution_index = WorthQueryInstalledDomainExecutionIndex::build(&artifacts);
        let mut by_marker_type = HashMap::new();
        let records = artifacts
            .into_iter()
            .enumerate()
            .map(|(index, artifact)| {
                by_marker_type.insert(artifact.marker_type, index);
                let counters = WorthQueryDomainInstallationConstructionCounters::for_package(
                    artifact.invariant_definitions.len(),
                    artifact.graph_obligations.len(),
                    artifact.graph_read_operations.len(),
                    artifact.declaration_families.len(),
                    artifact.contribution_policy.len(),
                );
                let installation_identity = installation_identity(
                    runtime_authority,
                    generation,
                    &artifact,
                    &execution_index.identity,
                );
                let receipt = WorthQueryDomainInstallationReceipt::new(
                    artifact.domain_owner.clone(),
                    artifact.semantic_version,
                    artifact.package_identity.clone(),
                    installation_identity.clone(),
                    generation,
                    counters,
                    WorthQueryDomainInstalledDefinitionCounts::new([
                        artifact.required_capabilities.len(),
                        artifact.required_configuration.len(),
                        artifact.operating_requirements.len(),
                        artifact.invariant_definitions.len(),
                        artifact.graph_obligations.len(),
                        artifact.graph_read_operations.len(),
                        artifact.declaration_families.len(),
                        artifact.contribution_policy.len(),
                    ]),
                );
                let authority = Arc::new(WorthQueryInstalledDomainAuthority::new(
                    runtime_authority,
                    generation,
                    artifact.domain_owner.clone(),
                    artifact.package_identity.clone(),
                    installation_identity,
                    artifact.contribution_policy.clone(),
                ));
                WorthQueryInstalledDomainRecord {
                    artifact,
                    authority,
                    receipt,
                }
            })
            .collect();
        Self {
            runtime_authority,
            generation,
            records,
            by_marker_type,
            execution_index,
            handle_lookups: AtomicUsize::new(0),
        }
    }

    pub(crate) fn domain<D: 'static>(
        &self,
    ) -> Result<WorthQueryInstalledDomainHandle<D>, WorthQueryDomainHandleDenial> {
        self.handle_lookups.fetch_add(1, Ordering::Relaxed);
        let record = self.record::<D>().ok_or_else(|| {
            WorthQueryDomainHandleDenial::new(WorthQueryDomainHandleDenialKind::DomainNotInstalled)
        })?;
        Ok(WorthQueryInstalledDomainHandle::mint(Arc::clone(
            &record.authority,
        )))
    }

    pub(crate) fn receipt<D: 'static>(&self) -> Option<&WorthQueryDomainInstallationReceipt> {
        self.record::<D>().map(|record| &record.receipt)
    }

    pub(crate) fn receipts(
        &self,
    ) -> impl ExactSizeIterator<Item = &WorthQueryDomainInstallationReceipt> {
        self.records.iter().map(|record| &record.receipt)
    }

    pub(crate) fn validate<D: 'static>(
        &self,
        handle: &WorthQueryInstalledDomainHandle<D>,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        self.validate_authority::<D>(&handle.authority)
    }

    pub(crate) fn validate_authority<D: 'static>(
        &self,
        authority: &WorthQueryInstalledDomainAuthority,
    ) -> Result<(), WorthQueryDomainHandleDenial> {
        if authority.runtime_authority() != self.runtime_authority {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::ForeignRuntime,
            ));
        }
        if authority.installation_generation() != self.generation {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::StaleInstallationGeneration,
            ));
        }
        let record = self.record::<D>().ok_or_else(|| {
            WorthQueryDomainHandleDenial::new(WorthQueryDomainHandleDenialKind::DomainNotInstalled)
        })?;
        if authority.package_identity() != &record.artifact.package_identity {
            return Err(WorthQueryDomainHandleDenial::new(
                WorthQueryDomainHandleDenialKind::PackageIdentityChanged,
            ));
        }
        Ok(())
    }

    pub(crate) fn execution_index(&self) -> &WorthQueryInstalledDomainExecutionIndex {
        &self.execution_index
    }

    pub(crate) fn rebuild_execution_index_report(
        &self,
    ) -> WorthQueryDomainExecutionIndexRebuildReport {
        let artifacts = self
            .records
            .iter()
            .map(|record| record.artifact.clone())
            .collect::<Vec<_>>();
        let rebuilt = WorthQueryInstalledDomainExecutionIndex::build(&artifacts);
        WorthQueryDomainExecutionIndexRebuildReport::new(
            self.execution_index.identity.as_str().to_string(),
            rebuilt.identity.as_str().to_string(),
            rebuilt.graph_read_operations.len(),
        )
    }

    pub(crate) fn lookup_counters(&self) -> WorthQueryDomainInstallationLookupCounters {
        WorthQueryDomainInstallationLookupCounters::new(
            self.handle_lookups.load(Ordering::Relaxed),
            self.execution_index
                .indexed_operation_lookups
                .load(Ordering::Relaxed),
            0,
        )
    }

    fn record<D: 'static>(&self) -> Option<&WorthQueryInstalledDomainRecord> {
        self.by_marker_type
            .get(&TypeId::of::<D>())
            .and_then(|index| self.records.get(*index))
    }
}

impl WorthQueryInstalledDomainExecutionIndex {
    fn build(artifacts: &[WorthQueryInstalledDomainArtifact]) -> Self {
        let mut graph_read_operations = BTreeMap::new();
        let mut declaration_families = BTreeMap::new();
        let mut contribution_policies = BTreeMap::new();
        let mut invariant_slots = BTreeMap::new();
        let mut graph_obligation_digests = Vec::new();
        for artifact in artifacts {
            for operation in &artifact.graph_read_operations {
                let key = operation
                    .operation_key()
                    .expect("admitted package operations have canonical keys");
                graph_read_operations.insert(key, operation.clone());
            }
            for family in &artifact.declaration_families {
                declaration_families.insert(
                    format!("{}:{}", artifact.domain_owner, family.name().as_str()),
                    family.canonical_part(),
                );
            }
            contribution_policies.insert(
                artifact.domain_owner.clone(),
                artifact
                    .contribution_policy
                    .iter()
                    .map(|category| category.as_str().to_string())
                    .collect(),
            );
            for invariant in &artifact.invariant_definitions {
                invariant_slots.insert(
                    format!("{}:{}", artifact.domain_owner, invariant.slot_key()),
                    invariant.canonical_part(),
                );
            }
            graph_obligation_digests.extend(
                artifact
                    .graph_obligations
                    .iter()
                    .map(|obligation| obligation.registration_digest().to_string()),
            );
        }
        graph_obligation_digests.sort();
        let identity = execution_index_identity(
            &graph_read_operations,
            &declaration_families,
            &contribution_policies,
            &invariant_slots,
            &graph_obligation_digests,
        );
        Self {
            graph_read_operations,
            declaration_families,
            contribution_policies,
            invariant_slots,
            graph_obligation_digests,
            identity,
            indexed_operation_lookups: AtomicUsize::new(0),
        }
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
            .filter(|registration| registration.matches_declared_operation(declaration))
    }

    fn matching_unsupported_declared_operation(
        &self,
        _declaration: &WorthQueryGraphReadDomainOperationDeclaration,
    ) -> Option<&WorthQueryGraphReadOperationUnsupportedShapeDeclaration> {
        None
    }
}

fn execution_index_identity(
    operations: &BTreeMap<
        WorthQueryGraphReadOperationKey,
        WorthQueryGraphReadOperationRegistration,
    >,
    families: &BTreeMap<String, String>,
    policies: &BTreeMap<String, Vec<String>>,
    invariants: &BTreeMap<String, String>,
    obligations: &[String],
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::InstalledDomainExecutionIndex)
        .field_value_sequence(
            WorthQueryEvidenceTag::new("operation"),
            operations.values().map(|operation| operation.digest_part()),
        )
        .field_value_sequence(WorthQueryEvidenceTag::new("family"), families.values())
        .field_value_sequence(
            WorthQueryEvidenceTag::new("policy"),
            policies.iter().flat_map(|(owner, values)| {
                values.iter().map(move |value| format!("{owner}:{value}"))
            }),
        )
        .field_value_sequence(WorthQueryEvidenceTag::new("invariant"), invariants.values())
        .field_value_sequence(WorthQueryEvidenceTag::new("obligation"), obligations)
        .seal()
}

fn installation_identity(
    runtime_authority: WorthQueryRuntimeAuthorityIdentity,
    generation: WorthQueryDomainInstallationGeneration,
    artifact: &WorthQueryInstalledDomainArtifact,
    execution_index_identity: &WorthQueryEvidenceIdentity,
) -> WorthQueryEvidenceIdentity {
    worth_query_evidence_identity(WorthQueryEvidenceScope::DomainInstallation)
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("package"),
            artifact.package_identity.evidence_identity(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("admission"),
            &artifact.admission_identity,
        )
        .field_value(
            WorthQueryEvidenceTag::new("runtime_authority"),
            runtime_authority.as_u64().to_string(),
        )
        .field_value(
            WorthQueryEvidenceTag::new("generation"),
            generation.ordinal().to_string(),
        )
        .field_evidence_identity(
            WorthQueryEvidenceTag::new("execution_index"),
            execution_index_identity,
        )
        .seal()
}
