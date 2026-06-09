use crate::bindings::authority::SpatialBindingKind;
use crate::bindings::query_native::{
    PrimitiveAnchorBindingQueryDomain, PrimitiveBindingQueryDomain,
};
use crate::bindings::query_native_anchor_binding_authoring::{
    PrimitiveAnchorBindingAuthoringError, PrimitiveAnchorBindingDeclarationEntry,
};
use crate::bindings::query_native_binding_authoring::{
    PrimitiveBindingAuthoringError, PrimitiveBindingDeclarationEntry,
};
use crate::bindings::query_native_rebinding_declared_binding_fact::DeclaredNeighborhoodBindingFact;
use crate::bindings::rebinding::binding_snapshot::BindingSnapshot;
use crate::bindings::rebinding::NeighborhoodBindingFamily;
use forge_query::facade::{
    ForgeQueryAdmittedConfiguredDomainHandle, ForgeQueryDomainOperatingContext,
    ForgeQueryOrdinaryOutcome,
};

#[derive(Clone, Debug, PartialEq)]
pub struct PrimitiveRebindingCandidateFact {
    binding_kind: SpatialBindingKind,
    binding_identity: String,
    site_identity: String,
    family: NeighborhoodBindingFamily,
    snapshot: BindingSnapshot,
}

impl PrimitiveRebindingCandidateFact {
    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.binding_kind
    }

    pub fn binding_identity(&self) -> &str {
        &self.binding_identity
    }

    pub fn site_identity(&self) -> &str {
        &self.site_identity
    }

    pub fn family(&self) -> NeighborhoodBindingFamily {
        self.family
    }

    pub(crate) fn snapshot(&self) -> &BindingSnapshot {
        &self.snapshot
    }

    pub(crate) fn from_neighborhood_binding_fact(fact: &DeclaredNeighborhoodBindingFact) -> Self {
        Self {
            binding_kind: fact.binding_kind(),
            binding_identity: fact.binding_identity().to_string(),
            site_identity: fact.site_identity().to_string(),
            family: fact.family(),
            snapshot: fact.snapshot().clone(),
        }
    }
}

#[derive(Clone, Debug, PartialEq)]
pub enum PrimitiveRebindingCandidateFactError {
    Binding(PrimitiveBindingAuthoringError),
    Anchor(PrimitiveAnchorBindingAuthoringError),
    QueryNotBound,
}

pub fn primitive_binding_rebinding_candidate_fact<C>(
    declaration: &PrimitiveBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveBindingQueryDomain, C>,
) -> Result<PrimitiveRebindingCandidateFact, PrimitiveRebindingCandidateFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(_) => declaration
            .rebinding_candidate_fact()
            .map_err(PrimitiveRebindingCandidateFactError::Binding)
            .cloned(),
        _ => Err(PrimitiveRebindingCandidateFactError::QueryNotBound),
    }
}

pub fn primitive_anchor_binding_rebinding_candidate_fact<C>(
    declaration: &PrimitiveAnchorBindingDeclarationEntry,
    handle: &ForgeQueryAdmittedConfiguredDomainHandle<PrimitiveAnchorBindingQueryDomain, C>,
) -> Result<PrimitiveRebindingCandidateFact, PrimitiveRebindingCandidateFactError>
where
    C: ForgeQueryDomainOperatingContext<PrimitiveAnchorBindingQueryDomain>,
{
    match handle.orchestrate_declaration_entry_outcome(declaration.clone()) {
        ForgeQueryOrdinaryOutcome::Bound(_) => declaration
            .rebinding_candidate_fact()
            .map_err(PrimitiveRebindingCandidateFactError::Anchor)
            .cloned(),
        _ => Err(PrimitiveRebindingCandidateFactError::QueryNotBound),
    }
}
