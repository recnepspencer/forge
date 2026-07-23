use worth_foundational::facade::CanonicalEquivalentBasis;
use worth_proof::{AuthorityMarker, AuthorityWitness};
use worth_query::facade::domain::{
    WorthQueryCompiledSemanticAspectDependency, WorthQuerySharedLiveProjectionPair,
};
use worth_query::facade::foundation::ObservationLaneWitness;

struct ForgedAuthority;
impl AuthorityMarker for ForgedAuthority {}

type Sharing = WorthQuerySharedLiveProjectionPair<(), (), (), ObservationLaneWitness>;

fn from_foundational(equivalence: CanonicalEquivalentBasis) -> Sharing {
    equivalence.into()
}

fn from_generic_proof(witness: AuthorityWitness<ForgedAuthority>) -> Sharing {
    witness.into()
}

fn from_raw_closure(raw: Vec<WorthQueryCompiledSemanticAspectDependency>) -> Sharing {
    raw.into()
}

fn main() {}
