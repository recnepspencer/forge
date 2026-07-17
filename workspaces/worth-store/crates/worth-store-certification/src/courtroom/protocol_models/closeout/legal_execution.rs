use std::path::{Path, PathBuf};

use worth_store_formal_models::assumptions::{
    admit_protocol_backend_capabilities, AdmittedProtocolBackendAssumptions,
    ProtocolBackendCapabilityDenial,
};
use worth_store_formal_models::runner::{
    execute_protocol_check_with_identity, ExecutedProtocolCheck,
    ProtocolArtifactIdentityInspectionDenial, ProtocolCheckBounds, ProtocolCheckInvocation,
    ProtocolCheckStatistics, ProtocolCheckVerdict, ProtocolCounterEvidenceIdentity,
    ProtocolRunnerFailure, TlcRunnerPaths,
};
use worth_store_formal_models::ProtocolFamily;
use worth_store_physical_backend::{AdmittedBackendCapabilityWitness, BackendDurabilityProfile};

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProtocolExecution {
    basis: CheckedProtocolExecutionBasis,
    statistics: ProtocolCheckStatistics,
    backend_assumptions: AdmittedProtocolBackendAssumptions,
}

#[derive(Debug, Clone, PartialEq, Eq)]
enum CheckedProtocolExecutionBasis {
    Runner(ExecutedProtocolCheck),
    #[cfg(test)]
    Structural(ProtocolCheckInvocation),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct CheckedProtocolExecutionReport {
    executions: Vec<CheckedProtocolExecution>,
}

#[derive(Debug)]
pub enum CheckedProtocolProgramFailure {
    Runner {
        protocol: ProtocolFamily,
        failure: ProtocolRunnerFailure,
    },
    ProtocolWasNotChecked {
        protocol: ProtocolFamily,
        verdict: ProtocolCheckVerdict,
    },
    BackendAssumptions {
        protocol: ProtocolFamily,
        denial: ProtocolBackendCapabilityDenial,
    },
}

pub fn run_checked_protocol_program<P: BackendDurabilityProfile>(
    java: impl AsRef<Path>,
    tool_jar: impl AsRef<Path>,
    state_root: impl AsRef<Path>,
    model_crate_root: impl AsRef<Path>,
    runtime_backend: &AdmittedBackendCapabilityWitness,
    bounds: ProtocolCheckBounds,
) -> Result<CheckedProtocolExecutionReport, CheckedProtocolProgramFailure> {
    let mut executions = Vec::with_capacity(ProtocolFamily::all().len());
    for protocol in ProtocolFamily::all() {
        let invocation = ProtocolCheckInvocation::for_checked_protocol(
            protocol,
            model_crate_root.as_ref(),
            bounds,
        );
        let backend_assumptions =
            admit_protocol_backend_capabilities::<P>(protocol, runtime_backend).map_err(
                |denial| CheckedProtocolProgramFailure::BackendAssumptions { protocol, denial },
            )?;
        let runner = TlcRunnerPaths::new(
            java.as_ref(),
            tool_jar.as_ref(),
            state_directory(state_root.as_ref(), protocol),
        );
        let check = execute_protocol_check_with_identity(&invocation, &runner)
            .map_err(|failure| CheckedProtocolProgramFailure::Runner { protocol, failure })?;
        executions.push(checked_execution(check, backend_assumptions)?);
    }
    Ok(CheckedProtocolExecutionReport { executions })
}

fn checked_execution(
    check: ExecutedProtocolCheck,
    backend_assumptions: AdmittedProtocolBackendAssumptions,
) -> Result<CheckedProtocolExecution, CheckedProtocolProgramFailure> {
    let protocol = check.protocol();
    let statistics = require_checked_statistics(protocol, check.verdict())?;
    Ok(CheckedProtocolExecution {
        statistics,
        basis: CheckedProtocolExecutionBasis::Runner(check),
        backend_assumptions,
    })
}

fn require_checked_statistics(
    protocol: ProtocolFamily,
    verdict: &ProtocolCheckVerdict,
) -> Result<ProtocolCheckStatistics, CheckedProtocolProgramFailure> {
    match verdict {
        ProtocolCheckVerdict::CheckedWithinBounds { statistics, .. } => Ok(*statistics),
        verdict => Err(CheckedProtocolProgramFailure::ProtocolWasNotChecked {
            protocol,
            verdict: verdict.clone(),
        }),
    }
}

impl CheckedProtocolExecution {
    pub const fn protocol(&self) -> ProtocolFamily {
        match &self.basis {
            CheckedProtocolExecutionBasis::Runner(check) => check.protocol(),
            #[cfg(test)]
            CheckedProtocolExecutionBasis::Structural(invocation) => invocation.protocol(),
        }
    }

