use std::collections::{BTreeMap, BTreeSet};

use worth_store_formal_models::assumptions::AdmittedProtocolBackendAssumptions;
use worth_store_formal_models::runner::{
    configured_invariant_count, project_checked_protocol_counters,
    project_counterexample_protocol_counters, require_checked_operator_bindings,
    CheckedOperatorBinding, ExactProtocolRefinementCoverageReceipt,
    ProtocolConformanceCounterInput, ProtocolCounterSnapshot,
};
use worth_store_formal_models::{protocol_model_contract, ProtocolFamily, ProtocolModelContract};
use worth_store_physical_backend::BackendDurabilityProfileId;

use super::ordinary_execution::{
    exact_modeled_action_coverage, execute_ordinary_protocol_suite,
    OrdinaryProtocolExecutionDenial, OrdinaryProtocolExecutionSuite,
};
use super::owner_coverage::require_exact_owner_coverage;
use super::CheckedProtocolExecution;
use super::CheckedProtocolExecutionReport;
use crate::courtroom::protocol_models::mutants::{
    ControlledMutantRejection, MutationProgramReport,
};

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ExactOwnerMappingEvidence {
    ExactOwnerCaseCoverage(ExactProtocolRefinementCoverageReceipt),
    SharedFrontierComposition(ExactProtocolRefinementCoverageReceipt),
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolCloseoutRow {
    protocol: ProtocolFamily,
    checked_execution: CheckedProtocolExecution,
    model_contract: ProtocolModelContract,
    owner_mapping: ExactOwnerMappingEvidence,
    ordinary_execution: OrdinaryProtocolExecutionSuite,
    checked_operator_bindings: Vec<CheckedOperatorBinding>,
    counters: ProtocolCloseoutCounters,
    controlled_defect: ControlledMutantRejection,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolCloseoutCounters {
    checked: ProtocolCounterSnapshot,
    controlled_defect: ProtocolCounterSnapshot,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum ProtocolResidualRisk {
    BoundedCheckingIsNotUnboundedProof,
    FairnessAndLivenessRemainUnclaimed,
    BackendClaimsRequireAdmittedRuntimeProfile,
    ReplicationProgressRequiresIntactDurableStore,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct ProtocolLawCloseoutReport {
    rows: Vec<ProtocolCloseoutRow>,
    residual_risks: BTreeSet<ProtocolResidualRisk>,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum ProtocolCloseoutDenial {
    MissingControlledDefect(ProtocolFamily),
    DuplicateControlledDefect(ProtocolFamily),
    ControlledDefectProtocolMismatch(ProtocolFamily),
    MissingCheckedExecution(ProtocolFamily),
    DuplicateCheckedExecution(ProtocolFamily),
    MissingOwnerMapping(ProtocolFamily),
    OrdinaryExecutionFailed {
        protocol: ProtocolFamily,
        denial: OrdinaryProtocolExecutionDenial,
    },
    IncompleteModeledActionCoverage(ProtocolFamily),
    CheckedOperatorBindingMismatch(ProtocolFamily),
    OwnerCoverageMismatch(ProtocolFamily),
    CounterEvidenceMismatch(ProtocolFamily),
    BackendProfileMismatch {
        checked: BackendDurabilityProfileId,
        executed: BackendDurabilityProfileId,
    },
}

pub fn adjudicate_protocol_law_closeout(
    checked_report: CheckedProtocolExecutionReport,
    mutation_report: MutationProgramReport,
) -> Result<ProtocolLawCloseoutReport, ProtocolCloseoutDenial> {
    let mut checked = checked_report.into_executions().into_iter().try_fold(
        BTreeMap::new(),
        |mut by_protocol, execution| {
            let protocol = execution.protocol();
            if by_protocol.insert(protocol, execution).is_some() {
                Err(ProtocolCloseoutDenial::DuplicateCheckedExecution(protocol))
            } else {
                Ok(by_protocol)
            }
        },
    )?;
    let mut defects = mutation_report.into_rejections().into_iter().try_fold(
        BTreeMap::new(),
        |mut by_protocol, rejection| {
            let protocol = rejection.mutant().protocol();
            if by_protocol.insert(protocol, rejection).is_some() {
                Err(ProtocolCloseoutDenial::DuplicateControlledDefect(protocol))
            } else {
                Ok(by_protocol)
            }
        },
    )?;
    let mut rows = Vec::with_capacity(ProtocolFamily::all().len());
    for protocol in ProtocolFamily::all() {
        let controlled_defect = defects
            .remove(&protocol)
            .ok_or(ProtocolCloseoutDenial::MissingControlledDefect(protocol))?;
        if controlled_defect.localization().failing_lane().as_str()
            != controlled_defect.mutant().certification_lane()
        {
            return Err(ProtocolCloseoutDenial::ControlledDefectProtocolMismatch(
                protocol,
            ));
        }
        let ordinary_execution = execute_ordinary_protocol_suite(protocol).map_err(|denial| {
            ProtocolCloseoutDenial::OrdinaryExecutionFailed { protocol, denial }
        })?;
        if !exact_modeled_action_coverage(&ordinary_execution) {
            return Err(ProtocolCloseoutDenial::IncompleteModeledActionCoverage(
                protocol,
            ));
        }
        let checked_execution = checked
            .remove(&protocol)
            .ok_or(ProtocolCloseoutDenial::MissingCheckedExecution(protocol))?;
        let checked_operator_bindings = require_checked_operator_bindings(
            checked_execution.invocation(),
            ordinary_execution.coverage_actions(),
        )
        .map_err(|_| ProtocolCloseoutDenial::CheckedOperatorBindingMismatch(protocol))?;
        let owner_mapping =
            require_exact_owner_coverage(&ordinary_execution, &checked_operator_bindings)
                .map(|receipt| {
                    if protocol == ProtocolFamily::SharedFrontiers {
                        ExactOwnerMappingEvidence::SharedFrontierComposition(receipt)
                    } else {
                        ExactOwnerMappingEvidence::ExactOwnerCaseCoverage(receipt)
                    }
                })
                .map_err(|_| ProtocolCloseoutDenial::OwnerCoverageMismatch(protocol))?;
        if protocol == ProtocolFamily::DurabilityRecovery {
            let checked_profile = checked_execution
                .backend_assumptions()
                .profile()
                .durability()
                .runtime_profile();
            let executed_profile = crate::courtroom::protocol_models::durability_recovery::scenario::ordinary_durability_profile();
            if checked_profile != executed_profile {
                return Err(ProtocolCloseoutDenial::BackendProfileMismatch {
                    checked: checked_profile,
                    executed: executed_profile,
                });
            }
        }
        if let Some(executed_profile) = controlled_defect.physical_replay().backend_profile() {
            let checked_profile = checked_execution
                .backend_assumptions()
                .profile()
                .durability()
                .runtime_profile();
            if checked_profile != executed_profile {
                return Err(ProtocolCloseoutDenial::BackendProfileMismatch {
                    checked: checked_profile,
                    executed: executed_profile,
                });
            }
        }
        let coverage = match owner_mapping {
            ExactOwnerMappingEvidence::ExactOwnerCaseCoverage(receipt)
            | ExactOwnerMappingEvidence::SharedFrontierComposition(receipt) => receipt,
        };
        let checked_identity = checked_execution
            .counter_identity()
            .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?;
        let checked_counters = project_checked_protocol_counters(
            checked_identity,
            checked_execution.statistics(),
            ProtocolConformanceCounterInput::from_exact_coverage(
                coverage,
                coverage.ordinary_executed_cases(),
                coverage.ordinary_executed_cases(),
            ),
            configured_invariant_count(checked_execution.invocation())
                .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?,
        )
        .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?;
        let mutant_invocation = controlled_defect
            .mutant()
            .invocation(checked_execution.invocation().bounds());
        let mutant_identity = controlled_defect
            .counter_identity(
                checked_execution.invocation().bounds(),
                checked_execution
                    .backend_assumptions()
                    .profile()
                    .durability()
                    .runtime_profile(),
            )
            .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?;
        let controlled_defect_counters = project_counterexample_protocol_counters(
            mutant_identity,
            controlled_defect.check_statistics(),
            configured_invariant_count(&mutant_invocation)
                .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?,
            controlled_defect.localization().counterexample(),
        )
        .map_err(|_| ProtocolCloseoutDenial::CounterEvidenceMismatch(protocol))?;
        let counters = ProtocolCloseoutCounters {
            checked: checked_counters,
            controlled_defect: controlled_defect_counters,
        };
        rows.push(ProtocolCloseoutRow {
            protocol,
            checked_execution,
            model_contract: protocol_model_contract(protocol),
            owner_mapping,
            ordinary_execution,
            checked_operator_bindings,
            counters,
            controlled_defect,
        });
    }
    Ok(ProtocolLawCloseoutReport {
        rows,
        residual_risks: BTreeSet::from([
            ProtocolResidualRisk::BoundedCheckingIsNotUnboundedProof,
            ProtocolResidualRisk::FairnessAndLivenessRemainUnclaimed,
            ProtocolResidualRisk::BackendClaimsRequireAdmittedRuntimeProfile,
            ProtocolResidualRisk::ReplicationProgressRequiresIntactDurableStore,
        ]),
    })
}

impl ProtocolCloseoutRow {
    pub const fn protocol(&self) -> ProtocolFamily {
        self.protocol
    }

    pub const fn checked_execution(&self) -> &CheckedProtocolExecution {
        &self.checked_execution
    }

    pub const fn model_contract(&self) -> ProtocolModelContract {
        self.model_contract
    }

    pub const fn owner_mapping(&self) -> ExactOwnerMappingEvidence {
        self.owner_mapping
    }

    pub const fn ordinary_execution(&self) -> &OrdinaryProtocolExecutionSuite {
        &self.ordinary_execution
    }

    pub fn checked_operator_bindings(&self) -> &[CheckedOperatorBinding] {
        &self.checked_operator_bindings
    }

    pub const fn counters(&self) -> &ProtocolCloseoutCounters {
        &self.counters
    }

    pub const fn backend_assumptions(&self) -> &AdmittedProtocolBackendAssumptions {
        self.checked_execution.backend_assumptions()
    }

    pub const fn controlled_defect(&self) -> &ControlledMutantRejection {
        &self.controlled_defect
    }
}

impl ProtocolCloseoutCounters {
    pub const fn checked(&self) -> &ProtocolCounterSnapshot {
        &self.checked
    }

    pub const fn controlled_defect(&self) -> &ProtocolCounterSnapshot {
        &self.controlled_defect
    }
}

impl ProtocolLawCloseoutReport {
    pub fn rows(&self) -> &[ProtocolCloseoutRow] {
        &self.rows
    }

    pub const fn residual_risks(&self) -> &BTreeSet<ProtocolResidualRisk> {
        &self.residual_risks
    }
}
