use forge_relational::facade::history::BranchId;
use schema::facade::platform::aspects::{
    Aspect, DiagnosticsAspect, GeometryAspect, LineageAspect, NamingAspect, TopologyAspect,
};
use schema::facade::platform::authority::{
    WorthTopologyGraphLifecyclePosture, WorthTopologyTouchedAspect,
    WorthTopologyTouchedOperatingWorldPosture, WorthTopologyTouchedScope,
};
use schema::facade::platform::relations::TopologyRelationKind;
use serde::Serialize;

use super::BasisDigestPart;
use crate::topology_operators::{TopologyMutationChangedScope, TopologyMutationFamily};

pub type TopologyTouchedAspect = WorthTopologyTouchedAspect;
pub type TopologyTouchedScope = WorthTopologyTouchedScope;
pub type TopologyGraphLifecyclePosture = WorthTopologyGraphLifecyclePosture;
pub type TopologyTouchedOperatingWorldPosture = WorthTopologyTouchedOperatingWorldPosture;

pub(crate) const fn topology_touched_aspect_from_schema_aspect(
    aspect: Aspect,
) -> TopologyTouchedAspect {
    match aspect {
        Aspect::Topology(TopologyAspect::Structure) => TopologyTouchedAspect::TopologyStructure,
        Aspect::Topology(TopologyAspect::Ownership) => TopologyTouchedAspect::TopologyOwnership,
        Aspect::Topology(TopologyAspect::Boundary) => TopologyTouchedAspect::TopologyBoundary,
        Aspect::Topology(TopologyAspect::Radial) => TopologyTouchedAspect::TopologyRadial,
        Aspect::Geometry(GeometryAspect::Binding) => TopologyTouchedAspect::GeometryBinding,
        Aspect::Geometry(GeometryAspect::Embedding) => TopologyTouchedAspect::GeometryEmbedding,
        Aspect::Geometry(GeometryAspect::Provenance) => TopologyTouchedAspect::GeometryProvenance,
        Aspect::Geometry(GeometryAspect::Approximation) => {
            TopologyTouchedAspect::GeometryApproximation
        }
        Aspect::Geometry(GeometryAspect::UvAnchoring) => TopologyTouchedAspect::GeometryUvAnchoring,
        Aspect::Geometry(GeometryAspect::Carrier) => TopologyTouchedAspect::GeometryCarrier,
        Aspect::Geometry(GeometryAspect::Precision) => TopologyTouchedAspect::GeometryPrecision,
        Aspect::Geometry(GeometryAspect::Fallback) => TopologyTouchedAspect::GeometryFallback,
        Aspect::Lineage(LineageAspect::Provenance) => TopologyTouchedAspect::LineageProvenance,
        Aspect::Naming(NamingAspect::PersistentName) => TopologyTouchedAspect::NamingPersistentName,
        Aspect::Diagnostics(DiagnosticsAspect::Decisions) => {
            TopologyTouchedAspect::DiagnosticsDecisions
        }
        Aspect::Diagnostics(DiagnosticsAspect::Interpretations) => {
            TopologyTouchedAspect::DiagnosticsInterpretations
        }
    }
}

impl BasisDigestPart for WorthTopologyTouchedAspect {
    fn digest_part(&self) -> String {
        format!("aspect:{}", self.as_str())
    }
}

pub(crate) const fn topology_touched_scope_from_changed_scope(
    scope: TopologyMutationChangedScope,
) -> TopologyTouchedScope {
    match scope {
        TopologyMutationChangedScope::Entity => TopologyTouchedScope::Entity,
        TopologyMutationChangedScope::Relation => TopologyTouchedScope::Relation,
        TopologyMutationChangedScope::LocalNeighborhood => TopologyTouchedScope::LocalNeighborhood,
        TopologyMutationChangedScope::Loop => TopologyTouchedScope::Loop,
        TopologyMutationChangedScope::Wire => TopologyTouchedScope::Wire,
        TopologyMutationChangedScope::Shell => TopologyTouchedScope::Shell,
        TopologyMutationChangedScope::RadialNeighborhood => {
            TopologyTouchedScope::RadialNeighborhood
        }
        TopologyMutationChangedScope::Naming => TopologyTouchedScope::Naming,
    }
}

impl BasisDigestPart for WorthTopologyTouchedScope {
    fn digest_part(&self) -> String {
        format!("scope:{}", self.as_str())
    }
}