    pub const fn invocation(&self) -> &ProtocolCheckInvocation {
        match &self.basis {
            CheckedProtocolExecutionBasis::Runner(check) => check.invocation(),
            #[cfg(test)]
            CheckedProtocolExecutionBasis::Structural(invocation) => invocation,
        }
    }

    pub const fn statistics(&self) -> ProtocolCheckStatistics {
        self.statistics
    }

    pub const fn backend_assumptions(&self) -> &AdmittedProtocolBackendAssumptions {
        &self.backend_assumptions
    }

    pub fn counter_identity(
        &self,
    ) -> Result<ProtocolCounterEvidenceIdentity, ProtocolArtifactIdentityInspectionDenial> {
        let backend_profile = self
            .backend_assumptions
            .profile()
            .durability()
            .runtime_profile();
        match &self.basis {
            CheckedProtocolExecutionBasis::Runner(check) => Ok(
                ProtocolCounterEvidenceIdentity::from_executed_check(check, backend_profile),
            ),
            #[cfg(test)]
            CheckedProtocolExecutionBasis::Structural(invocation) => {
                ProtocolCounterEvidenceIdentity::from_declared_fixture(invocation, backend_profile)
            }
        }
    }

    pub const fn executed_check(&self) -> &ExecutedProtocolCheck {
        match &self.basis {
            CheckedProtocolExecutionBasis::Runner(check) => check,
            #[cfg(test)]
            CheckedProtocolExecutionBasis::Structural(_) => {
                panic!("structural closeout fixtures do not carry runner evidence")
            }
        }
    }
}

impl CheckedProtocolExecutionReport {
    pub fn executions(&self) -> &[CheckedProtocolExecution] {
        &self.executions
    }

    pub(super) fn into_executions(self) -> Vec<CheckedProtocolExecution> {
        self.executions
    }
}

#[cfg(test)]
pub(super) fn structural_checked_protocol_fixture_for_closeout_tests<
    P: BackendDurabilityProfile,
>(
    model_crate_root: impl AsRef<Path>,
    bounds: ProtocolCheckBounds,
    runtime_backend: &AdmittedBackendCapabilityWitness,
) -> CheckedProtocolExecutionReport {
    let executions = ProtocolFamily::all()
        .into_iter()
        .map(|protocol| {
            let invocation = ProtocolCheckInvocation::for_checked_protocol(
                protocol,
                model_crate_root.as_ref(),
                bounds,
            );
            let statistics = ProtocolCheckStatistics::observed(1, 1, 1, 0, 1);
            CheckedProtocolExecution {
                basis: CheckedProtocolExecutionBasis::Structural(invocation),
                statistics,
                backend_assumptions: admit_protocol_backend_capabilities::<P>(
                    protocol,
                    runtime_backend,
                )
                .expect("structural closeout fixture needs admitted backend assumptions"),
            }
        })
        .collect();
    CheckedProtocolExecutionReport { executions }
}

fn state_directory(root: &Path, protocol: ProtocolFamily) -> PathBuf {
    root.join(format!("{protocol:?}").to_ascii_lowercase())
}

#[cfg(test)]
mod tests {
    use worth_store_formal_models::runner::ProtocolCounterexample;

    use super::*;

    #[test]
    fn a_counterexample_cannot_become_checked_execution_evidence() {
        let verdict = ProtocolCheckVerdict::CounterexampleFound {
            counterexample: ProtocolCounterexample::diagnostic(
                ProtocolFamily::DurabilityRecovery,
                vec!["illegal edge".to_owned()],
            ),
            statistics: ProtocolCheckStatistics::observed(1, 2, 2, 0, 2),
        };

        assert!(matches!(
            require_checked_statistics(ProtocolFamily::DurabilityRecovery, &verdict),
            Err(CheckedProtocolProgramFailure::ProtocolWasNotChecked { .. })
        ));
    }
}
