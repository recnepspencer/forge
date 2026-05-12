use super::markers::ProofMarker;

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
