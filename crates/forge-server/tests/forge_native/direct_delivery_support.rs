use forge_proof::TransitionOutcome;
use forge_server::{
    ForgeServerDirectDeliveryClass, ForgeServerDirectDeliveryOutcome,
    ForgeServerDirectDeliveryRequest, ForgeServerDirectFreshnessMode,
    ForgeServerDirectLeaseDeclarationOutcome, ForgeServerQueryRequestedResume,
};

pub(crate) fn direct_request(
    requested_resume: ForgeServerQueryRequestedResume,
) -> ForgeServerDirectDeliveryRequest {
    ForgeServerDirectDeliveryRequest::new(
        ForgeServerDirectFreshnessMode::LiveStrict,
        ForgeServerDirectDeliveryClass::AuthoritativeOrdered,
        requested_resume,
    )
}

pub(crate) fn direct_delivery_success(
    outcome: ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerDirectDeliveryContract {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct delivery contract, got {other:?}"),
    }
}

pub(crate) fn direct_lease_success(
    outcome: ForgeServerDirectLeaseDeclarationOutcome,
) -> forge_server::ForgeServerDirectLeaseDeclaration {
    match outcome {
        TransitionOutcome::Success(value) => value,
        other => panic!("expected successful direct lease declaration, got {other:?}"),
    }
}

pub(crate) fn direct_lease_denied(
    outcome: ForgeServerDirectLeaseDeclarationOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct lease declaration, got {other:?}"),
    }
}

pub(crate) fn direct_delivery_denied(
    outcome: ForgeServerDirectDeliveryOutcome,
) -> forge_server::ForgeServerQueryHandoffDenial {
    match outcome {
        TransitionOutcome::Denied(denial) => denial,
        other => panic!("expected denied direct delivery contract, got {other:?}"),
    }
}
