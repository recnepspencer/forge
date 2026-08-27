use std::collections::BTreeMap;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiMotionStagingDenial {
    CapacityExceeded,
    TrackIdentityExhausted,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum UiMotionCommitDenial {
    PreparedFrameMismatch,
    PublicationProposalMismatch,
    PublicationRejected,
    Census,
}

/// Canonical semantic motion-track owner. Presentation sampling receives only
/// committed track copies and cannot mutate this table.
pub(crate) struct UiMotionRuntimeState {
    persistence: crate::runtime::UiServiceStatePersistencePosture,
    next_track_identity: u64,
    tracks: BTreeMap<super::UiMotionTargetIdentity, super::UiCommittedMotionTrack>,
    exit_retentions: BTreeMap<super::UiMotionTrackIdentity, super::UiMotionExitRetentionReceipt>,
    census: super::UiMotionResourceCensus,
    publication_sequence: u64,
    last_fact: Option<super::UiMotionProducedFact>,
}

impl UiMotionRuntimeState {
    pub(crate) const fn new(persistence: crate::runtime::UiServiceStatePersistencePosture) -> Self {
        Self {
            persistence,
            next_track_identity: 1,
            tracks: BTreeMap::new(),
            exit_retentions: BTreeMap::new(),
            census: super::UiMotionResourceCensus::zero(),
            publication_sequence: 0,
            last_fact: None,
        }
    }

    pub(in crate::runtime) const fn persistence(
        &self,
    ) -> crate::runtime::UiServiceStatePersistencePosture {
        self.persistence
    }

    pub(in crate::runtime) fn stage(
        &mut self,
        proposal: crate::runtime::session::service_proposal::UiServiceProposalIdentity,
        request: super::UiMotionTransitionRequest,
    ) -> Result<super::UiStagedMotionServiceProposal, UiMotionStagingDenial> {
        self.census
            .stage()
            .map_err(|_| UiMotionStagingDenial::CapacityExceeded)?;
        let Some(identity) = super::UiMotionTrackIdentity::allocate(self.next_track_identity)
        else {
            self.rollback_staging();
            return Err(UiMotionStagingDenial::TrackIdentityExhausted);
        };
        self.next_track_identity = match self.next_track_identity.checked_add(1) {
            Some(next) => next,
            None => {
                self.rollback_staging();
                return Err(UiMotionStagingDenial::TrackIdentityExhausted);
            }
        };
        let scope = crate::runtime::session::service_proposal::
            UiServiceProposalOccupancyScopeIdentity::for_mounted_owner(
                request.successor().target().mounted_instance(),
            );
        Ok(super::UiStagedMotionServiceProposal {
            identity,
            proposal,
            scope,
            request,
            fact: crate::runtime::session::service_proposal::UiServiceProducedFactReference::for_motion_proposal(
                proposal,
                scope,
            ),
        })
    }

    pub(in crate::runtime) const fn derive(
        &self,
        staged: super::UiStagedMotionServiceProposal,
        prepared_frame: worth_ui_host_contract::UiMountedFrameIdentity,
    ) -> super::UiDerivedMotionServiceProposal {
        super::UiDerivedMotionServiceProposal {
            staged,
            prepared_frame,
        }
    }

    pub(in crate::runtime) fn discard_staged(
        &mut self,
        _staged: super::UiStagedMotionServiceProposal,
    ) {
        self.rollback_staging();
    }

    pub(in crate::runtime) fn discard_derived(
        &mut self,
        _derived: super::UiDerivedMotionServiceProposal,
    ) {
        self.rollback_staging();
    }

    pub(in crate::runtime) fn commit_published(
        &mut self,
        mut derived: super::UiDerivedMotionServiceProposal,
        publication: crate::runtime::session::service_proposal::UiServiceProposalPublicationReceipt,
        mounted_frame: worth_ui_host_contract::UiMountedFrameIdentity,
        mounted_presentation: worth_ui_host_contract::UiHostObservationPresentationBasis,
    ) -> Result<
        super::UiMotionCommitReceipt,
        (super::UiDerivedMotionServiceProposal, UiMotionCommitDenial),
    > {
        if derived.prepared_frame != mounted_frame {
            return Err((derived, UiMotionCommitDenial::PreparedFrameMismatch));
        }
        if derived.staged.proposal != publication.proposal() {
            return Err((derived, UiMotionCommitDenial::PublicationProposalMismatch));
        }
        if publication.disposition()
            != crate::runtime::session::service_proposal::UiServiceProposalPublicationDisposition::Accepted
        {
            return Err((derived, UiMotionCommitDenial::PublicationRejected));
        }
        derived.staged.request = match derived
            .staged
            .request
            .bind_published_successor(mounted_presentation)
        {
            Ok(request) => request,
            Err(_) => return Err((derived, UiMotionCommitDenial::PreparedFrameMismatch)),
        };
        let target = derived.staged.request.successor().target();
        let retarget = self
            .tracks
            .get(&target)
            .map(|_| super::retarget::resolve(derived.staged.request.declaration().interruption()));
        let displaced_exit_retention = self
            .exit_retentions
            .values()
            .find(|retention| retention.target() == target)
            .copied();
        let retains_exit =
            derived.staged.request.declaration().fill() == super::UiMotionFillPolicy::ExitRetention;
        let reserves_exit_retention = retains_exit && displaced_exit_retention.is_none();
        if reserves_exit_retention && self.census.reserve_exit_retention().is_err() {
            return Err((derived, UiMotionCommitDenial::Census));
        }
        if self.census.commit_staged(retarget.is_some()).is_err() {
            if reserves_exit_retention {
                self.census
                    .release_exit_retention()
                    .expect("same commit reserved the exit-retention census");
            }
            return Err((derived, UiMotionCommitDenial::Census));
        }
        let track = super::UiCommittedMotionTrack::new(&derived, retarget);
        let exit_retention = retains_exit.then(|| super::UiMotionExitRetentionReceipt::new(track));
        if let Some(displaced) = displaced_exit_retention {
            assert_eq!(
                self.exit_retentions.remove(&displaced.track()),
                Some(displaced),
                "Motion interruption displaces its exact retained exit"
            );
            if !retains_exit {
                self.census
                    .release_exit_retention()
                    .expect("displaced Motion exit owns one census entry");
            }
        }
        if let Some(exit_retention) = exit_retention {
            self.exit_retentions
                .insert(track.identity(), exit_retention);
        }
        let kind = retarget.map_or(
            super::UiMotionProducedFactKind::Started,
            super::UiMotionProducedFactKind::Retargeted,
        );
        let fact = self.publish(track.identity(), track.request(), kind);
        self.tracks.insert(target, track);
        Ok(super::UiMotionCommitReceipt::new(
            track,
            fact,
            exit_retention,
            displaced_exit_retention,
        ))
    }

    pub(crate) fn terminalize(
        &mut self,
        track: super::UiMotionTrackIdentity,
        cause: super::UiMotionTerminalCause,
    ) -> Option<super::UiMotionTerminalReceipt> {
        let target = self
            .tracks
            .iter()
            .find_map(|(target, candidate)| (candidate.identity() == track).then_some(*target))?;
        self.terminalize_target(target, cause)
    }

    pub(super) fn terminalize_target(
        &mut self,
        target: super::UiMotionTargetIdentity,
        cause: super::UiMotionTerminalCause,
    ) -> Option<super::UiMotionTerminalReceipt> {
        let track = self.tracks.remove(&target)?;
        self.census
            .terminal()
            .expect("terminal Motion track retains one active census entry");
        let fact = self.publish(
            track.identity(),
            track.request(),
            super::UiMotionProducedFactKind::Terminal(cause),
        );
        let exit_retention = self.exit_retentions.get(&track.identity()).copied();
        Some(super::UiMotionTerminalReceipt::new(
            track,
            cause,
            fact,
            exit_retention,
        ))
    }

    pub(crate) fn release_exit_retention(
        &mut self,
        retention: super::UiMotionExitRetentionReceipt,
    ) -> bool {
        if self.exit_retentions.get(&retention.track()) != Some(&retention) {
            return false;
        }
        self.exit_retentions.remove(&retention.track());
        self.census
            .release_exit_retention()
            .expect("retained Motion exit owns one census entry");
        true
    }

    fn publish(
        &mut self,
        track: super::UiMotionTrackIdentity,
        request: super::UiMotionTransitionRequest,
        kind: super::UiMotionProducedFactKind,
    ) -> super::UiMotionProducedFact {
        self.publication_sequence = self
            .publication_sequence
            .checked_add(1)
            .expect("bounded Motion fact sequence exhausted");
        let fact =
            super::UiMotionProducedFact::new(self.publication_sequence, track, request, kind);
        self.last_fact = Some(fact);
        fact
    }

    fn rollback_staging(&mut self) {
        self.census
            .discard_staged()
            .expect("discarded Motion proposal retains one census entry");
    }

    pub(in crate::runtime) const fn census(&self) -> super::UiMotionResourceCensus {
        self.census
    }

    pub(crate) const fn publication_count(&self) -> u64 {
        self.publication_sequence
    }

    pub(in crate::runtime) const fn last_fact(&self) -> Option<super::UiMotionProducedFact> {
        self.last_fact
    }

    pub(crate) fn shutdown(&mut self) -> super::UiMotionShutdownReport {
        let census_before_shutdown = self.census;
        let targets: Vec<_> = self.tracks.keys().copied().collect();
        for target in targets {
            self.terminalize_target(target, super::UiMotionTerminalCause::ApplicationShutdown)
                .expect("shutdown target was retained by the canonical Motion table");
        }
        let report = super::UiMotionShutdownReport::from_census(census_before_shutdown);
        self.exit_retentions.clear();
        self.census = report.final_census();
        report
    }
}
