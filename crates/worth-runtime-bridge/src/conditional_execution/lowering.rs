use worth_signal::facade::{
    AspectMask, SignalConditionalCondition, SignalConditionalContractDefinition,
};

use super::{BridgeConditionalCondition, BridgeConditionalContract};

pub(super) fn lower_signal_contract(
    contract: &BridgeConditionalContract,
    dependency_aspects: AspectMask,
    condition_aspects: AspectMask,
) -> Result<SignalConditionalContractDefinition, super::BridgeConditionalDenial> {
    let trigger_aspects = condition_aspect_mask(contract, dependency_aspects, condition_aspects);
    let condition = match contract.condition() {
        BridgeConditionalCondition::Always => SignalConditionalCondition::Always,
        BridgeConditionalCondition::AspectFiltered => {
            SignalConditionalCondition::AspectFilter(trigger_aspects)
        }
        BridgeConditionalCondition::DeltaThreshold(threshold) => {
            SignalConditionalCondition::DeltaThreshold(threshold.clone())
        }
        BridgeConditionalCondition::OnDemand => SignalConditionalCondition::OnDemand,
        BridgeConditionalCondition::RuntimePredicate => {
            SignalConditionalCondition::RuntimePredicate
        }
        BridgeConditionalCondition::TemporalWake => SignalConditionalCondition::TemporalWake,
    };
    Ok(SignalConditionalContractDefinition {
        condition,
        dependency_aspects,
        trigger_aspects,
        dependency_comparator: contract.dependency_comparator(),
        output_comparator: contract.output_comparator(),
        artifact_reuse: contract.artifact_reuse(),
    })
}

fn condition_aspect_mask(
    contract: &BridgeConditionalContract,
    dependency_aspects: AspectMask,
    condition_aspects: AspectMask,
) -> AspectMask {
    match contract.condition() {
        BridgeConditionalCondition::OnDemand | BridgeConditionalCondition::TemporalWake => {
            AspectMask::EMPTY
        }
        BridgeConditionalCondition::Always | BridgeConditionalCondition::RuntimePredicate => {
            dependency_aspects
        }
        BridgeConditionalCondition::AspectFiltered
        | BridgeConditionalCondition::DeltaThreshold(_) => condition_aspects,
    }
}
