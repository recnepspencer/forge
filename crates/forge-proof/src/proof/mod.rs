mod markers;
mod minting;
mod sets;
mod structural_facts;
mod witnesses;

pub use markers::ProofMarker;
#[cfg(test)]
pub(crate) use minting::mint_proof;
pub use sets::{NoProofs, Proof, ProofSet, ProofSetCons};
pub use structural_facts::{CanonicalOrder, Disjointness, Normalization, Uniqueness};
#[cfg(test)]
pub(crate) use witnesses::{mint_authority_witness, mint_capability_witness};
pub use witnesses::{AuthorityMarker, AuthorityWitness, CapabilityMarker, CapabilityWitness};
