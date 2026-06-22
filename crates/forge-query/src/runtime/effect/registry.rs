use std::collections::{BTreeMap, BTreeSet};

use super::declaration::{ForgeQueryEffectDeclaration, ForgeQueryEffectTriggerSource};
use super::delivery::{ForgeQueryEffectCounters, ForgeQueryEffectDelivery};

pub(in crate::runtime) struct ForgeQueryEffectRuntime {
    pub(in crate::runtime::effect) declaration: ForgeQueryEffectDeclaration,
    pub(in crate::runtime) deliveries: Vec<ForgeQueryEffectDelivery>,
    pub(in crate::runtime::effect) counters: ForgeQueryEffectCounters,
    pub(in crate::runtime) latest_delivery: Option<ForgeQueryEffectDelivery>,
}
impl ForgeQueryEffectRuntime {
    pub(in crate::runtime::effect) fn new(declaration: ForgeQueryEffectDeclaration) -> Self {
        Self {
            declaration,
            deliveries: Vec::new(),
            counters: ForgeQueryEffectCounters::default(),
            latest_delivery: None,
        }
    }

    pub(in crate::runtime) fn name(&self) -> &str {
        self.declaration.name()
    }

    pub(in crate::runtime) fn effect_policy(&self) -> super::super::ForgeQueryEffectPolicy {
        self.declaration.effect_policy()
    }

    pub(in crate::runtime) fn pending_write_intent_count(&self) -> usize {
        self.counters.pending_write_intents()
    }

    pub(in crate::runtime::effect) fn record_delivery(
        &mut self,
        delivery: ForgeQueryEffectDelivery,
    ) {
        self.latest_delivery = Some(delivery.clone());
        self.deliveries.push(delivery);
    }

    pub(in crate::runtime) fn latest_delivery(&self) -> Option<&ForgeQueryEffectDelivery> {
        self.latest_delivery.as_ref()
    }
}
#[derive(Default)]
pub(in crate::runtime) struct ForgeQueryEffectIndex {
    live_to_effects: BTreeMap<String, BTreeSet<String>>,
    computed_to_effects: BTreeMap<String, BTreeSet<String>>,
}
impl ForgeQueryEffectIndex {
    pub(in crate::runtime::effect) fn register(
        &mut self,
        declaration: &ForgeQueryEffectDeclaration,
    ) {
        self.unregister(declaration.name());
        match &declaration.trigger().source {
            ForgeQueryEffectTriggerSource::LiveView { view_name } => {
                self.live_to_effects
                    .entry(view_name.clone())
                    .or_default()
                    .insert(declaration.name().to_string());
            }
            ForgeQueryEffectTriggerSource::ComputedView { view_name } => {
                self.computed_to_effects
                    .entry(view_name.clone())
                    .or_default()
                    .insert(declaration.name().to_string());
            }
        }
    }
    fn unregister(&mut self, effect_name: &str) {
        remove_from_index(&mut self.live_to_effects, effect_name);
        remove_from_index(&mut self.computed_to_effects, effect_name);
    }
    pub(in crate::runtime::effect) fn live_candidates<'a>(
        &self,
        view_names: impl IntoIterator<Item = &'a str>,
    ) -> Vec<String> {
        view_names
            .into_iter()
            .filter_map(|name| self.live_to_effects.get(name))
            .flatten()
            .cloned()
            .collect()
    }
    pub(in crate::runtime::effect) fn computed_candidates<'a>(
        &self,
        view_names: impl IntoIterator<Item = &'a str>,
    ) -> Vec<String> {
        view_names
            .into_iter()
            .filter_map(|name| self.computed_to_effects.get(name))
            .flatten()
            .cloned()
            .collect()
    }
}
fn remove_from_index(index: &mut BTreeMap<String, BTreeSet<String>>, effect_name: &str) {
    let empty_keys: Vec<String> = index
        .iter_mut()
        .filter_map(|(key, values)| {
            values.remove(effect_name);
            values.is_empty().then(|| key.clone())
        })
        .collect();
    for key in empty_keys {
        index.remove(&key);
    }
}

pub(in crate::runtime) fn insert_effect_runtime(
    effects: &mut BTreeMap<String, ForgeQueryEffectRuntime>,
    effect_index: &mut ForgeQueryEffectIndex,
    declaration: ForgeQueryEffectDeclaration,
) {
    effect_index.register(&declaration);
    effects.insert(
        declaration.name().to_string(),
        ForgeQueryEffectRuntime::new(declaration),
    );
}