pub(crate) const fn topology_lifecycle_posture_from_mutation_family(
    family: TopologyMutationFamily,
) -> TopologyGraphLifecyclePosture {
    match family {
        TopologyMutationFamily::CreateTopologyEntity => {
            TopologyGraphLifecyclePosture::EntityCreation
        }
        TopologyMutationFamily::RetireTopologyEntity => {
            TopologyGraphLifecyclePosture::EntityRetirement
        }
        TopologyMutationFamily::AttachBoundaryMembership
        | TopologyMutationFamily::AttachShellOrWireMembership => {
            TopologyGraphLifecyclePosture::ExistingRelationCreate
        }
        TopologyMutationFamily::RewireLoopSuccessor
        | TopologyMutationFamily::RewireLoopEndpoint
        | TopologyMutationFamily::SpliceRadialAdjacency => {
            TopologyGraphLifecyclePosture::ExistingRelationRetarget
        }
        TopologyMutationFamily::DetachBoundaryMembership
        | TopologyMutationFamily::DetachShellOrWireMembership
        | TopologyMutationFamily::DetachRadialAdjacency => {
            TopologyGraphLifecyclePosture::ExistingRelationRemoval
        }
    }
}

impl BasisDigestPart for WorthTopologyGraphLifecyclePosture {
    fn digest_part(&self) -> String {
        format!("lifecycle:{}", self.as_str())
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct TopologyTouchedOperatingWorldIdentityDigest {
    identity_digest: String,
}

impl TopologyTouchedOperatingWorldIdentityDigest {
    pub(crate) fn from_branch_id(branch_id: &BranchId) -> Self {
        Self {
            identity_digest: format!("forge-relational.branch:{}", branch_id.0),
        }
    }

    #[cfg(test)]
    pub(crate) fn for_test(identity_digest: impl Into<String>) -> Self {
        let identity_digest = identity_digest.into();
        assert!(
            !identity_digest.trim().is_empty(),
            "non-mainline touched operating world requires an identity digest"
        );
        Self { identity_digest }
    }

    pub fn as_str(&self) -> &str {
        &self.identity_digest
    }
}

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash, Serialize)]
pub struct TopologyTouchedOperatingWorld {
    posture: TopologyTouchedOperatingWorldPosture,
    identity_digest: Option<TopologyTouchedOperatingWorldIdentityDigest>,
}

impl TopologyTouchedOperatingWorld {
    pub const fn mainline() -> Self {
        Self {
            posture: TopologyTouchedOperatingWorldPosture::Mainline,
            identity_digest: None,
        }
    }

    pub fn branch(identity_digest: TopologyTouchedOperatingWorldIdentityDigest) -> Self {
        Self::identified(
            TopologyTouchedOperatingWorldPosture::Branch,
            identity_digest,
        )
    }

    pub fn preview(identity_digest: TopologyTouchedOperatingWorldIdentityDigest) -> Self {
        Self::identified(
            TopologyTouchedOperatingWorldPosture::Preview,
            identity_digest,
        )
    }

    pub fn configured_domain_handle(
        identity_digest: TopologyTouchedOperatingWorldIdentityDigest,
    ) -> Self {
        Self::identified(
            TopologyTouchedOperatingWorldPosture::ConfiguredDomainHandle,
            identity_digest,
        )
    }

    fn identified(
        posture: TopologyTouchedOperatingWorldPosture,
        identity_digest: TopologyTouchedOperatingWorldIdentityDigest,
    ) -> Self {
        Self {
            posture,
            identity_digest: Some(identity_digest),
        }
    }

    pub const fn posture(&self) -> TopologyTouchedOperatingWorldPosture {
        self.posture
    }

    pub fn identity_digest(&self) -> Option<&str> {
        self.identity_digest.as_ref().map(|digest| digest.as_str())
    }

    pub const fn as_str(&self) -> &'static str {
        self.posture.as_str()
    }
}

impl BasisDigestPart for TopologyTouchedOperatingWorld {
    fn digest_part(&self) -> String {
        match self.identity_digest() {
            Some(identity_digest) => format!("world:{}:{}", self.as_str(), identity_digest),
            None => format!("world:{}", self.as_str()),
        }
    }
}

impl BasisDigestPart for TopologyRelationKind {
    fn digest_part(&self) -> String {
        format!("relation-kind:{}", self.kind_name())
    }
}
