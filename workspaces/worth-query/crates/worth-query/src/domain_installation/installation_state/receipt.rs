use crate::evidence_identity::WorthQueryEvidenceIdentity;

use super::{WorthQueryDomainInstallationGeneration, WorthQueryDomainPackageIdentity};
use crate::domain_installation::execution_index::WorthQueryInstalledDomainExecutionIndexShape;

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDomainInstalledDefinitionCounts {
    required_capabilities: usize,
    required_configuration_sections: usize,
    operating_requirements: usize,
    invariants: usize,
    graph_obligations: usize,
    graph_read_operations: usize,
    declaration_families: usize,
    domain_operations: usize,
    contribution_categories: usize,
}

impl WorthQueryDomainInstalledDefinitionCounts {
    pub(crate) const fn new(values: [usize; 9]) -> Self {
        Self {
            required_capabilities: values[0],
            required_configuration_sections: values[1],
            operating_requirements: values[2],
            invariants: values[3],
            graph_obligations: values[4],
            graph_read_operations: values[5],
            declaration_families: values[6],
            domain_operations: values[7],
            contribution_categories: values[8],
        }
    }

    pub const fn required_capabilities(self) -> usize {
        self.required_capabilities
    }
    pub const fn required_configuration_sections(self) -> usize {
        self.required_configuration_sections
    }
    pub const fn operating_requirements(self) -> usize {
        self.operating_requirements
    }
    pub const fn invariants(self) -> usize {
        self.invariants
    }
    pub const fn graph_obligations(self) -> usize {
        self.graph_obligations
    }
    pub const fn graph_read_operations(self) -> usize {
        self.graph_read_operations
    }
    pub const fn declaration_families(self) -> usize {
        self.declaration_families
    }
    pub const fn domain_operations(self) -> usize {
        self.domain_operations
    }
    pub const fn contribution_categories(self) -> usize {
        self.contribution_categories
    }
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDomainInstallationConstructionCounters {
    package_validation_proof_checks: usize,
    package_lowerings: usize,
    invariant_index_entries: usize,
    graph_obligation_index_entries: usize,
    graph_read_operation_index_entries: usize,
    declaration_family_index_entries: usize,
    domain_operation_index_entries: usize,
    contribution_policy_index_entries: usize,
    derived_index_builds: usize,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthQueryDomainInstallationLookupCounters {
    handle_lookups: usize,
    indexed_operation_lookups: usize,
    package_content_scans: usize,
}

impl WorthQueryDomainInstallationLookupCounters {
    pub(crate) const fn new(
        handle_lookups: usize,
        indexed_operation_lookups: usize,
        package_content_scans: usize,
    ) -> Self {
        Self {
            handle_lookups,
            indexed_operation_lookups,
            package_content_scans,
        }
    }

    pub const fn handle_lookups(self) -> usize {
        self.handle_lookups
    }
    pub const fn indexed_operation_lookups(self) -> usize {
        self.indexed_operation_lookups
    }
    pub const fn package_content_scans(self) -> usize {
        self.package_content_scans
    }
}

impl WorthQueryDomainInstallationConstructionCounters {
    pub(crate) fn for_package(
        invariant_count: usize,
        graph_obligation_count: usize,
        graph_read_operation_count: usize,
        declaration_family_count: usize,
        domain_operation_count: usize,
        contribution_policy_count: usize,
    ) -> Self {
        Self {
            package_validation_proof_checks: 1,
            package_lowerings: 1,
            invariant_index_entries: invariant_count,
            graph_obligation_index_entries: graph_obligation_count,
            graph_read_operation_index_entries: graph_read_operation_count,
            declaration_family_index_entries: declaration_family_count,
            domain_operation_index_entries: domain_operation_count,
            contribution_policy_index_entries: contribution_policy_count,
            derived_index_builds: 1,
        }
    }

    pub const fn package_validation_proof_checks(self) -> usize {
        self.package_validation_proof_checks
    }
    pub const fn package_lowerings(self) -> usize {
        self.package_lowerings
    }
    pub const fn invariant_index_entries(self) -> usize {
        self.invariant_index_entries
    }
    pub const fn graph_obligation_index_entries(self) -> usize {
        self.graph_obligation_index_entries
    }
    pub const fn graph_read_operation_index_entries(self) -> usize {
        self.graph_read_operation_index_entries
    }
    pub const fn declaration_family_index_entries(self) -> usize {
        self.declaration_family_index_entries
    }
    pub const fn domain_operation_index_entries(self) -> usize {
        self.domain_operation_index_entries
    }
    pub const fn contribution_policy_index_entries(self) -> usize {
        self.contribution_policy_index_entries
    }
    pub const fn derived_index_builds(self) -> usize {
        self.derived_index_builds
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainInstallationReceipt {
    domain_owner: String,
    semantic_version: super::WorthQueryDomainSemanticVersion,
    package_identity: WorthQueryDomainPackageIdentity,
    installation_identity: WorthQueryEvidenceIdentity,
    generation: WorthQueryDomainInstallationGeneration,
    counters: WorthQueryDomainInstallationConstructionCounters,
    definition_counts: WorthQueryDomainInstalledDefinitionCounts,
    warnings: Vec<String>,
}

impl WorthQueryDomainInstallationReceipt {
    pub(crate) fn new(
        domain_owner: String,
        semantic_version: super::WorthQueryDomainSemanticVersion,
        package_identity: WorthQueryDomainPackageIdentity,
        installation_identity: WorthQueryEvidenceIdentity,
        generation: WorthQueryDomainInstallationGeneration,
        counters: WorthQueryDomainInstallationConstructionCounters,
        definition_counts: WorthQueryDomainInstalledDefinitionCounts,
    ) -> Self {
        Self {
            domain_owner,
            semantic_version,
            package_identity,
            installation_identity,
            generation,
            counters,
            definition_counts,
            warnings: Vec::new(),
        }
    }

    pub fn domain_owner(&self) -> &str {
        &self.domain_owner
    }
    pub const fn semantic_version(&self) -> super::WorthQueryDomainSemanticVersion {
        self.semantic_version
    }

    pub fn package_identity(&self) -> &WorthQueryDomainPackageIdentity {
        &self.package_identity
    }
    pub fn installation_identity(&self) -> &str {
        self.installation_identity.as_str()
    }
    pub const fn installation_generation(&self) -> WorthQueryDomainInstallationGeneration {
        self.generation
    }
    pub const fn construction_counters(&self) -> WorthQueryDomainInstallationConstructionCounters {
        self.counters
    }
    pub const fn definition_counts(&self) -> WorthQueryDomainInstalledDefinitionCounts {
        self.definition_counts
    }
    pub fn warnings(&self) -> &[String] {
        &self.warnings
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDomainExecutionIndexRebuildReport {
    active_identity: String,
    rebuilt_identity: String,
    invariant_count: usize,
    graph_obligation_count: usize,
    operation_count: usize,
    domain_operation_count: usize,
    operation_graph_participation_count: usize,
    operation_required_domain_count: usize,
    declaration_family_count: usize,
    contribution_policy_count: usize,
    equivalent: bool,
}

impl WorthQueryDomainExecutionIndexRebuildReport {
    pub(crate) fn new(
        active_identity: String,
        rebuilt_identity: String,
        shape: WorthQueryInstalledDomainExecutionIndexShape,
    ) -> Self {
        let equivalent = active_identity == rebuilt_identity;
        Self {
            active_identity,
            rebuilt_identity,
            invariant_count: shape.invariant_count,
            graph_obligation_count: shape.graph_obligation_count,
            operation_count: shape.operation_count,
            domain_operation_count: shape.domain_operation_count,
            operation_graph_participation_count: shape.operation_graph_participation_count,
            operation_required_domain_count: shape.operation_required_domain_count,
            declaration_family_count: shape.declaration_family_count,
            contribution_policy_count: shape.contribution_policy_count,
            equivalent,
        }
    }

    pub fn active_identity(&self) -> &str {
        &self.active_identity
    }
    pub fn rebuilt_identity(&self) -> &str {
        &self.rebuilt_identity
    }
    pub const fn invariant_count(&self) -> usize {
        self.invariant_count
    }
    pub const fn graph_obligation_count(&self) -> usize {
        self.graph_obligation_count
    }
    pub const fn operation_count(&self) -> usize {
        self.operation_count
    }
    pub const fn domain_operation_count(&self) -> usize {
        self.domain_operation_count
    }
    pub const fn operation_graph_participation_count(&self) -> usize {
        self.operation_graph_participation_count
    }
    pub const fn operation_required_domain_count(&self) -> usize {
        self.operation_required_domain_count
    }
    pub const fn declaration_family_count(&self) -> usize {
        self.declaration_family_count
    }
    pub const fn contribution_policy_count(&self) -> usize {
        self.contribution_policy_count
    }
    pub const fn is_equivalent(&self) -> bool {
        self.equivalent
    }
}
