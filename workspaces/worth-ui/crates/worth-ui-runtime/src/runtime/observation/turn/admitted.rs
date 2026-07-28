use super::super::progress::UiObservationProgress;
use super::super::UiObservationFamily;

pub struct UiAdmittedObservation {
    family: UiObservationFamily,
    owner_order: u64,
    retained_bytes: usize,
    session: crate::facade::WorthUiActiveApplicationSessionIdentity,
    source_basis: u64,
    progress: Option<UiObservationProgress>,
    payload: UiAdmittedObservationPayload,
}

pub(in crate::runtime::observation) enum UiAdmittedObservationPayload {
    Source(crate::runtime::WorthUiWatchedCandidateSubmission),
    Host(super::super::admission::UiHostObservation),
    Measurement(crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult),
    Query(worth_ui_query_binding::WorthUiValidatedCollectionChangeObservation),
    CommittedScrollExtent(super::super::admission::UiCommittedScrollExtentObservation),
    CommittedPortalAnchor(super::super::admission::UiCommittedPortalAnchorObservation),
}

pub(in crate::runtime::observation) struct UiAdmittedObservationSeal {
    pub(in crate::runtime::observation) family: UiObservationFamily,
    pub(in crate::runtime::observation) owner_order: u64,
    pub(in crate::runtime::observation) retained_bytes: usize,
    pub(in crate::runtime::observation) session:
        crate::facade::WorthUiActiveApplicationSessionIdentity,
    pub(in crate::runtime::observation) source_basis: u64,
    pub(in crate::runtime::observation) progress: Option<UiObservationProgress>,
    pub(in crate::runtime::observation) payload: UiAdmittedObservationPayload,
}

impl UiAdmittedObservation {
    pub(in crate::runtime::observation) fn seal(input: UiAdmittedObservationSeal) -> Self {
        Self {
            family: input.family,
            owner_order: input.owner_order,
            retained_bytes: input.retained_bytes,
            session: input.session,
            source_basis: input.source_basis,
            progress: input.progress,
            payload: input.payload,
        }
    }

    pub const fn family(&self) -> UiObservationFamily {
        self.family
    }

    pub const fn owner_order(&self) -> u64 {
        self.owner_order
    }

    pub const fn retained_bytes(&self) -> usize {
        self.retained_bytes
    }

    pub(super) const fn session(&self) -> crate::facade::WorthUiActiveApplicationSessionIdentity {
        self.session
    }

    pub(super) const fn source_basis(&self) -> u64 {
        self.source_basis
    }

    pub(super) fn progress(&self) -> Option<&UiObservationProgress> {
        self.progress.as_ref()
    }

    pub fn source_observation(
        &self,
    ) -> Option<super::super::admission::UiAdmittedSourceObservation<'_>> {
        match &self.payload {
            UiAdmittedObservationPayload::Source(candidate) => Some(
                super::super::admission::UiAdmittedSourceObservation::new(candidate),
            ),
            UiAdmittedObservationPayload::Host(_)
            | UiAdmittedObservationPayload::Measurement(_)
            | UiAdmittedObservationPayload::Query(_)
            | UiAdmittedObservationPayload::CommittedScrollExtent(_)
            | UiAdmittedObservationPayload::CommittedPortalAnchor(_) => None,
        }
    }

    pub fn query_change_order(&self) -> Option<u64> {
        match &self.payload {
            UiAdmittedObservationPayload::Query(observation) => Some(observation.change_order()),
            UiAdmittedObservationPayload::Source(_)
            | UiAdmittedObservationPayload::Host(_)
            | UiAdmittedObservationPayload::Measurement(_)
            | UiAdmittedObservationPayload::CommittedScrollExtent(_)
            | UiAdmittedObservationPayload::CommittedPortalAnchor(_) => None,
        }
    }

    pub fn host_observation(&self) -> Option<&super::super::admission::UiHostObservation> {
        match &self.payload {
            UiAdmittedObservationPayload::Host(observation) => Some(observation),
            UiAdmittedObservationPayload::Source(_)
            | UiAdmittedObservationPayload::Measurement(_)
            | UiAdmittedObservationPayload::Query(_)
            | UiAdmittedObservationPayload::CommittedScrollExtent(_)
            | UiAdmittedObservationPayload::CommittedPortalAnchor(_) => None,
        }
    }

    pub fn measurement(
        &self,
    ) -> Option<&crate::host_exchange::measurement_admission::UiSolicitedHostMeasurementResult>
    {
        match &self.payload {
            UiAdmittedObservationPayload::Measurement(measurement) => Some(measurement),
            _ => None,
        }
    }

    pub fn committed_scroll_extent(
        &self,
    ) -> Option<&super::super::admission::UiCommittedScrollExtentObservation> {
        match &self.payload {
            UiAdmittedObservationPayload::CommittedScrollExtent(observation) => Some(observation),
            _ => None,
        }
    }

    pub fn committed_portal_anchor(
        &self,
    ) -> Option<&super::super::admission::UiCommittedPortalAnchorObservation> {
        match &self.payload {
            UiAdmittedObservationPayload::CommittedPortalAnchor(observation) => Some(observation),
            _ => None,
        }
    }
}
