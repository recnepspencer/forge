pub(crate) fn workflow_name(case: &str, objective: &str) -> String {
    format!("fintech.{case}.{objective}")
}

pub(crate) fn scenario_name(case: &str, lane: &str) -> String {
    format!("fintech.{case}.{lane}")
}

pub(crate) fn invariant_id(case: &str, invariant: &str) -> String {
    format!("{case}:{invariant}")
}
