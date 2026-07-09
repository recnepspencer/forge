use serde::Serialize;

use crate::boundary::errors::WORTHSignalJsError;

use super::{
    canonical_worker_certification_digest,
    worker_phase7_performance_catalog::{
        required_bridge_allocation_posture, required_complexity_contracts, required_counter_names,
        required_failure_modes,
    },
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7PerformanceContractPackage {
    pub certification_family: &'static str,
    pub covered_counter_count: u64,
    pub covered_complexity_contract_count: u64,
    pub prohibited_failure_mode_count: u64,
    pub counter_names: Vec<&'static str>,
    pub complexity_contracts: Vec<WorkerPhase7ComplexityContract>,
    pub prohibited_failure_modes: Vec<WorkerPhase7PerformanceFailureMode>,
    pub bridge_allocation_posture: WorkerPhase7BridgeAllocationPosture,
    pub counter_catalog_digest: String,
    pub complexity_contract_digest: String,
    pub failure_mode_digest: String,
    pub bridge_allocation_posture_digest: String,
    pub certification_digest: String,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7ComplexityContract {
    pub operation: &'static str,
    pub contract: &'static str,
    pub cost_bases: Vec<&'static str>,
    pub forbidden_cost_bases: Vec<&'static str>,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7PerformanceFailureMode {
    pub mode: &'static str,
    pub prohibited_behavior: &'static str,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize)]
#[serde(rename_all = "camelCase")]
pub struct WorkerPhase7BridgeAllocationPosture {
    pub posture: &'static str,
    pub serialization_allocation_counter: &'static str,
    pub deserialization_allocation_counter: &'static str,
    pub lifecycle_scope: &'static str,
    pub hidden_allocation_allowed: bool,
}

pub fn certify_worker_phase7_performance_contracts(
) -> Result<WorkerPhase7PerformanceContractPackage, WORTHSignalJsError> {
    WorkerPhase7PerformanceContractPackage::from_catalog(
        required_counter_names(),
        required_complexity_contracts(),
        required_failure_modes(),
        required_bridge_allocation_posture(),
    )
}

impl WorkerPhase7PerformanceContractPackage {
    pub(crate) fn from_catalog(
        counter_names: Vec<&'static str>,
        complexity_contracts: Vec<WorkerPhase7ComplexityContract>,
        prohibited_failure_modes: Vec<WorkerPhase7PerformanceFailureMode>,
        bridge_allocation_posture: WorkerPhase7BridgeAllocationPosture,
    ) -> Result<Self, WORTHSignalJsError> {
        reject_missing_counter_names(counter_names.as_slice())?;
        reject_duplicate_counter_names(counter_names.as_slice())?;
        reject_missing_complexity_contracts(complexity_contracts.as_slice())?;
        reject_duplicate_complexity_contracts(complexity_contracts.as_slice())?;
        reject_incomplete_complexity_contract_cost_bases(complexity_contracts.as_slice())?;
        reject_vague_complexity_contracts(complexity_contracts.as_slice())?;
        reject_missing_failure_modes(prohibited_failure_modes.as_slice())?;
        reject_duplicate_failure_modes(prohibited_failure_modes.as_slice())?;
        reject_weak_bridge_allocation_posture(&bridge_allocation_posture)?;

        let counter_catalog_digest =
            canonical_worker_certification_digest(&("workerPhase7Counters", &counter_names))?;
        let complexity_contract_digest = canonical_worker_certification_digest(&(
            "workerPhase7ComplexityContracts",
            &complexity_contracts,
        ))?;
        let failure_mode_digest = canonical_worker_certification_digest(&(
            "workerPhase7PerformanceFailureModes",
            &prohibited_failure_modes,
        ))?;
        let bridge_allocation_posture_digest = canonical_worker_certification_digest(&(
            "workerPhase7BridgeAllocationPosture",
            &bridge_allocation_posture,
        ))?;
        let certification_digest = canonical_worker_certification_digest(&(
            "workerPhase7PerformanceContractCertification",
            counter_catalog_digest.as_str(),
            complexity_contract_digest.as_str(),
            failure_mode_digest.as_str(),
            bridge_allocation_posture_digest.as_str(),
        ))?;

        Ok(Self {
            certification_family: "workerPhase7PerformanceContractCertification",
            covered_counter_count: counter_names.len() as u64,
            covered_complexity_contract_count: complexity_contracts.len() as u64,
            prohibited_failure_mode_count: prohibited_failure_modes.len() as u64,
            counter_names,
            complexity_contracts,
            prohibited_failure_modes,
            bridge_allocation_posture,
            counter_catalog_digest,
            complexity_contract_digest,
            failure_mode_digest,
            bridge_allocation_posture_digest,
            certification_digest,
        })
    }
}

fn reject_missing_counter_names(counter_names: &[&str]) -> Result<(), WORTHSignalJsError> {
    for required in required_counter_names() {
        if !counter_names.contains(&required) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts require counter {required}",
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_counter_names(counter_names: &[&str]) -> Result<(), WORTHSignalJsError> {
    for (index, counter_name) in counter_names.iter().enumerate() {
        if counter_names[(index + 1)..].contains(counter_name) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts duplicate counter {counter_name}",
            )));
        }
    }
    Ok(())
}

fn reject_missing_complexity_contracts(
    contracts: &[WorkerPhase7ComplexityContract],
) -> Result<(), WORTHSignalJsError> {
    for required in required_complexity_contracts() {
        if !contracts
            .iter()
            .any(|contract| contract.operation == required.operation)
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts require complexity contract {}",
                required.operation,
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_complexity_contracts(
    contracts: &[WorkerPhase7ComplexityContract],
) -> Result<(), WORTHSignalJsError> {
    for (index, contract) in contracts.iter().enumerate() {
        if contracts[(index + 1)..]
            .iter()
            .any(|candidate| candidate.operation == contract.operation)
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts duplicate complexity contract {}",
                contract.operation,
            )));
        }
    }
    Ok(())
}

