use forge_proof::TransitionOutcome;
use forge_store_aspect_native::StoreAspectIdentity;

use crate::{
    StoreHostileReadmissionJsonFixtureBoundaryOutcome,
    StoreHostileReadmissionJsonFixtureBoundaryWitness,
    json_fixture_boundary::require_hostile_readmission_boundary,
};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreHostileReadmissionJsonFixture<T> {
    identity: StoreAspectIdentity,
    attacker_document: T,
}

impl<T> StoreHostileReadmissionJsonFixture<T> {
    pub fn attacker_document(identity: StoreAspectIdentity, attacker_document: T) -> Self {
        Self {
            identity,
            attacker_document,
        }
    }

    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn attacker_document_value(
        &self,
        _boundary: StoreHostileReadmissionJsonFixtureBoundaryWitness,
    ) -> &T {
        &self.attacker_document
    }

    pub fn into_attacker_document(
        self,
        _boundary: StoreHostileReadmissionJsonFixtureBoundaryWitness,
    ) -> T {
        self.attacker_document
    }

    #[track_caller]
    pub fn allow_in_hostile_readmission_suite(
        &self,
    ) -> StoreHostileReadmissionJsonFixtureBoundaryOutcome {
        require_hostile_readmission_boundary()
    }

    pub fn deny_non_terminal_fixture_use(
        &self,
    ) -> StoreHostileReadmissionJsonFixtureBoundaryOutcome {
        TransitionOutcome::denied(
            crate::StoreJsonFixtureBoundaryDenial::HostileReadmissionJsonRequiresHostileReadmissionSuite,
        )
    }
}
