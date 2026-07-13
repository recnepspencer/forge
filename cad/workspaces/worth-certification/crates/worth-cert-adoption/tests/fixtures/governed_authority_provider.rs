use worth_proof::{
    AuthorityMarker, AuthorityProves, AuthorityWitness, CapabilityMarker, CapabilityWitness, Proof,
    ProofMarker,
};

pub struct EntryAdmission {
    _private: (),
}

impl AuthorityMarker for EntryAdmission {}

pub struct EntryExecution {
    _private: (),
}

impl CapabilityMarker for EntryExecution {}

pub struct EntryAdmissionGranted;

impl ProofMarker for EntryAdmissionGranted {}
impl AuthorityProves<EntryAdmissionGranted> for EntryAdmission {}

pub fn admit_authority_only(_authority: AuthorityWitness<EntryAdmission>) {}

pub fn admit_capability(_capability: CapabilityWitness<EntryExecution>) {}

pub fn admit_proof(_proof: Proof<EntryAdmissionGranted, EntryAdmission>) {}

pub fn issue_entry_admission() -> AuthorityWitness<EntryAdmission> {
    AuthorityWitness::from_authority_marker(EntryAdmission { _private: () })
}

pub fn issue_entry_execution() -> CapabilityWitness<EntryExecution> {
    CapabilityWitness::from_capability_marker(EntryExecution { _private: () })
}
