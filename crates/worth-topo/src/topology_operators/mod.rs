pub(crate) mod application;
mod contract_sequence;
mod contracts;
mod declaration_entry;
mod facade;
mod local_rewrites;
mod naming_continuity;
mod rejection_locality;
mod replay;

pub(crate) use application::topology_relation_dependency_path;
pub(crate) use application::TopologyOperatorExecutionError;
pub(crate) use application::TopologyOperatorRunner;
pub use application::{TopologyDeclarationEntryRefusalClass, TopologyDeclarationEntryStopClass};
pub(crate) use contract_sequence::{
    naming_edit_continuity_matrix_for_contracts, topology_edit_digest_for_contracts,
    topology_edit_families_for_contracts, topology_edit_naming_report_for_contracts,
};
pub use contracts::{
    BoundaryMembershipKind, LoopEndpointKind, LoopSuccessorKind, ShellOrWireMembershipKind,
    TopologyDerivedRegion, TopologyEditAction, TopologyEditChangedScope, TopologyEditContract,
    TopologyEditDerivedFallbackPolicy, TopologyEditFamily, TopologyEditNamingOutcome,
    TopologyEditNamingReport, TopologyEditNamingRow, TopologyEditNamingScope,
};
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
pub use facade::TopologyEditApplicationMode;
pub use naming_continuity::NamingEditContinuityMatrix;
pub use rejection_locality::{
    RejectedEditScopeReport, RejectedEditScopeRow, TopologyEditRejectionClass,
};
pub use replay::{TopologyEditDigest, TopologyOperatorDigest};
