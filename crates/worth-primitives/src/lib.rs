mod canonical_witness_geometry;
mod digest_protocol;
mod family_contract;
mod geometry_identity;

pub use canonical_witness_geometry::{
    canonical_orthotope_vertices, canonical_prism_vertices, canonical_pyramid_vertices,
    canonical_simplex_vertices, canonical_wire_body_vertices, derive_shell_with_hole_layout,
    shell_with_hole_vertices_from_layout, PrimitiveCanonicalWitnessGeometry,
    PrimitivePlanarWitnessAuthority, ShellWithHoleLayoutLegality, ShellWithHoleWitnessLayout,
    ShellWithHoleWitnessLayoutError, ShellWithHoleWitnessLayoutPolicy,
    CANONICAL_SIMPLEX_LATERAL_RATIO,
};
pub use digest_protocol::{
    truth_digest_parts, TruthDigestScope, TruthDigestVersion, DIGEST_VERSION,
};
pub use family_contract::{
    PrimitiveConstructionBirthSynopsisContract, PrimitiveConstructionFamilyContract,
    PrimitiveConstructionFamilyContractRegistry, PrimitiveConstructionFamilyKey,
    PrimitiveConstructionSupportContract, PrimitiveConstructionTopologyContract,
    PrimitiveWitnessDescriptor, PrimitiveWitnessSupportSummary, PrimitiveWitnessTopologySummary,
};
pub use geometry_identity::{
    PrimitiveCurvedSupportIdentity, PrimitiveGeometryIdentityBundle,
    PrimitiveRealizationGeometryDigest, PrimitiveRealizedSupportIdentity,
    PrimitiveScaffoldGeometryDigest, PrimitiveSupportPlaneIdentity,
    PrimitiveTriaxialEllipsoidIdentity, PrimitiveVertexIdentity,
};