fn reject_incomplete_complexity_contract_cost_bases(
    contracts: &[WorkerPhase7ComplexityContract],
) -> Result<(), WORTHSignalJsError> {
    for required in required_complexity_contracts() {
        let contract = contracts
            .iter()
            .find(|contract| contract.operation == required.operation)
            .ok_or_else(|| {
                WORTHSignalJsError::invalid_input(format!(
                    "worker Phase 7 performance contracts require complexity contract {}",
                    required.operation,
                ))
            })?;
        for cost_basis in required.cost_bases {
            if !contract.cost_bases.contains(&cost_basis) {
                return Err(WORTHSignalJsError::invalid_input(format!(
                    "worker Phase 7 performance contract {} requires cost base {cost_basis}",
                    contract.operation,
                )));
            }
        }
    }
    Ok(())
}

fn reject_vague_complexity_contracts(
    contracts: &[WorkerPhase7ComplexityContract],
) -> Result<(), WORTHSignalJsError> {
    for contract in contracts {
        if contract.cost_bases.is_empty()
            || contract.cost_bases.contains(&"totalGraphSize")
            || contract.cost_bases.contains(&"ambientMainThreadState")
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contract {} uses a forbidden cost base",
                contract.operation,
            )));
        }
    }
    Ok(())
}

fn reject_weak_bridge_allocation_posture(
    posture: &WorkerPhase7BridgeAllocationPosture,
) -> Result<(), WORTHSignalJsError> {
    if posture.posture != "explicitBoundaryAllocationAccounting"
        || posture.serialization_allocation_counter != "bridgeSerializationAllocationCount"
        || posture.deserialization_allocation_counter != "bridgeDeserializationAllocationCount"
        || posture.lifecycle_scope != "bridgeEnvelopeLifecycle"
        || posture.hidden_allocation_allowed
    {
        return Err(WORTHSignalJsError::invalid_input(
            "worker Phase 7 performance contracts require explicit bridge allocation posture",
        ));
    }
    Ok(())
}

fn reject_missing_failure_modes(
    modes: &[WorkerPhase7PerformanceFailureMode],
) -> Result<(), WORTHSignalJsError> {
    for required in required_failure_modes() {
        if !modes.iter().any(|mode| mode.mode == required.mode) {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts require failure mode {}",
                required.mode,
            )));
        }
    }
    Ok(())
}

fn reject_duplicate_failure_modes(
    modes: &[WorkerPhase7PerformanceFailureMode],
) -> Result<(), WORTHSignalJsError> {
    for (index, mode) in modes.iter().enumerate() {
        if modes[(index + 1)..]
            .iter()
            .any(|candidate| candidate.mode == mode.mode)
        {
            return Err(WORTHSignalJsError::invalid_input(format!(
                "worker Phase 7 performance contracts duplicate failure mode {}",
                mode.mode,
            )));
        }
    }
    Ok(())
}
