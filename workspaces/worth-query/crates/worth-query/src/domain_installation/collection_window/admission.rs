use worth_proof::{
    Artifact, AssumptionBasis, AuthorityMarker, AuthorityWitness, CurrentValidity,
    FreshnessScopedBasis, NoProofs, PhaseMarker,
};

use super::{
    WorthQueryCollectionCursor, WorthQueryCollectionWindowBreadth,
    WorthQueryCollectionWindowCounters,
};
use crate::domain_installation::WorthQueryBoundCapabilityGeneration;

pub(super) struct WorthQueryCollectionWindowAdmissionPhase;
impl PhaseMarker for WorthQueryCollectionWindowAdmissionPhase {}

struct WorthQueryCollectionWindowAdmissionAuthority;
impl AuthorityMarker for WorthQueryCollectionWindowAdmissionAuthority {}

pub(super) struct WorthQueryCollectionWindowAdmissionBasis {
    pub capability_identity: u64,
    pub capability_generation: WorthQueryBoundCapabilityGeneration,
    pub basis_identity: String,
    pub ordering_identity: String,
    pub start_row_ordinal: usize,
    pub admitted_width: usize,
}

pub(super) struct WorthQueryCollectionWindowAdmissionEvidence {
    pub identity: String,
}

type WorthQueryCollectionWindowAdmissionProof = Artifact<
    WorthQueryCollectionWindowAdmissionPhase,
    WorthQueryCollectionWindowAdmissionEvidence,
    NoProofs,
    FreshnessScopedBasis<
        CurrentValidity,
        AssumptionBasis<WorthQueryCollectionWindowAdmissionBasis>,
    >,
>;

pub struct WorthQueryAdmittedCollectionWindow {
    pub(super) cursor: WorthQueryCollectionCursor,
    pub(super) breadth: WorthQueryCollectionWindowBreadth,
    pub(super) counters: WorthQueryCollectionWindowCounters,
    proof: WorthQueryCollectionWindowAdmissionProof,
}

impl WorthQueryAdmittedCollectionWindow {
    pub(super) fn mint(
        cursor: WorthQueryCollectionCursor,
        breadth: WorthQueryCollectionWindowBreadth,
        counters: WorthQueryCollectionWindowCounters,
    ) -> Self {
        let basis = WorthQueryCollectionWindowAdmissionBasis {
            capability_identity: cursor.capability_identity,
            capability_generation: cursor.capability_generation,
            basis_identity: cursor.basis_identity.clone(),
            ordering_identity: cursor.ordering_identity.clone(),
            start_row_ordinal: cursor.next_row_ordinal,
            admitted_width: breadth.admitted_width() as usize,
        };
        let identity = crate::identity::hash_parts(&[
            "worth_query_collection_window_admission_v1".into(),
            format!("capability:{}", basis.capability_identity),
            format!("generation:{}", basis.capability_generation.ordinal()),
            format!("basis:{}", basis.basis_identity),
            format!("ordering:{}", basis.ordering_identity),
            format!("start:{}", basis.start_row_ordinal),
            format!("width:{}", basis.admitted_width),
        ]);
        let proof = Artifact::with_current_basis(
            WorthQueryCollectionWindowAdmissionEvidence {
                identity: identity.clone(),
            },
            basis,
            AuthorityWitness::from_authority_marker(WorthQueryCollectionWindowAdmissionAuthority),
        );
        Self {
            cursor,
            breadth,
            counters,
            proof,
        }
    }

    pub(super) fn basis(&self) -> &WorthQueryCollectionWindowAdmissionBasis {
        self.proof.strong_basis().value()
    }

    pub fn identity(&self) -> &str {
        &self.proof.payload().identity
    }

    pub const fn counters(&self) -> WorthQueryCollectionWindowCounters {
        self.counters
    }
}
