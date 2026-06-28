use forge_foundational::{
    compatibility, AspectContract, AuthoritativeRecordAspectStateArtifact, BoundarySourceLocator,
    JsonCompatibilityLoweringDeferred, JsonCompatibilityLoweringFailure,
    JsonCompatibilityLoweringStale, JsonCompatibilityRebindRequired,
};
use forge_proof::TransitionOutcome;
use serde_json::Value;

use crate::{
    StoreAspectAuthorityInput, StoreAspectBoundaryFact, StoreAspectIdentity,
    StoreAspectNativeDenial, StorePhysicalBoundaryWitness, StoreTerminalJsonProjection,
    StoreTerminalProjectionDenial,
};

pub type StoreTerminalJsonReadmissionOutcome = TransitionOutcome<
    StoreTerminalJsonReadmission,
    StoreTerminalProjectionDenial,
    JsonCompatibilityLoweringDeferred,
    JsonCompatibilityLoweringStale,
    JsonCompatibilityRebindRequired,
    JsonCompatibilityLoweringFailure,
>;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct StoreTerminalJsonReadmission {
    identity: StoreAspectIdentity,
    admitted_state: AuthoritativeRecordAspectStateArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
}

impl StoreTerminalJsonReadmission {
    pub const fn identity(&self) -> &StoreAspectIdentity {
        &self.identity
    }

    pub const fn admitted_state(&self) -> &AuthoritativeRecordAspectStateArtifact {
        &self.admitted_state
    }

    pub const fn physical_witness(&self) -> StorePhysicalBoundaryWitness {
        self.physical_witness
    }

    pub fn rebuild_store_boundary_fact(
        &self,
    ) -> Result<StoreAspectBoundaryFact, StoreAspectNativeDenial> {
        StoreAspectBoundaryFact::from_admitted_state(
            self.identity.clone(),
            StoreAspectAuthorityInput::new(self.admitted_state.clone(), self.physical_witness),
        )
    }
}

pub fn readmit_terminal_json_projection_as_store_aspect_state(
    projection: StoreTerminalJsonProjection,
    contract: AspectContract,
    source: BoundarySourceLocator,
    physical_witness: StorePhysicalBoundaryWitness,
) -> StoreTerminalJsonReadmissionOutcome {
    let identity = projection.terminal_projection_identity().clone();
    if !terminal_projection_identity_matches_contract(&identity, &contract) {
        return TransitionOutcome::denied(StoreTerminalProjectionDenial::ContractIdentityMismatch);
    }

    let admitted_state_outcome = lower_terminal_projection_document_to_native_state(
        contract,
        source,
        projection.into_terminal_projection_document(),
    );
    match admitted_state_outcome {
        TransitionOutcome::Success(admitted_state) => {
            readmit_native_state_as_store_boundary(identity, admitted_state, physical_witness)
        }
        TransitionOutcome::Denied(denial) => TransitionOutcome::denied(
            StoreTerminalProjectionDenial::JsonCompatibilityDenied(denial),
        ),
        TransitionOutcome::Deferred(deferred) => TransitionOutcome::deferred(deferred),
        TransitionOutcome::Stale(stale) => TransitionOutcome::stale(stale),
        TransitionOutcome::RebindRequired(rebind) => TransitionOutcome::rebind_required(rebind),
        TransitionOutcome::Failed(failure) => TransitionOutcome::failed(failure),
    }
}

pub fn readmit_external_terminal_projection_document_as_store_aspect_state(
    identity: StoreAspectIdentity,
    terminal_projection_document: Value,
    contract: AspectContract,
    source: BoundarySourceLocator,
    physical_witness: StorePhysicalBoundaryWitness,
) -> StoreTerminalJsonReadmissionOutcome {
    readmit_terminal_json_projection_as_store_aspect_state(
        StoreTerminalJsonProjection::from_terminal_projection_document(
            identity,
            terminal_projection_document,
        ),
        contract,
        source,
        physical_witness,
    )
}

fn terminal_projection_identity_matches_contract(
    identity: &StoreAspectIdentity,
    contract: &AspectContract,
) -> bool {
    identity.aspect_key() == contract.key()
}

fn lower_terminal_projection_document_to_native_state(
    contract: AspectContract,
    source: BoundarySourceLocator,
    terminal_projection_document: Value,
) -> TransitionOutcome<
    AuthoritativeRecordAspectStateArtifact,
    forge_foundational::JsonCompatibilityLoweringDenial,
    JsonCompatibilityLoweringDeferred,
    JsonCompatibilityLoweringStale,
    JsonCompatibilityRebindRequired,
    JsonCompatibilityLoweringFailure,
> {
    let input = compatibility()
        .json()
        .input(contract, source, terminal_projection_document);
    compatibility().json().lower_state([input])
}

fn readmit_native_state_as_store_boundary(
    identity: StoreAspectIdentity,
    admitted_state: AuthoritativeRecordAspectStateArtifact,
    physical_witness: StorePhysicalBoundaryWitness,
) -> StoreTerminalJsonReadmissionOutcome {
    match StoreAspectBoundaryFact::from_admitted_state(
        identity.clone(),
        StoreAspectAuthorityInput::new(admitted_state.clone(), physical_witness),
    ) {
        Ok(_) => TransitionOutcome::success(StoreTerminalJsonReadmission {
            identity,
            admitted_state,
            physical_witness,
        }),
        Err(denial) => {
            TransitionOutcome::denied(StoreTerminalProjectionDenial::StoreAuthorityDenied(denial))
        }
    }
}
