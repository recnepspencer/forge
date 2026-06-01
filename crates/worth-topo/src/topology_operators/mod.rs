pub(crate) mod application;
mod declaration_entry;
mod declared_mutation_sequence_builder;
mod facade;
mod local_rewrites;
mod mutation_digest;
pub(crate) mod mutation_records;
mod mutation_sequence;
mod naming_continuity;
mod rejection_locality;

pub(crate) use application::topology_relation_dependency_path;
pub use application::{TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass};
pub use declaration_entry::{
    TopologyAttachBoundaryMembershipDeclaration, TopologyAttachBoundaryMembershipFamily,
    TopologyAttachShellOrWireMembershipDeclaration, TopologyAttachShellOrWireMembershipFamily,
    TopologyCreateInnerLoopOnExistingFaceDeclaration, TopologyCreateInnerLoopOnExistingFaceFamily,
    TopologyCreateTopologyEntityDeclaration, TopologyCreateTopologyEntityFamily,
    TopologyDetachBoundaryMembershipDeclaration, TopologyDetachBoundaryMembershipFamily,
    TopologyDetachRadialAdjacencyDeclaration, TopologyDetachRadialAdjacencyFamily,
    TopologyDetachShellOrWireMembershipDeclaration, TopologyDetachShellOrWireMembershipFamily,
    TopologyLoopSuccessorRewireMember, TopologyRadialSpliceMember,
    TopologyRehomeAllOwnedFacesToNewShellDeclaration, TopologyRehomeAllOwnedFacesToNewShellFamily,
    TopologyRehomeAllOwnedHalfEdgesToNewWireDeclaration,
    TopologyRehomeAllOwnedHalfEdgesToNewWireFamily, TopologyRetireTopologyEntityDeclaration,
    TopologyRetireTopologyEntityFamily, TopologyRewireLoopEndpointDeclaration,
    TopologyRewireLoopEndpointFamily, TopologyRewireLoopSuccessorProgramDeclaration,
    TopologyRewireLoopSuccessorProgramFamily, TopologyShellRehomeFaceMember,
    TopologySpliceRadialAdjacencyDeclaration, TopologySpliceRadialAdjacencyFamily,
    TopologySpliceRadialAdjacencyProgramDeclaration, TopologySpliceRadialAdjacencyProgramFamily,
    TopologySplitConnectedHalfEdgeSetToNewWireDeclaration,
    TopologySplitConnectedHalfEdgeSetToNewWireFamily,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellDeclaration,
    TopologySplitSingleFaceFromTwoFaceShellToNewShellFamily, TopologyWireRehomeHalfEdgeMember,
    TopologyWireSplitHalfEdgeMember,
};
pub(crate) use declared_mutation_sequence_builder::TopologyDeclaredMutationSequenceBuilder;
pub use facade::TopologyMutationApplicationMode;
pub use mutation_digest::{TopologyMutationDigest, TopologyMutationSequenceDigest};
pub(crate) use mutation_records::TopologyDeclaredMutationActionRef;
pub use mutation_records::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyMutationChangedScope, TopologyMutationDerivedFallbackPolicy,
    TopologyMutationFamily, TopologyMutationNamingOutcome, TopologyMutationNamingReport,
    TopologyMutationNamingRow, TopologyMutationNamingScope,
};
#[cfg(test)]
pub(crate) use mutation_sequence::topology_mutation_digest_for_records;
pub(crate) use mutation_sequence::TopologyDeclaredMutationMember;
pub(crate) use mutation_sequence::TopologyDeclaredMutationSequence;
pub use naming_continuity::NamingMutationContinuityMatrix;
pub use rejection_locality::{
    RejectedMutationScopeReport, RejectedMutationScopeRow, TopologyMutationRejectionClass,
};
