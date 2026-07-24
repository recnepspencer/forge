#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct UiDragResizeCounters {
    admitted_samples: u16,
    preview_publications: u16,
    durable_mutations: u16,
    committed_receipts: u16,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiResizePreviewCandidate {
    frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    target: crate::graph::UiGraphNodeIdentity,
    extent: super::UiResizeLogicalExtent,
    allocation_candidates: Box<[crate::runtime::UiAllocationPreviewCandidate]>,
}

#[derive(Clone, Debug, PartialEq)]
pub(crate) struct UiResizePreviewOutcome {
    candidate: UiResizePreviewCandidate,
    counters: UiDragResizeCounters,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiDurableResizeCommitOutcome {
    extent: super::UiResizeLogicalExtent,
    transaction: crate::runtime::UiCommittedAllocationReplan,
    durable_state: crate::runtime::UiAllocationDurableSemanticState,
    counters: UiDragResizeCounters,
    evidence: crate::evidence::UiDragResizeEvidence,
}

#[derive(Clone, Debug, PartialEq)]
pub struct UiDurableResizeCommitDenialReport {
    denial: crate::runtime::UiAllocationReplanTransactionCommitDenial,
    identity_digest: u64,
    extent: super::UiResizeLogicalExtent,
    counters: UiDragResizeCounters,
    evidence: crate::evidence::UiDragResizeEvidence,
}

impl UiResizePreviewOutcome {
    pub(crate) fn from_selection(
        plan: &crate::runtime::UiNarrowedAllocationFramePlan,
        selection: &crate::graph::UiAdmittedReplanNeighborhoodSet,
    ) -> Result<Self, crate::runtime::UiAllocationReplanTransactionCommitDenial> {
        let sample = plan
            .resize_preview_sample()
            .ok_or(crate::runtime::UiAllocationReplanTransactionCommitDenial::MissingSelection)?;
        let basis = crate::runtime::UiResizeAllocationPlanningBasis::seal(
            selection,
            sample.target(),
            None,
            sample.extent(),
        )
        .ok_or(crate::runtime::UiAllocationReplanTransactionCommitDenial::ResizeBasisDenied)?;
        let candidates = crate::runtime::planning::replan_selected_candidates_with_resize(
            selection, &basis,
        )
        .map_err(|ordinal| {
            crate::runtime::UiAllocationReplanTransactionCommitDenial::CandidatePlanningDenied {
                ordinal,
            }
        })?;
        let allocation_candidates = candidates
            .into_iter()
            .map(crate::runtime::allocation_receipt::project_allocation_preview)
            .collect::<Vec<_>>()
            .into_boxed_slice();
        let candidate = UiResizePreviewCandidate {
            frame_epoch: plan.frame_epoch(),
            target: sample.target(),
            extent: sample.extent(),
            allocation_candidates,
        };
        let counters = UiDragResizeCounters {
            admitted_samples: plan.resize_preview_sample_count(),
            preview_publications: 1,
            durable_mutations: 0,
            committed_receipts: 0,
        };
        Ok(Self {
            candidate,
            counters,
        })
    }
    pub(crate) fn preview_candidate(&self) -> &UiResizePreviewCandidate {
        &self.candidate
    }
    pub(crate) fn counters(&self) -> UiDragResizeCounters {
        self.counters
    }
}

impl UiResizePreviewCandidate {
    pub(crate) fn frame_epoch(&self) -> crate::runtime::UiAllocationFrameEpoch {
        self.frame_epoch
    }
    pub(crate) fn target(&self) -> crate::graph::UiGraphNodeIdentity {
        self.target
    }
    pub(crate) fn extent(&self) -> super::UiResizeLogicalExtent {
        self.extent
    }
    pub(crate) fn allocation_candidates(&self) -> &[crate::runtime::UiAllocationPreviewCandidate] {
        &self.allocation_candidates
    }
}

impl UiDragResizeCounters {
    pub fn admitted_samples(self) -> u16 {
        self.admitted_samples
    }
    pub fn preview_publications(self) -> u16 {
        self.preview_publications
    }
    pub fn durable_mutations(self) -> u16 {
        self.durable_mutations
    }
    pub fn committed_receipts(self) -> u16 {
        self.committed_receipts
    }
}

impl UiDurableResizeCommitOutcome {
    pub(crate) fn new(
        extent: super::UiResizeLogicalExtent,
        transaction: crate::runtime::UiCommittedAllocationReplan,
        durable_state: crate::runtime::UiAllocationDurableSemanticState,
        mutated: bool,
        replayed: bool,
    ) -> Self {
        let committed_receipts = if replayed {
            0
        } else {
            transaction.counters().committed_receipts()
        };
        let counters = UiDragResizeCounters {
            admitted_samples: 1,
            preview_publications: 0,
            durable_mutations: u16::from(mutated),
            committed_receipts,
        };
        let evidence = crate::evidence::UiDragResizeEvidence::new(
            crate::evidence::UiDragResizeStrategy::TerminalDurableCommit,
            transaction.transaction().frame_epoch(),
            counters,
        );
        Self {
            extent,
            transaction,
            durable_state,
            counters,
            evidence,
        }
    }
    pub fn extent(&self) -> super::UiResizeLogicalExtent {
        self.extent
    }
    pub fn committed_replan(&self) -> &crate::runtime::UiCommittedAllocationReplan {
        &self.transaction
    }
    pub fn durable_semantic_state(&self) -> &crate::runtime::UiAllocationDurableSemanticState {
        &self.durable_state
    }
    pub fn counters(&self) -> UiDragResizeCounters {
        self.counters
    }
    pub fn evidence(&self) -> &crate::evidence::UiDragResizeEvidence {
        &self.evidence
    }
}

impl UiDurableResizeCommitDenialReport {
    pub(crate) fn new(
        denial: crate::runtime::UiAllocationReplanTransactionCommitDenial,
        identity_digest: u64,
        extent: super::UiResizeLogicalExtent,
        frame_epoch: crate::runtime::UiAllocationFrameEpoch,
    ) -> Self {
        let counters = UiDragResizeCounters {
            admitted_samples: 1,
            preview_publications: 0,
            durable_mutations: 0,
            committed_receipts: 0,
        };
        Self {
            denial,
            identity_digest,
            extent,
            counters,
            evidence: crate::evidence::UiDragResizeEvidence::new(
                crate::evidence::UiDragResizeStrategy::TerminalDurableCommit,
                frame_epoch,
                counters,
            ),
        }
    }
    pub fn denial(&self) -> crate::runtime::UiAllocationReplanTransactionCommitDenial {
        self.denial
    }
    pub fn identity_digest(&self) -> u64 {
        self.identity_digest
    }
    pub fn extent(&self) -> super::UiResizeLogicalExtent {
        self.extent
    }
    pub fn counters(&self) -> UiDragResizeCounters {
        self.counters
    }
    pub fn evidence(&self) -> &crate::evidence::UiDragResizeEvidence {
        &self.evidence
    }
}

#[cfg(test)]
mod denial_report_tests {
    #[test]
    fn delayed_denial_is_self_describing_and_records_zero_commit_effects() {
        let extent = super::super::UiResizeLogicalExtent::try_from_logical_pixels(320.0).unwrap();
        let report = super::UiDurableResizeCommitDenialReport::new(
            crate::runtime::UiAllocationReplanTransactionCommitDenial::ResizeBasisDenied,
            41,
            extent,
            crate::runtime::UiAllocationFrameEpoch::for_test(7),
        );
        assert_eq!(report.identity_digest(), 41);
        assert_eq!(report.extent(), extent);
        assert_eq!(report.counters().durable_mutations(), 0);
        assert_eq!(report.counters().committed_receipts(), 0);
        assert_eq!(report.evidence().frame_epoch().as_u64(), 7);
    }
}
