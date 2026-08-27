mod cancellation;
mod census;
mod compiler;
mod occupancy;
mod participation;
mod request_basis;

#[cfg(test)]
use participation::fixture_service_family_participation;
#[cfg(test)]
use request_basis::{fixture_application_generation_in_session, fixture_service_request_coherence};

pub(in crate::runtime) use cancellation::UiServiceProposalCancellationDenial;
pub(in crate::runtime) use census::{UiServiceProposalCensus, UiServiceProposalCensusDenial};
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
