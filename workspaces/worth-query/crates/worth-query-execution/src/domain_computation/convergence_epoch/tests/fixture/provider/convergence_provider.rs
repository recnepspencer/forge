use std::sync::{mpsc::Sender, Arc};

use crate::domain_computation::artifact_owner::WorthQueryMoveOnlyArtifactHandle;
use crate::domain_computation::{
    WorthQueryCandidateSemanticFamilies, WorthQueryConvergenceAssessment,
    WorthQueryConvergenceComparison, WorthQueryConvergenceDisposition,
    WorthQueryConvergenceDomainFailure, WorthQueryConvergenceDomainProvider,
    WorthQueryConvergenceFeasibility, WorthQueryConvergenceIncumbentUpdate,
    WorthQueryConvergenceProgress, WorthQueryConvergenceProviderFamilies,
    WorthQueryConvergenceRepeatedState, WorthQueryIterationSemanticFamilies,
};

use super::disposition::{FixtureDisposition, FixtureFamilyMismatch as Family};
use super::domain_port_probe::FixtureDomainPortProbe;
use super::report_history_probe::FixtureReportHistoryProbe;
use super::yield_recovery::{FixtureYieldRecoveryArtifact, FixtureYieldRecoveryProbe};

#[derive(Clone)]
pub(super) struct FixtureCleanupArtifact {
    pub(super) sender: Sender<WorthQueryMoveOnlyArtifactHandle>,
    pub(super) behavior: FixtureYieldRecoveryArtifact,
    pub(super) probe: FixtureYieldRecoveryProbe,
}

pub(in crate::domain_computation::convergence_epoch::tests::fixture) struct ConvergentProvider {
    families: WorthQueryConvergenceProviderFamilies,
    disposition: FixtureDisposition,
    cleanup_artifact: Option<FixtureCleanupArtifact>,
    yield_recovery_probe: Option<FixtureYieldRecoveryProbe>,
    domain_port_probe: Option<FixtureDomainPortProbe>,
    report_history_probe: Option<FixtureReportHistoryProbe>,
}

