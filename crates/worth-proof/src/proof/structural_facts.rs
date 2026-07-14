use super::markers::ProofMarker;
use super::sets::AuthorityProves;
use super::witnesses::AuthorityMarker;

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct CanonicalOrder;

impl ProofMarker for CanonicalOrder {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Uniqueness;

impl ProofMarker for Uniqueness {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Disjointness;

impl ProofMarker for Disjointness {}

#[derive(Debug, Clone, Copy, Default, PartialEq, Eq)]
pub struct Normalization;

impl ProofMarker for Normalization {}

#[derive(Debug, PartialEq, Eq)]
pub struct StructuralProofAuthority(());

impl AuthorityMarker for StructuralProofAuthority {}

impl AuthorityProves<CanonicalOrder> for StructuralProofAuthority {}
impl AuthorityProves<Uniqueness> for StructuralProofAuthority {}
impl AuthorityProves<Disjointness> for StructuralProofAuthority {}
impl AuthorityProves<Normalization> for StructuralProofAuthority {}
