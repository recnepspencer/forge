use std::sync::Arc;

use crate::domain_computation::{
    WorthQueryCandidateSemanticFamilies, WorthQueryConvergenceAssessment,
    WorthQueryConvergenceComparison, WorthQueryConvergenceDisposition,
    WorthQueryConvergenceDomainFailure, WorthQueryConvergenceDomainProvider,
    WorthQueryConvergenceFeasibility, WorthQueryConvergenceIncumbentUpdate,
    WorthQueryConvergenceProgress, WorthQueryConvergenceProviderFamilies,
    WorthQueryConvergenceRepeatedState, WorthQueryIterationSemanticFamilies,
};

use super::disposition::{FixtureDisposition, FixtureFamilyMismatch as Family};

pub(in crate::domain_computation::convergence_epoch::tests::fixture) struct ConvergentProvider {
    families: WorthQueryConvergenceProviderFamilies,
    disposition: FixtureDisposition,
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
        }
    }

    pub(in crate::domain_computation::convergence_epoch::tests::fixture) const fn disposition(
        &self,
    ) -> FixtureDisposition {
        self.disposition
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
        let (candidate_occurrence_identity, incumbent_update) =
            incumbent_transition(self.disposition, iteration_ordinal);
        WorthQueryConvergenceComparison::new(
            candidate_occurrence_identity,
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
        FixtureDisposition::Converged
        | FixtureDisposition::ComparatorFailure
        | FixtureDisposition::ComparatorPanic
        | FixtureDisposition::ProgressFailure
        | FixtureDisposition::ProgressPanic
        | FixtureDisposition::RepeatedStateFailure
        | FixtureDisposition::RepeatedStatePanic
        | FixtureDisposition::FamilyInspectionPanic
        | FixtureDisposition::YieldThenConverged
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
    iteration_ordinal: usize,
) -> (String, WorthQueryConvergenceIncumbentUpdate) {
    match fixture {
        FixtureDisposition::ParetoReplacement if iteration_ordinal == 1 => (
            "candidate-1".into(),
            WorthQueryConvergenceIncumbentUpdate::AddCandidate,
        ),
        FixtureDisposition::ParetoReplacement => (
            format!("candidate-{iteration_ordinal}"),
            WorthQueryConvergenceIncumbentUpdate::RemoveCandidatesAndAdd {
                removed_occurrence_identities: vec![Arc::from("candidate-1")],
            },
        ),
        FixtureDisposition::ParetoCollision => (
            "candidate-pareto".into(),
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
