use worth_foundational::facade::{
    canonical_basis_sequence_material, canonicalization, prepare_canonical_basis_sequence,
    AspectBinding, AspectContract, AspectMask, AuthoritativeAspectChangeKind, CanonicalBasisDomain,
    CanonicalBasisEntry, CanonicalBasisEntryKind, CanonicalBasisLocus, CanonicalBasisValue,
    CanonicalBundleReadyArtifact, CanonicalizationRuleVersion, ProjectionMask, TruthPartitionRole,
};

pub type WorthQueryTruthPartitionRole = TruthPartitionRole;

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub struct WorthQueryConditionalGraphReadRole(String);

impl WorthQueryConditionalGraphReadRole {
    pub fn new(value: impl Into<String>) -> Result<Self, WorthQuerySemanticTruthDependencyDenial> {
        let value = value.into();
        if value.trim().is_empty()
            || value.trim() != value
            || value.chars().any(char::is_whitespace)
        {
            return Err(WorthQuerySemanticTruthDependencyDenial::InvalidGraphReadRole);
        }
        Ok(Self(value))
    }

    pub fn as_str(&self) -> &str {
        &self.0
    }
}

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum WorthQuerySemanticLocality {
    SourceRecord,
    SourcePartition(WorthQueryTruthPartitionRole),
    WholeLogicalGraph,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQuerySemanticTruthDependency {
    graph_read_role: WorthQueryConditionalGraphReadRole,
    contract: AspectContract,
    projection_mask: AspectMask<ProjectionMask>,
    binding: AspectBinding,
    locality: WorthQuerySemanticLocality,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
}

#[derive(Debug)]
pub struct WorthQuerySemanticDependencyCanonicalBasis {
    foundational: CanonicalBundleReadyArtifact,
    binding: String,
    locality: WorthQuerySemanticLocality,
    relevant_changes: Vec<AuthoritativeAspectChangeKind>,
    graph_read_role: WorthQueryConditionalGraphReadRole,
}

impl WorthQuerySemanticDependencyCanonicalBasis {
    pub fn foundational(&self) -> &CanonicalBundleReadyArtifact {
        &self.foundational
    }

    pub fn relational_binding(&self) -> &str {
        &self.binding
    }

    pub fn locality(&self) -> &WorthQuerySemanticLocality {
        &self.locality
    }

    pub fn relevant_changes(&self) -> &[AuthoritativeAspectChangeKind] {
        &self.relevant_changes
    }

    pub fn graph_read_role(&self) -> &WorthQueryConditionalGraphReadRole {
        &self.graph_read_role
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQuerySemanticTruthDependencyDenial {
    InvalidPartitionRole,
    InvalidGraphReadRole,
    EmptyRelevantChangeSet,
    ProjectionMaskNotAdmitted,
    ChangeMeaningDoesNotMatchBinding,
    CanonicalBasisConstructionFailed,
}

impl WorthQuerySemanticTruthDependency {
    pub fn new(
        graph_read_role: WorthQueryConditionalGraphReadRole,
        contract: AspectContract,
        projection_mask: AspectMask<ProjectionMask>,
        binding: AspectBinding,
        locality: WorthQuerySemanticLocality,
        relevant_changes: impl IntoIterator<Item = AuthoritativeAspectChangeKind>,
    ) -> Result<Self, WorthQuerySemanticTruthDependencyDenial> {
        contract
            .admits_projection_mask(&projection_mask)
            .map_err(|_| WorthQuerySemanticTruthDependencyDenial::ProjectionMaskNotAdmitted)?;
        let mut relevant_changes = relevant_changes.into_iter().collect::<Vec<_>>();
        relevant_changes.sort();
        relevant_changes.dedup();
        if relevant_changes.is_empty() {
            return Err(WorthQuerySemanticTruthDependencyDenial::EmptyRelevantChangeSet);
        }
        if relevant_changes
            .iter()
            .any(|change| !binding_admits_change(&binding, *change))
        {
            return Err(WorthQuerySemanticTruthDependencyDenial::ChangeMeaningDoesNotMatchBinding);
        }
        let dependency = Self {
            graph_read_role,
            contract,
            projection_mask,
            binding,
            locality,
            relevant_changes,
        };
        dependency.canonical_basis()?;
        Ok(dependency)
    }

    pub fn contract(&self) -> &AspectContract {
        &self.contract
    }

    pub fn graph_read_role(&self) -> &WorthQueryConditionalGraphReadRole {
        &self.graph_read_role
    }

    pub fn projection_mask(&self) -> &AspectMask<ProjectionMask> {
        &self.projection_mask
    }

    pub fn binding(&self) -> &AspectBinding {
        &self.binding
    }

    pub fn locality(&self) -> &WorthQuerySemanticLocality {
        &self.locality
    }

    pub fn relevant_changes(&self) -> &[AuthoritativeAspectChangeKind] {
        &self.relevant_changes
    }

    pub fn canonical_basis(
        &self,
    ) -> Result<WorthQuerySemanticDependencyCanonicalBasis, WorthQuerySemanticTruthDependencyDenial>
    {
        let version = CanonicalizationRuleVersion::new("worth-query-semantic-dependency-v1")
            .expect("static canonicalization version is valid");
        let contract = canonicalization()
            .basis()
            .at(version.clone())
            .from_contract(self.contract.clone())
            .into_result()
            .map_err(|_| {
                WorthQuerySemanticTruthDependencyDenial::CanonicalBasisConstructionFailed
            })?;
        let mask = canonicalization()
            .basis()
            .at(version.clone())
            .from_mask(self.contract.key().clone(), self.projection_mask.clone())
            .into_result()
            .map_err(|_| {
                WorthQuerySemanticTruthDependencyDenial::CanonicalBasisConstructionFailed
            })?;
        let semantic = prepare_canonical_basis_sequence(
            version.clone(),
            CanonicalBasisDomain::Future("query-semantic-dependency"),
            semantic_basis_entries(self),
        )
        .into_result()
        .map_err(|_| WorthQuerySemanticTruthDependencyDenial::CanonicalBasisConstructionFailed)?;
        let foundational = canonicalization()
            .basis()
            .at(version)
            .bundle([contract, mask, semantic])
            .into_result()
            .map_err(|_| {
                WorthQuerySemanticTruthDependencyDenial::CanonicalBasisConstructionFailed
            })?;
        Ok(WorthQuerySemanticDependencyCanonicalBasis {
            foundational,
            binding: self.binding.canonical_name(),
            locality: self.locality.clone(),
            relevant_changes: self.relevant_changes.clone(),
            graph_read_role: self.graph_read_role.clone(),
        })
    }
}

fn semantic_basis_entries(
    dependency: &WorthQuerySemanticTruthDependency,
) -> Vec<CanonicalBasisEntry> {
    let domain = CanonicalBasisDomain::Future("query-semantic-dependency");
    let mut entries = vec![
        semantic_entry(domain, "binding", dependency.binding.canonical_name()),
        semantic_entry(
            domain,
            "graph-read-role",
            dependency.graph_read_role.as_str().to_string(),
        ),
        semantic_entry(domain, "locality", locality_token(&dependency.locality)),
    ];
    entries.extend(
        dependency
            .relevant_changes
            .iter()
            .enumerate()
            .map(|(index, change)| {
                semantic_entry(
                    domain,
                    &format!("relevant-change.{index:04}"),
                    change.canonical_name().to_string(),
                )
            }),
    );
    entries
}

fn semantic_entry(domain: CanonicalBasisDomain, locus: &str, value: String) -> CanonicalBasisEntry {
    CanonicalBasisEntry::new(
        domain,
        CanonicalBasisLocus::Named(locus.to_string().into()),
        CanonicalBasisEntryKind::Value,
        CanonicalBasisValue::ExactText(value.into()),
    )
}

fn binding_admits_change(binding: &AspectBinding, change: AuthoritativeAspectChangeKind) -> bool {
    use AuthoritativeAspectChangeKind as Change;
    match binding {
        AspectBinding::RelationSourceEndpoint => matches!(change, Change::RelationSourceEndpoint),
        AspectBinding::RelationTargetEndpoint => matches!(change, Change::RelationTargetEndpoint),
        AspectBinding::StructuralRegion
        | AspectBinding::StructuralPartition
        | AspectBinding::StructuralFacet => matches!(
            change,
            Change::StructuralCreate
                | Change::StructuralUpdate
                | Change::StructuralDelete
                | Change::StructuralRetainForAudit
        ),
        AspectBinding::LifecycleTransition => matches!(
            change,
            Change::LifecycleCreate | Change::LifecycleDelete | Change::LifecycleRetainForAudit
        ),
        AspectBinding::EntityField { .. } | AspectBinding::RelationField { .. } => matches!(
            change,
            Change::WholeAspectSet
                | Change::WholeAspectClear
                | Change::FieldSet
                | Change::FieldClear
                | Change::Opaque
        ),
        _ => false,
    }
}

pub(crate) fn dependency_token(dependency: &WorthQuerySemanticTruthDependency) -> String {
    let basis = dependency
        .canonical_basis()
        .expect("validated semantic dependencies retain a canonical basis");
    let foundational = basis
        .foundational()
        .payload()
        .sequences()
        .iter()
        .map(|sequence| canonical_basis_sequence_material(sequence.payload()))
        .collect::<Vec<_>>()
        .join("|");
    let locality = locality_token(dependency.locality());
    let changes = dependency
        .relevant_changes()
        .iter()
        .map(|change| change.canonical_name())
        .collect::<Vec<_>>()
        .join(",");
    let mut material = String::new();
    super::push_token(&mut material, "foundational", &foundational);
    super::push_token(
        &mut material,
        "graph-read-role",
        dependency.graph_read_role().as_str(),
    );
    super::push_token(
        &mut material,
        "binding",
        &dependency.binding().canonical_name(),
    );
    super::push_token(&mut material, "locality", &locality);
    super::push_token(&mut material, "changes", &changes);
    material
}

pub(crate) fn locality_token(locality: &WorthQuerySemanticLocality) -> String {
    match locality {
        WorthQuerySemanticLocality::SourceRecord => "source-record".to_string(),
        WorthQuerySemanticLocality::SourcePartition(role) => {
            format!("source-partition:{}", role.as_str())
        }
        WorthQuerySemanticLocality::WholeLogicalGraph => "whole-logical-graph".to_string(),
    }
}

pub(crate) fn contract_token(contract: &AspectContract) -> String {
    let version = CanonicalizationRuleVersion::new("worth-query-conditional-contract-v1")
        .expect("static canonicalization version is valid");
    let basis = canonicalization()
        .basis()
        .at(version)
        .from_contract(contract.clone())
        .into_result()
        .expect("admitted Foundational contracts always produce canonical basis");
    canonical_basis_sequence_material(basis.payload())
}
