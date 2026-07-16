use std::path::{Path, PathBuf};

use worth_store_formal_models::runner::{
    execute_protocol_check_with_identity, ExecutedProtocolCheck,
    ProtocolArtifactIdentityInspectionDenial, ProtocolCheckBounds, ProtocolCheckStatistics,
    ProtocolCheckVerdict, ProtocolCounterEvidenceIdentity, ProtocolRunnerFailure, TlcRunnerPaths,
};

use super::localization::localize_controlled_mutant;
use super::physical_replay::replay_controlled_counterexample;
use super::{
    ControlledMutantLocalization, ControlledMutantLocalizationDenial,
    CounterexamplePhysicalReplayDenial, CounterexamplePhysicalReplayEvidence,
};
use crate::courtroom::protocol_models::mutants::ControlledProtocolMutant;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ControlledMutantRejection {
    mutant: ControlledProtocolMutant,
    localization: ControlledMutantLocalization,
    physical_replay: CounterexamplePhysicalReplayEvidence,
    check_basis: ControlledMutantCheckBasis,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum ControlledMutantCheckBasis {
    Runner(Box<ExecutedProtocolCheck>),
    #[cfg(test)]
    Structural,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct MutationProgramReport {
    rejections: Vec<ControlledMutantRejection>,
}

#[derive(Debug)]
pub enum MutationProgramFailure {
    Runner {
        mutant: ControlledProtocolMutant,
        failure: ProtocolRunnerFailure,
    },
    MutantSurvived {
        mutant: ControlledProtocolMutant,
        verdict: ProtocolCheckVerdict,
    },
    Localization {
        mutant: ControlledProtocolMutant,
        denial: ControlledMutantLocalizationDenial,
    },
    PhysicalReplay {
        mutant: ControlledProtocolMutant,
        denial: CounterexamplePhysicalReplayDenial,
    },
}

pub fn run_controlled_mutant_program(
    java: impl AsRef<Path>,
    tool_jar: impl AsRef<Path>,
    state_root: impl AsRef<Path>,
    bounds: ProtocolCheckBounds,
) -> Result<MutationProgramReport, MutationProgramFailure> {
    let mut rejections = Vec::with_capacity(ControlledProtocolMutant::all().len());
    for mutant in ControlledProtocolMutant::all() {
        let invocation = mutant.invocation(bounds);
        let runner = TlcRunnerPaths::new(
            java.as_ref(),
            tool_jar.as_ref(),
            state_directory(state_root.as_ref(), mutant),
        );
        let check = execute_protocol_check_with_identity(&invocation, &runner)
            .map_err(|failure| MutationProgramFailure::Runner { mutant, failure })?;
        let verdict = check.verdict().clone();
        let ProtocolCheckVerdict::CounterexampleFound { counterexample, .. } = verdict else {
            return Err(MutationProgramFailure::MutantSurvived { mutant, verdict });
        };
        let localization = localize_controlled_mutant(mutant, counterexample)
            .map_err(|denial| MutationProgramFailure::Localization { mutant, denial })?;
        let physical_replay = replay_controlled_counterexample(mutant, &localization)
            .map_err(|denial| MutationProgramFailure::PhysicalReplay { mutant, denial })?;
        rejections.push(ControlledMutantRejection {
            mutant,
            localization,
            physical_replay,
            check_basis: ControlledMutantCheckBasis::Runner(Box::new(check)),
        });
    }
    Ok(MutationProgramReport { rejections })
}

impl ControlledMutantRejection {
    pub const fn mutant(&self) -> ControlledProtocolMutant {
        self.mutant
    }

    pub const fn localization(&self) -> &ControlledMutantLocalization {
        &self.localization
    }

    pub const fn physical_replay(&self) -> &CounterexamplePhysicalReplayEvidence {
        &self.physical_replay
    }

    pub const fn executed_check(&self) -> &ExecutedProtocolCheck {
        match &self.check_basis {
            ControlledMutantCheckBasis::Runner(check) => check,
            #[cfg(test)]
            ControlledMutantCheckBasis::Structural => {
                panic!("structural mutation fixtures do not carry runner evidence")
            }
        }
    }

    pub fn counter_identity(
        &self,
        _bounds: ProtocolCheckBounds,
        backend_profile: worth_store_physical_backend::BackendDurabilityProfileId,
    ) -> Result<ProtocolCounterEvidenceIdentity, ProtocolArtifactIdentityInspectionDenial> {
        match &self.check_basis {
            ControlledMutantCheckBasis::Runner(check) => Ok(
                ProtocolCounterEvidenceIdentity::from_executed_check(check, backend_profile),
            ),
            #[cfg(test)]
            ControlledMutantCheckBasis::Structural => {
                ProtocolCounterEvidenceIdentity::from_declared_fixture(
                    &self.mutant.invocation(_bounds),
                    backend_profile,
                )
            }
        }
    }

    pub const fn check_statistics(&self) -> ProtocolCheckStatistics {
        match &self.check_basis {
            ControlledMutantCheckBasis::Runner(check) => match check.verdict() {
                ProtocolCheckVerdict::CounterexampleFound { statistics, .. } => *statistics,
                _ => panic!("controlled mutant rejection must retain counterexample statistics"),
            },
            #[cfg(test)]
            ControlledMutantCheckBasis::Structural => {
                ProtocolCheckStatistics::observed(1, 2, 2, 0, 2)
            }
        }
    }
}

impl MutationProgramReport {
    pub fn rejections(&self) -> &[ControlledMutantRejection] {
        &self.rejections
    }

    pub(in crate::courtroom::protocol_models) fn into_rejections(
        self,
    ) -> Vec<ControlledMutantRejection> {
        self.rejections
    }
}

#[cfg(test)]
pub(in crate::courtroom::protocol_models) fn structural_mutation_fixture_for_closeout_tests(
) -> MutationProgramReport {
    use super::localization::expected_checker_edge;
    use worth_store_formal_models::runner::{ProtocolCounterexample, ProtocolCounterexampleState};

    let rejections = ControlledProtocolMutant::all()
        .into_iter()
        .map(|mutant| {
            let localization = localize_controlled_mutant(
                mutant,
                ProtocolCounterexample::from_tlc_states(
                    mutant.protocol(),
                    vec![ProtocolCounterexampleState::observed(
                        1,
                        expected_checker_edge(mutant),
                        vec![(
                            "mutantEdge".to_owned(),
                            format!("\"{}\"", expected_checker_edge(mutant)),
                        )],
                    )],
                ),
            )
            .expect("catalogued controlled mutant has an exact localization");
            let physical_replay = replay_controlled_counterexample(mutant, &localization)
                .expect("catalogued controlled mutant has an executable physical replay");
            ControlledMutantRejection {
                mutant,
                check_basis: ControlledMutantCheckBasis::Structural,
                localization,
                physical_replay,
            }
        })
        .collect();
    MutationProgramReport { rejections }
}

fn state_directory(root: &Path, mutant: ControlledProtocolMutant) -> PathBuf {
    root.join(mutant.certification_lane().replace('.', "-"))
}