impl ConvergentProvider {
    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn new(
        disposition: FixtureDisposition,
    ) -> Self {
        let candidate = WorthQueryCandidateSemanticFamilies::new(
            family(disposition, Family::Universe, "universe"),
            family(disposition, Family::Termination, "termination"),
            family(disposition, Family::Feasibility, "feasibility"),
            family(disposition, Family::Comparison, "comparison"),
            family(disposition, Family::Incumbent, "incumbent"),
        )
        .expect("fixture candidate families must be portable");
        let iteration = WorthQueryIterationSemanticFamilies::new(
            family(disposition, Family::Progress, "progress"),
            family(disposition, Family::Comparator, "comparator"),
            family(disposition, Family::RepeatedState, "repeated-state"),
        )
        .expect("fixture iteration families must be portable");
        Self {
            families: WorthQueryConvergenceProviderFamilies::new(candidate, iteration),
            disposition,
            cleanup_artifact: None,
            yield_recovery_probe: None,
            domain_port_probe: None,
            report_history_probe: None,
        }
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn with_domain_port_probe(
        mut self,
        probe: FixtureDomainPortProbe,
    ) -> Self {
        self.domain_port_probe = Some(probe);
        self
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn with_report_history_probe(
        mut self,
        probe: FixtureReportHistoryProbe,
    ) -> Self {
        self.report_history_probe = Some(probe);
        self
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn with_cleanup_artifact_handle_sender(
        mut self,
        sender: Sender<WorthQueryMoveOnlyArtifactHandle>,
    ) -> Self {
        self.cleanup_artifact = Some(FixtureCleanupArtifact {
            sender,
            behavior: FixtureYieldRecoveryArtifact::Cooperative,
            probe: FixtureYieldRecoveryProbe::default(),
        });
        self
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn with_yield_recovery_probe(
        mut self,
        probe: FixtureYieldRecoveryProbe,
    ) -> Self {
        self.yield_recovery_probe = Some(probe);
        self
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) fn with_yield_recovery_artifact(
        mut self,
        sender: Sender<WorthQueryMoveOnlyArtifactHandle>,
        behavior: FixtureYieldRecoveryArtifact,
        probe: FixtureYieldRecoveryProbe,
    ) -> Self {
        self.cleanup_artifact = Some(FixtureCleanupArtifact {
            sender,
            behavior,
            probe: probe.clone(),
        });
        self.yield_recovery_probe = Some(probe);
        self
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) const fn disposition(
        &self,
    ) -> FixtureDisposition {
        self.disposition
    }

    pub(super) fn cleanup_artifact(&self) -> Option<FixtureCleanupArtifact> {
        self.cleanup_artifact.clone()
    }

    pub(super) fn yield_recovery_probe(&self) -> Option<FixtureYieldRecoveryProbe> {
        self.yield_recovery_probe.clone()
    }
}

impl WorthQueryConvergenceDomainProvider for ConvergentProvider {
    fn convergence_families(&self) -> &WorthQueryConvergenceProviderFamilies {
        assert!(
            !matches!(self.disposition, FixtureDisposition::FamilyInspectionPanic),
            "fixture convergence family inspection panic"
        );
        &self.families
    }

    fn compare(
        &self,
        assessment: &WorthQueryConvergenceAssessment<'_>,
    ) -> Result<WorthQueryConvergenceComparison, WorthQueryConvergenceDomainFailure> {
        if let Some(probe) = &self.domain_port_probe {
            probe.entered_comparator();
        }
        if let Some(probe) = &self.report_history_probe {
            probe.observe(assessment);
        }
        assert!(
            !matches!(self.disposition, FixtureDisposition::ComparatorPanic),
            "fixture convergence comparator panic"
        );
        if matches!(self.disposition, FixtureDisposition::ComparatorFailure) {
            return Err(WorthQueryConvergenceDomainFailure::new(
                "installed comparator could not classify the candidate",
            ));
        }
        let iteration_ordinal = assessment.iteration_ordinal();
        let (disposition, feasibility, _, _) = decision_axes(self.disposition, iteration_ordinal);
        let (candidate_selection_key, incumbent_update) =
            incumbent_transition(self.disposition, assessment);
        WorthQueryConvergenceComparison::new(
            candidate_selection_key,
            format!("state-{iteration_ordinal}"),
            disposition,
            feasibility,
            incumbent_update,
        )
        .map_err(WorthQueryConvergenceDomainFailure::new)
    }

    fn measure_progress(
        &self,
        assessment: &WorthQueryConvergenceAssessment<'_>,
        _comparison: &WorthQueryConvergenceComparison,
    ) -> Result<WorthQueryConvergenceProgress, WorthQueryConvergenceDomainFailure> {
        if let Some(probe) = &self.domain_port_probe {
            probe.entered_progress();
        }
        assert!(
            !matches!(self.disposition, FixtureDisposition::ProgressPanic),
            "fixture convergence progress panic"
        );
        if matches!(self.disposition, FixtureDisposition::ProgressFailure) {
            return Err(WorthQueryConvergenceDomainFailure::new(
                "installed progress measure could not classify the candidate",
            ));
        }
        let (_, _, progress, _) = decision_axes(self.disposition, assessment.iteration_ordinal());
        Ok(progress)
    }

    fn detect_repeated_state(
        &self,
        assessment: &WorthQueryConvergenceAssessment<'_>,
        _comparison: &WorthQueryConvergenceComparison,
        _progress: WorthQueryConvergenceProgress,
    ) -> Result<WorthQueryConvergenceRepeatedState, WorthQueryConvergenceDomainFailure> {
        if let Some(probe) = &self.domain_port_probe {
            probe.entered_repeated_state();
        }
        assert!(
            !matches!(self.disposition, FixtureDisposition::RepeatedStatePanic),
            "fixture convergence repeated-state panic"
        );
        if matches!(self.disposition, FixtureDisposition::RepeatedStateFailure) {
            return Err(WorthQueryConvergenceDomainFailure::new(
                "installed repeated-state detector could not classify the candidate",
            ));
        }
        let (_, _, _, repeated_state) =
            decision_axes(self.disposition, assessment.iteration_ordinal());
        Ok(repeated_state)
    }
}

fn family(disposition: FixtureDisposition, mismatch: Family, name: &'static str) -> String {
    let namespace = if disposition.mismatches(mismatch) {
        "foreign"
    } else {
        "convergence"
    };
    format!("worth.{namespace}.{name}")
}

fn decision_axes(
    fixture: FixtureDisposition,
    iteration_ordinal: usize,
) -> (
    WorthQueryConvergenceDisposition,
    WorthQueryConvergenceFeasibility,
    WorthQueryConvergenceProgress,
    WorthQueryConvergenceRepeatedState,
) {
    let disposition = match fixture {
        FixtureDisposition::Continue | FixtureDisposition::RepeatedContinue => {
            WorthQueryConvergenceDisposition::Continue
        }
        FixtureDisposition::HistoryRetain
        | FixtureDisposition::HistoryClear
        | FixtureDisposition::HistoryInvalidTransition
        | FixtureDisposition::HistoryInvalidDomain
            if iteration_ordinal == 1 =>
        {
            WorthQueryConvergenceDisposition::Continue
        }
        FixtureDisposition::HistoryInvalidDomain => {
            WorthQueryConvergenceDisposition::StableWithoutProof
        }
        FixtureDisposition::HistoryRetain
        | FixtureDisposition::HistoryClear
        | FixtureDisposition::HistoryInvalidTransition => {
            WorthQueryConvergenceDisposition::Converged
        }
        FixtureDisposition::Converged
        | FixtureDisposition::ComparatorFailure
        | FixtureDisposition::ComparatorPanic
        | FixtureDisposition::ProgressFailure
        | FixtureDisposition::ProgressPanic
        | FixtureDisposition::RepeatedStateFailure
        | FixtureDisposition::RepeatedStatePanic
        | FixtureDisposition::FamilyInspectionPanic
        | FixtureDisposition::YieldThenCheckpointUnavailable
        | FixtureDisposition::YieldThenConverged
        | FixtureDisposition::YieldThenRestorePanic
        | FixtureDisposition::YieldThenCheckpointDropPanic
        | FixtureDisposition::YieldThenSuspensionFailure
        | FixtureDisposition::ChunkedConverged(_)
        | FixtureDisposition::StageQueueContractMismatch
        | FixtureDisposition::FamilyMismatch(_) => WorthQueryConvergenceDisposition::Converged,
        FixtureDisposition::ParetoReplacement | FixtureDisposition::ParetoCollision
            if iteration_ordinal == 1 =>
        {
            WorthQueryConvergenceDisposition::Continue
        }
        FixtureDisposition::ParetoReplacement | FixtureDisposition::ParetoCollision => {
            WorthQueryConvergenceDisposition::Converged
        }
        FixtureDisposition::ParetoPartialReplacement if iteration_ordinal < 3 => {
            WorthQueryConvergenceDisposition::Continue
        }
        FixtureDisposition::ParetoPartialReplacement => WorthQueryConvergenceDisposition::Converged,
        FixtureDisposition::StableWithoutProof | FixtureDisposition::IncoherentStable => {
            WorthQueryConvergenceDisposition::StableWithoutProof
        }
        FixtureDisposition::FeasibleIncumbent => {
            WorthQueryConvergenceDisposition::FeasibleIncumbent
        }
        FixtureDisposition::Oscillating
        | FixtureDisposition::OscillatingSelected
        | FixtureDisposition::DomainClassifiedOscillation => {
            WorthQueryConvergenceDisposition::Oscillating
        }
        FixtureDisposition::Stalled | FixtureDisposition::IndeterminateComparison => {
            WorthQueryConvergenceDisposition::Indeterminate
        }
    };
    let progress = match fixture {
        FixtureDisposition::StableWithoutProof => WorthQueryConvergenceProgress::Stable,
        FixtureDisposition::Stalled => WorthQueryConvergenceProgress::Stalled,
        FixtureDisposition::IndeterminateComparison => WorthQueryConvergenceProgress::Indeterminate,
        _ => WorthQueryConvergenceProgress::Advanced,
    };
    let repeated_state = match fixture {
        FixtureDisposition::Oscillating
        | FixtureDisposition::OscillatingSelected
        | FixtureDisposition::RepeatedContinue => WorthQueryConvergenceRepeatedState::Repeated,
        FixtureDisposition::IndeterminateComparison => {
            WorthQueryConvergenceRepeatedState::Indeterminate
        }
        _ => WorthQueryConvergenceRepeatedState::Novel,
    };
    let feasibility = match fixture {
        FixtureDisposition::IndeterminateComparison => {
            WorthQueryConvergenceFeasibility::Indeterminate
        }
        _ => WorthQueryConvergenceFeasibility::Feasible,
    };
    (disposition, feasibility, progress, repeated_state)
}

fn incumbent_transition(
    fixture: FixtureDisposition,
    assessment: &WorthQueryConvergenceAssessment<'_>,
) -> (String, WorthQueryConvergenceIncumbentUpdate) {
    let iteration_ordinal = assessment.iteration_ordinal();
    match fixture {
        FixtureDisposition::ParetoReplacement if iteration_ordinal == 1 => (
            "candidate-1".into(),
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ),
        FixtureDisposition::ParetoReplacement => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
                removed_occurrence_identities: vec![Arc::from(
                    assessment.incumbents()[0].occurrence_identity(),
                )],
            },
        ),
        FixtureDisposition::ParetoCollision => (
            "candidate-pareto".into(),
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ),
        FixtureDisposition::ParetoPartialReplacement if iteration_ordinal < 3 => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ),
        FixtureDisposition::ParetoPartialReplacement => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
                removed_occurrence_identities: vec![Arc::from(
                    assessment.incumbents()[0].occurrence_identity(),
                )],
            },
        ),
        FixtureDisposition::HistoryRetain if iteration_ordinal > 1 => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::Retain,
        ),
        FixtureDisposition::HistoryClear if iteration_ordinal > 1 => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::Clear,
        ),
        FixtureDisposition::HistoryInvalidTransition if iteration_ordinal > 1 => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ),
        FixtureDisposition::Oscillating => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::Retain,
        ),
        _ => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::ReplaceWithCandidate,
        ),
    }
}
