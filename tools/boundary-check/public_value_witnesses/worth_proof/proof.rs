//! Proof values minted only from downstream-owned sealed markers.

pub(crate) struct WitnessProof;
impl worth_proof::ProofMarker for WitnessProof {}

worth_proof::authority_marker!(pub(crate) WitnessAuthority);
impl worth_proof::AuthorityProves<WitnessProof> for WitnessAuthority {}

fn authority() -> worth_proof::AuthorityWitness<WitnessAuthority> {
    WitnessAuthority::witness()
}

pub(crate) fn no_proofs() -> worth_proof::NoProofs {
    worth_proof::NoProofs
}

pub(crate) fn canonical_order() -> worth_proof::CanonicalOrder {
    worth_proof::CanonicalOrder
}

pub(crate) fn uniqueness() -> worth_proof::Uniqueness {
    worth_proof::Uniqueness
}

pub(crate) fn disjointness() -> worth_proof::Disjointness {
    worth_proof::Disjointness
}

pub(crate) fn normalization() -> worth_proof::Normalization {
    worth_proof::Normalization
}

pub(crate) fn proof() -> worth_proof::Proof<WitnessProof, WitnessAuthority> {
    worth_proof::Proof::from_authority_witness(&authority())
}

pub(crate) fn proof_set_cons() -> worth_proof::ProofSetCons<
    worth_proof::Proof<WitnessProof, WitnessAuthority>,
    worth_proof::NoProofs,
> {
    worth_proof::ProofSetCons::new(proof(), worth_proof::NoProofs)
}

pub(crate) fn authority_witness() -> worth_proof::AuthorityWitness<WitnessAuthority> {
    authority()
}

worth_proof::capability_marker!(pub(crate) WitnessCapability);

pub(crate) fn capability_witness() -> worth_proof::CapabilityWitness<WitnessCapability> {
    WitnessCapability::witness()
}
