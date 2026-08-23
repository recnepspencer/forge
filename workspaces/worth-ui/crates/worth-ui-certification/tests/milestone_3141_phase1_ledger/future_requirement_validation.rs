use super::super::{
    claim_contract, execution_contract, requirement_contract::RequirementContract, Row,
};

pub(crate) fn validate_open_claim(row: &Row, contract: &RequirementContract) -> Result<(), String> {
    if !matches!(row["phase"].as_str(), "3" | "4" | "5" | "6") {
        return Ok(());
    }
    let scenario = claim_contract::scenario_delta(&row["requirement"])
        .ok_or_else(|| "future requirement omits its mutation case".to_owned())?;
    let mutation = format!("family={};case={scenario}", contract.mutation_family);
    let expected_fault = execution_contract::fault_boundary(&row["requirement"])
        .ok_or_else(|| "future requirement omits its exact fault boundary".to_owned())?;
    if row["scenario_delta"] != scenario
        || row["mutation_control"] != mutation
        || row["fault_injection_boundary"] != expected_fault
    {
        return Err(format!("future claim drifted: {}", row["requirement"]));
    }
    let open_counter = format!("{}=open", contract.counter_family);
    if row["result"] == "OPEN" && row["structural_counters"] != open_counter {
        validate_prepared_open_claim(row, contract)?;
    }
    Ok(())
}

fn validate_prepared_open_claim(row: &Row, contract: &RequirementContract) -> Result<(), String> {
    let expected = execution_contract::counter_amount(&row["requirement"])
        .ok_or_else(|| "prepared future claim lacks an execution counter".to_owned())?;
    if row["structural_counters"] != format!("{}={expected}", contract.counter_family)
        || row["production_entry"] == "not-bound"
        || row["independent_oracle"] == "not-bound"
        || row["exact_command"] == "not-bound"
        || row["source_identity"] == "not-bound"
    {
        return Err("prepared future requirement is not exactly bound".to_owned());
    }
    Ok(())
}
