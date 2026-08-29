mod cancellation;
mod census;
mod compiler;
mod coordination;
mod occupancy;
mod participation;
mod request_basis;

#[cfg(any(test, feature = "certification-support"))]
use participation::fixture_service_family_participation;
#[cfg(all(feature = "certification-support", not(test)))]
use request_basis::fixture_service_request_coherence;
#[cfg(test)]
use request_basis::{fixture_application_generation_in_session, fixture_service_request_coherence};

pub(in crate::runtime) use cancellation::UiServiceProposalCancellationDenial;
pub(in crate::runtime) use census::{UiServiceProposalCensus, UiServiceProposalCensusDenial};
#[cfg(feature = "certification-support")]
pub(crate) use compiler::proposal_scale_evidence;
pub(in crate::runtime) use compiler::{
    UiPreflightedServiceProposal, UiReservedServiceProposal, UiServiceFamilyProposal,
    UiServiceMountedWorkReference, UiServiceProducedFactReference,
    UiServiceProposalBeforeEffectCancellationReceipt, UiServiceProposalCandidate,
    UiServiceProposalCompiler, UiServiceProposalCompilerShutdownReceipt, UiServiceProposalDemand,
    UiServiceProposalDemandConstructionDenial, UiServiceProposalDependencyEdge,
    UiServiceProposalIdentity, UiServiceProposalOwnerAcknowledgement,
    UiServiceProposalPreflightDenial, UiServiceProposalPublicationDenial,
    UiServiceProposalPublicationDisposition, UiServiceProposalPublicationReceipt,
    UiServiceProposalReservationDenial, UiServiceProposalReservationOutcome,
    UiServiceProposalSettlement, UiServiceProposalSettlementDenial, UiServiceProposalStage,
    UiServiceProposalStageReceipt, UiServiceProposalStagedBatch, UiServiceProposalStaging,
    UiServiceProposalStagingDenial, UiServiceProposalTeardown, UiServiceProposalTeardownDenial,
    UiServiceProposalTerminalOwnerOutcome, UiServiceProposalTerminalReason,
    UiServiceProposalTerminalReceipt,
};
pub(in crate::runtime) use coordination::{
    UiDeclaredFocusSelectionAction, UiFocusRevealRequirement, UiSelectionInvocationCause,
};
pub(in crate::runtime) use occupancy::{
    UiServiceProposalConflictDisposition, UiServiceProposalConflictPolicy,
    UiServiceProposalDisplacement, UiServiceProposalOccupancyDenial,
    UiServiceProposalOccupancyLease, UiServiceProposalOccupancyScopeIdentity,
    UiServiceProposalOccupancyWorkCounters,
};
pub(in crate::runtime) use participation::{
    UiServiceFamilyParticipation, UiServiceFamilyParticipationDenial,
};

pub(in crate::runtime) use request_basis::{
    UiAdmittedIntentServiceRequestAuthority, UiPortalDismissalServiceRequestAuthority,
    UiPortalExitTerminalServiceRequestAuthority, UiServiceCancellationIdentity,
    UiServiceRequestBasis, UiServiceRequestBasisDenial, UiServiceRequestCoherence,
    UiServiceRequestCoherenceAxes, UiServiceRequestCoherenceDrift, UiServiceRequestIdentity,
    UiServiceRequestOrigin, UiServiceRequestOriginAuthority, UiServiceResourceBudgetIdentity,
    UiServiceSourceOrder, UiServiceSurfaceBasis,
};
