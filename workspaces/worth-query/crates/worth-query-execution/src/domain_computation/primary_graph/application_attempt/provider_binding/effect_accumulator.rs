use std::collections::BTreeMap;
use std::sync::Arc;

use worth_relational::facade::transactions::{EntityReference, MutationIntent, WorkerIntentBatch};

use super::effect_lowering::{
    created_entity_symbols, lower_provider_effect, WorthQueryLoweredProviderEffect,
};
use super::{
    retained_bytes_denial, WorthQueryAdmittedApplicationEmissionBatch,
    WorthQueryApplicationAttemptDenial, WorthQueryApplicationObservedFact,
    WorthQueryApplicationRealizedEffect,
};
use crate::domain_computation::primary_graph::application_attempt::WorthQueryApplicationEmission;
use crate::domain_computation::WorthQueryProvisionalEffectStep;

pub(super) struct WorthQueryProviderEffectAccumulator<'facts> {
    facts: &'facts [WorthQueryApplicationObservedFact],
    symbols: BTreeMap<EntityReference, Arc<str>>,
    lowered: Vec<WorthQueryLoweredProviderEffect>,
}

pub(super) struct WorthQueryCompletedProviderEffectLowering {
    steps: Vec<WorthQueryProvisionalEffectStep>,
    batch: WorkerIntentBatch,
    emissions: WorthQueryAdmittedApplicationEmissionBatch,
}

impl<'facts> WorthQueryProviderEffectAccumulator<'facts> {
    pub(super) fn new(
        facts: &'facts [WorthQueryApplicationObservedFact],
        effects: &[WorthQueryApplicationRealizedEffect],
    ) -> Self {
        Self {
            facts,
            symbols: created_entity_symbols(effects),
            lowered: Vec::with_capacity(effects.len()),
        }
    }

    pub(super) fn add_effect(
        &mut self,
        effect: WorthQueryApplicationRealizedEffect,
    ) -> Result<(), WorthQueryApplicationAttemptDenial> {
        let lowered = lower_provider_effect(self.facts, &self.symbols, effect)?;
        self.lowered.push(lowered);
        Ok(())
    }

    pub(super) fn finish(
        self,
        expected_emission_retained_bytes: u64,
        emission_retained_bytes_ceiling: u64,
    ) -> Result<WorthQueryCompletedProviderEffectLowering, WorthQueryApplicationAttemptDenial> {
        let (steps, intents, emissions) = materialize_effect_collections(self.lowered);
        let batch = intents.into_iter().fold(
            WorkerIntentBatch::new("application-provider-attempt"),
            WorkerIntentBatch::push,
        );
        let emissions = WorthQueryAdmittedApplicationEmissionBatch::admit(
            emissions,
            emission_retained_bytes_ceiling,
        )
        .map_err(|_| retained_bytes_denial())?;
        if emissions.retained_bytes() != expected_emission_retained_bytes {
            return Err(retained_bytes_denial());
        }
        Ok(WorthQueryCompletedProviderEffectLowering {
            steps,
            batch,
            emissions,
        })
    }
}

impl WorthQueryCompletedProviderEffectLowering {
    pub(super) fn into_parts(
        self,
    ) -> (
        Vec<WorthQueryProvisionalEffectStep>,
        WorkerIntentBatch,
        WorthQueryAdmittedApplicationEmissionBatch,
    ) {
        (self.steps, self.batch, self.emissions)
    }
}

fn materialize_effect_collections(
    lowered: Vec<WorthQueryLoweredProviderEffect>,
) -> (
    Vec<WorthQueryProvisionalEffectStep>,
    Vec<MutationIntent>,
    Vec<WorthQueryApplicationEmission>,
) {
    let mut steps = Vec::new();
    let mut intents = Vec::new();
    let mut emissions = Vec::new();
    for effect in lowered {
        match effect {
            WorthQueryLoweredProviderEffect::Mutation {
                steps: effect_steps,
                intent,
            } => {
                steps.extend(effect_steps);
                intents.push(intent);
            }
            WorthQueryLoweredProviderEffect::Emission(emission) => emissions.push(emission),
        }
    }
    (steps, intents, emissions)
}
