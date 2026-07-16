use worth_store_replication::{
    ReplicationDeliveryKind, ReplicationProgressDenial, ReplicationProgressOutcome,
    ReplicationProgressOutcomeView, ReplicationPublicationDenial, ReplicationPublicationOutcome,
    ReplicationPublicationOutcomeView, ReplicationPublicationReadiness,
    ReplicationSourceAdmissionDenial, ReplicationSourceAdmissionOutcome,
    ReplicationSourceAdmissionOutcomeView,
};

use super::ReplicationAdmissionAction;

pub fn map_replication_source_admission_outcome(
    outcome: &ReplicationSourceAdmissionOutcome,
) -> ReplicationAdmissionAction {
    match outcome.view() {
        ReplicationSourceAdmissionOutcomeView::Admitted(_) => {
            ReplicationAdmissionAction::SourceAdmitted
        }
        ReplicationSourceAdmissionOutcomeView::Denied(denial) => match denial {
            ReplicationSourceAdmissionDenial::PeerIdentityRequired => {
                ReplicationAdmissionAction::SourcePeerIdentityDenied
            }
            ReplicationSourceAdmissionDenial::SourceEpochRequired => {
                ReplicationAdmissionAction::SourceEpochRequiredDenied
            }
            ReplicationSourceAdmissionDenial::LineageIdentityRequired => {
                ReplicationAdmissionAction::SourceLineageIdentityDenied
            }
            ReplicationSourceAdmissionDenial::CurrentAuthorityMismatch => {
                ReplicationAdmissionAction::SourceCurrentAuthorityDenied
            }
            ReplicationSourceAdmissionDenial::ReplayIdentityMismatch => {
                ReplicationAdmissionAction::SourceReplayIdentityDenied
            }
        },
    }
}

pub fn map_replication_progress_outcome(
    outcome: &ReplicationProgressOutcome,
) -> ReplicationAdmissionAction {
    match outcome.view() {
        ReplicationProgressOutcomeView::Observed(progress) => match progress.delivery_kind() {
            ReplicationDeliveryKind::Fresh => ReplicationAdmissionAction::FreshProgressObserved,
            ReplicationDeliveryKind::Resumed => ReplicationAdmissionAction::ResumeProgressObserved,
        },
        ReplicationProgressOutcomeView::Duplicate(_) => {
            ReplicationAdmissionAction::DuplicateObserved
        }
        ReplicationProgressOutcomeView::Denied(denial) => match denial {
            ReplicationProgressDenial::CurrentAuthorityMismatch => {
                ReplicationAdmissionAction::ResumeCurrentAuthorityDenied
            }
            ReplicationProgressDenial::SourceEpochMismatch => {
                ReplicationAdmissionAction::SourceEpochDivergenceDetected
            }
            ReplicationProgressDenial::LineageDivergence => {
                ReplicationAdmissionAction::LineageDivergenceDetected
            }
            ReplicationProgressDenial::DivergentReplayOverlap => {
                ReplicationAdmissionAction::ReplayOverlapDivergenceDetected
            }
            ReplicationProgressDenial::ReplayProgressGap => {
                ReplicationAdmissionAction::ResumeProgressGapDenied
            }
        },
    }
}

pub fn map_replication_publication_readiness(
    readiness: &ReplicationPublicationReadiness,
) -> ReplicationAdmissionAction {
    match readiness.delivery_kind() {
        ReplicationDeliveryKind::Fresh => ReplicationAdmissionAction::FreshPublicationPending,
        ReplicationDeliveryKind::Resumed => ReplicationAdmissionAction::ResumePublicationPending,
    }
}

pub fn map_replication_publication_outcome(
    outcome: &ReplicationPublicationOutcome,
) -> ReplicationAdmissionAction {
    match outcome.view() {
        ReplicationPublicationOutcomeView::Published(published) => {
            match published.delivery_kind() {
                ReplicationDeliveryKind::Fresh => {
                    ReplicationAdmissionAction::FreshPublicationDurable
                }
                ReplicationDeliveryKind::Resumed => {
                    ReplicationAdmissionAction::ResumePublicationDurable
                }
            }
        }
        ReplicationPublicationOutcomeView::Denied(denial) => match denial {
            ReplicationPublicationDenial::CurrentAuthorityChanged => {
                ReplicationAdmissionAction::PublicationCurrentAuthorityDenied
            }
            ReplicationPublicationDenial::PeerProgressChanged => {
                ReplicationAdmissionAction::PublicationPeerProgressChangedDenied
            }
            ReplicationPublicationDenial::PeerCapacityExceeded => {
                ReplicationAdmissionAction::PublicationPeerCapacityDenied
            }
            ReplicationPublicationDenial::ProgressStoreIo => {
                ReplicationAdmissionAction::PublicationProgressStoreDenied
            }
        },
    }
}
