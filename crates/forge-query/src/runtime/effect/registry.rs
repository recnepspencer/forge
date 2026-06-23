use std::collections::{BTreeMap, BTreeSet};

use super::declaration::{ForgeQueryEffectDeclaration, ForgeQueryEffectTriggerSource};
use super::delivery::{ForgeQueryEffectCounters, ForgeQueryEffectDelivery};
use crate::runtime::{ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget};

#[derive(Clone, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub(in crate::runtime) struct ForgeQueryEffectTarget {
    effect_name: String,
}

impl ForgeQueryEffectTarget {
    pub(in crate::runtime) fn new(effect_name: impl Into<String>) -> Self {
        Self {
            effect_name: effect_name.into(),
        }
    }

    pub(in crate::runtime) fn from_declaration(declaration: &ForgeQueryEffectDeclaration) -> Self {
        Self::new(declaration.name())
    }

    pub(in crate::runtime) fn from_name(effect_name: &str) -> Self {
        Self::new(effect_name)
    }

    pub(in crate::runtime) fn as_str(&self) -> &str {
        &self.effect_name
    }
}

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
    live_to_effects: BTreeMap<ForgeQueryLiveArtifactTarget, BTreeSet<ForgeQueryEffectTarget>>,
    computed_to_effects:
        BTreeMap<ForgeQueryDerivedMaterializationTarget, BTreeSet<ForgeQueryEffectTarget>>,
}
impl ForgeQueryEffectIndex {
    pub(in crate::runtime::effect) fn register(
        &mut self,
        declaration: &ForgeQueryEffectDeclaration,
    ) {
        let effect_target = ForgeQueryEffectTarget::from_declaration(declaration);
        self.unregister(&effect_target);
        match &declaration.trigger().source {
            ForgeQueryEffectTriggerSource::LiveView { view_name } => {
                self.live_to_effects
                    .entry(ForgeQueryLiveArtifactTarget::from_view_name(
                        view_name.clone(),
                    ))
                    .or_default()
                    .insert(effect_target);
            }
            ForgeQueryEffectTriggerSource::ComputedView { view_name } => {
                self.computed_to_effects
                    .entry(ForgeQueryDerivedMaterializationTarget::new(
                        view_name.clone(),
                    ))
                    .or_default()
                    .insert(effect_target);
            }
        }
    }
    fn unregister(&mut self, effect_target: &ForgeQueryEffectTarget) {
        remove_from_index(&mut self.live_to_effects, effect_target);
        remove_from_index(&mut self.computed_to_effects, effect_target);
    }
    pub(in crate::runtime::effect) fn live_candidates<'a>(
        &self,
        targets: impl IntoIterator<Item = &'a ForgeQueryLiveArtifactTarget>,
    ) -> Vec<ForgeQueryEffectTarget> {
        targets
            .into_iter()
            .filter_map(|target| self.live_to_effects.get(target))
            .flatten()
            .cloned()
            .collect()
    }
    pub(in crate::runtime::effect) fn computed_candidates<'a>(
        &self,
        targets: impl IntoIterator<Item = &'a ForgeQueryDerivedMaterializationTarget>,
    ) -> Vec<ForgeQueryEffectTarget> {
        targets
            .into_iter()
            .filter_map(|target| self.computed_to_effects.get(target))
            .flatten()
            .cloned()
            .collect()
    }
}
fn remove_from_index<T: Ord + Clone>(
    index: &mut BTreeMap<T, BTreeSet<ForgeQueryEffectTarget>>,
    effect_target: &ForgeQueryEffectTarget,
) {
    let empty_keys: Vec<T> = index
        .iter_mut()
        .filter_map(|(key, values)| {
            values.remove(effect_target);
            values.is_empty().then(|| key.clone())
        })
        .collect();
    for key in empty_keys {
        index.remove(&key);
    }
}

pub(in crate::runtime) fn insert_effect_runtime(
    effects: &mut BTreeMap<ForgeQueryEffectTarget, ForgeQueryEffectRuntime>,
    effect_index: &mut ForgeQueryEffectIndex,
    declaration: ForgeQueryEffectDeclaration,
) {
    effect_index.register(&declaration);
    effects.insert(
        ForgeQueryEffectTarget::from_declaration(&declaration),
        ForgeQueryEffectRuntime::new(declaration),
    );
}
