#[path = "routing_aspects.rs"]
mod routing_aspects;

use std::collections::{BTreeMap, BTreeSet};

use crate::memory_workspace::ForgeQueryMutationReceipt;

use super::super::{
    ForgeQueryAuthorityLane, ForgeQueryDerivedViewRuntime, ForgeQueryEffectAction,
    ForgeQueryRuntimeError,
};
use super::declaration::{
    ForgeQueryEffectCondition, ForgeQueryEffectDeclaration,
    ForgeQueryEffectExpressionFailurePosture, ForgeQueryEffectSuppressionPolicy,
    ForgeQueryEffectTriggerSource, ForgeQueryEffectTriggerSourceKind,
};
use super::delivery::{ForgeQueryEffectDelivery, ForgeQueryEffectPayload};
use super::registry::{ForgeQueryEffectIndex, ForgeQueryEffectRuntime};
use crate::runtime::ForgeQueryAspectTouch;
use crate::runtime::{ForgeQueryDerivedMaterializationTarget, ForgeQueryLiveArtifactTarget};
use routing_aspects::{aspects_match, insert_declared_aspects, validate_declared_effect_aspects};

pub(in crate::runtime) fn admit_effect_declaration(
    live_view_names: &BTreeSet<String>,
    computed_view_names: &BTreeSet<String>,
    declaration: &ForgeQueryEffectDeclaration,
) -> Result<(), ForgeQueryRuntimeError> {
    if declaration.name().is_empty() {
        return Err(effect_declaration_error(
            declaration.name(),
            "name-admission",
            "effect name may not be empty",
        ));
    }
    declaration
        .effect_policy()
        .admit(declaration.action(), declaration.target_lane())
        .map_err(ForgeQueryRuntimeError::EffectPolicyDenied)?;
    match (declaration.action(), declaration.target_lane()) {
        (ForgeQueryEffectAction::Deliver, ForgeQueryAuthorityLane::EffectDeliveryState) => {}
        (ForgeQueryEffectAction::WriteIntent, ForgeQueryAuthorityLane::PendingWriteIntent) => {}
        (ForgeQueryEffectAction::WriteIntent, _) => {
            return Err(effect_declaration_error(
                declaration.name(),
                "write-intent-admission",
                "effect-triggered writes must lower to pending write intent authority before any commit",
            ));
        }
        _ => {
            return Err(effect_declaration_error(
                declaration.name(),
                "authority-admission",
                "effect declarations may only deliver into effect delivery state or lower into pending write intent authority",
            ));
        }
    }
    if declaration.trigger().aspect_touches().is_empty() {
        return Err(effect_declaration_error(
            declaration.name(),
            "trigger-admission",
            "effect trigger must declare at least one aspect",
        ));
    }
    validate_declared_effect_aspects(declaration.trigger().aspect_touches()).map_err(
        |message| effect_declaration_error(declaration.name(), "trigger-admission", message),
    )?;
    match &declaration.trigger().source {
        ForgeQueryEffectTriggerSource::LiveView { view_name } => {
            if !live_view_names.contains(view_name) {
                return Err(effect_declaration_error(
                    declaration.name(),
                    "trigger-admission",
                    format!("live trigger `{view_name}` is not declared"),
                ));
            }
        }
        ForgeQueryEffectTriggerSource::ComputedView { view_name } => {
            if !computed_view_names.contains(view_name) {
                return Err(effect_declaration_error(
                    declaration.name(),
                    "trigger-admission",
                    format!("computed trigger `{view_name}` is not declared"),
                ));
            }
        }
    }
    if let ForgeQueryEffectCondition::Expression(expression) = declaration.condition() {
        if expression.descriptor().is_empty() {
            return Err(effect_declaration_error(
                declaration.name(),
                "condition-admission",
                "expression descriptor may not be empty",
            ));
        }
        if expression.input_aspect_touches().is_empty()
            || expression.output_aspect_touches().is_empty()
        {
            return Err(effect_declaration_error(
                declaration.name(),
                "condition-admission",
                "expression conditions must declare input and output aspects",
            ));
        }
        validate_declared_effect_aspects(expression.input_aspect_touches()).map_err(|message| {
            effect_declaration_error(declaration.name(), "condition-admission", message)
        })?;
        validate_declared_effect_aspects(expression.output_aspect_touches()).map_err(
            |message| effect_declaration_error(declaration.name(), "condition-admission", message),
        )?;
    }
    Ok(())
}
fn effect_declaration_error(
    effect_name: &str,
    stage: &'static str,
    message: impl Into<String>,
) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::EffectDeclaration {
        effect_name: effect_name.to_string(),
        stage,
        message: message.into(),
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub(in crate::runtime) struct ForgeQueryEffectRouteResult {
    considered_effect_count: usize,
    delivered_effect_count: usize,
    pending_write_intent_count: usize,
    suppressed_effect_count: usize,
    meaningful_suppression_count: usize,
    expression_failure_count: usize,
}
impl ForgeQueryEffectRouteResult {
    pub(in crate::runtime) fn considered_effect_count(&self) -> usize {
        self.considered_effect_count
    }
    pub(in crate::runtime) fn delivered_effect_count(&self) -> usize {
        self.delivered_effect_count
    }
    pub(in crate::runtime) fn pending_write_intent_count(&self) -> usize {
        self.pending_write_intent_count
    }
    pub(in crate::runtime) fn suppressed_effect_count(&self) -> usize {
        self.suppressed_effect_count
    }
    pub(in crate::runtime) fn meaningful_suppression_count(&self) -> usize {
        self.meaningful_suppression_count
    }
    pub(in crate::runtime) fn expression_failure_count(&self) -> usize {
        self.expression_failure_count
    }
}
pub(in crate::runtime) fn route_effect_deliveries(
    effects: &mut BTreeMap<String, ForgeQueryEffectRuntime>,
    effect_index: &ForgeQueryEffectIndex,
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    live_view_targets: &BTreeMap<String, String>,
    receipt: &ForgeQueryMutationReceipt,
    affected_live_view_targets: &[ForgeQueryLiveArtifactTarget],
    affected_derived_view_targets: &[ForgeQueryDerivedMaterializationTarget],
) -> ForgeQueryEffectRouteResult {
    let mut candidates: BTreeSet<String> = BTreeSet::new();
    candidates.extend(
        effect_index.live_candidates(
            affected_live_view_targets
                .iter()
                .map(|target| target.view_name()),
        ),
    );
    candidates.extend(
        effect_index.computed_candidates(
            affected_derived_view_targets
                .iter()
                .map(|target| target.view_name()),
        ),
    );
    let mut result = ForgeQueryEffectRouteResult::default();
    for effect_name in candidates {
        let Some(effect) = effects.get_mut(&effect_name) else {
            continue;
        };
        result.considered_effect_count += 1;
        effect.counters.considered += 1;
        let trigger = collect_trigger_change(
            &effect.declaration,
            derived_views,
            live_view_targets,
            receipt,
        );
        let Some(trigger) = trigger else {
            let reason = match effect.declaration.suppression_policy() {
                ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta => {
                    "meaningful semantic delta suppression: trigger source changed but declared aspects did not"
                }
                ForgeQueryEffectSuppressionPolicy::None => {
                    "trigger source changed but declared aspects did not"
                }
            };
            let suppressed = ForgeQueryEffectDelivery::suppressed(
                &effect.declaration,
                receipt.commit_identity.clone(),
                effect.declaration.trigger().source_name(),
                effect.declaration.trigger().source_kind(),
                reason,
            );
            effect.record_delivery(suppressed);
            effect.counters.suppressed += 1;
            result.suppressed_effect_count += 1;
            if effect.declaration.suppression_policy()
                == ForgeQueryEffectSuppressionPolicy::MeaningfulSemanticDelta
            {
                effect.counters.meaningful_suppressions += 1;
                result.meaningful_suppression_count += 1;
            }
            continue;
        };
        match evaluate_condition(effect.declaration.condition(), &trigger.aspect_touches) {
            EffectConditionOutcome::Delivered(payload) => match effect.declaration.action() {
                ForgeQueryEffectAction::Deliver => {
                    let delivery = ForgeQueryEffectDelivery::delivered(
                        &effect.declaration,
                        receipt.commit_identity.clone(),
                        trigger.source,
                        trigger.source_kind,
                        trigger.aspect_touches,
                        payload,
                    );
                    effect.record_delivery(delivery);
                    effect.counters.delivered += 1;
                    result.delivered_effect_count += 1;
                }
                ForgeQueryEffectAction::WriteIntent => {
                    let delivery = ForgeQueryEffectDelivery::pending_write_intent(
                        &effect.declaration,
                        receipt.commit_identity.clone(),
                        trigger.source,
                        trigger.source_kind,
                        trigger.aspect_touches,
                        payload,
                    );
                    effect.record_delivery(delivery);
                    effect.counters.pending_write_intents += 1;
                    result.pending_write_intent_count += 1;
                }
                ForgeQueryEffectAction::Derive => {
                    let delivery = ForgeQueryEffectDelivery::suppressed(
                            &effect.declaration,
                            receipt.commit_identity.clone(),
                            trigger.source,
                            trigger.source_kind,
                            "derive-only effects are admitted by policy but not executable as runtime deliveries",
                        );
                    effect.record_delivery(delivery);
                    effect.counters.suppressed += 1;
                    result.suppressed_effect_count += 1;
                }
            },
            EffectConditionOutcome::Suppressed(reason) => {
                let delivery = ForgeQueryEffectDelivery::suppressed(
                    &effect.declaration,
                    receipt.commit_identity.clone(),
                    trigger.source,
                    trigger.source_kind,
                    reason,
                );
                effect.record_delivery(delivery);
                effect.counters.suppressed += 1;
                result.suppressed_effect_count += 1;
            }
            EffectConditionOutcome::Failed(reason) => {
                let delivery = ForgeQueryEffectDelivery::expression_failed(
                    &effect.declaration,
                    receipt.commit_identity.clone(),
                    trigger.source,
                    trigger.source_kind,
                    trigger.aspect_touches,
                    reason,
                );
                effect.record_delivery(delivery);
                effect.counters.expression_failures += 1;
                result.expression_failure_count += 1;
            }
        }
    }
    result
}
struct TriggerChange {
    source: String,
    source_kind: ForgeQueryEffectTriggerSourceKind,
    aspect_touches: Vec<ForgeQueryAspectTouch>,
}
fn collect_trigger_change(
    declaration: &ForgeQueryEffectDeclaration,
    derived_views: &BTreeMap<String, ForgeQueryDerivedViewRuntime>,
    live_view_targets: &BTreeMap<String, String>,
    receipt: &ForgeQueryMutationReceipt,
) -> Option<TriggerChange> {
    match &declaration.trigger().source {
        ForgeQueryEffectTriggerSource::LiveView { view_name } => {
            let aspects = declaration.trigger().aspect_touches();
            let target = live_view_targets.get(view_name)?;
            let mut changed_aspects = BTreeSet::new();
            for delta in &receipt.deltas {
                if &delta.collection != target {
                    continue;
                }
                if delta.admitted_touched_aspects().is_empty() {
                    insert_declared_aspects(&mut changed_aspects, aspects);
                } else {
                    for touch in delta
                        .admitted_touched_aspects()
                        .iter()
                        .filter(|touch| aspects_match(aspects, touch))
                    {
                        changed_aspects.insert(touch.clone());
                    }
                }
            }
            (!changed_aspects.is_empty()).then(|| TriggerChange {
                source: view_name.clone(),
                source_kind: ForgeQueryEffectTriggerSourceKind::LiveView,
                aspect_touches: changed_aspects.into_iter().collect(),
            })
        }
        ForgeQueryEffectTriggerSource::ComputedView { view_name } => {
            let aspects = declaration.trigger().aspect_touches();
            let view = derived_views.get(view_name)?;
            let mut changed_aspects = BTreeSet::new();
            for patch in &view.patches {
                if patch.commit_identity() != &receipt.commit_identity {
                    continue;
                }
                if patch.aspect_touches().is_empty() {
                    insert_declared_aspects(&mut changed_aspects, aspects);
                } else {
                    for aspect in patch
                        .aspect_touches()
                        .iter()
                        .filter(|touch| aspects_match(aspects, touch))
                    {
                        changed_aspects.insert(aspect.clone());
                    }
                }
            }
            (!changed_aspects.is_empty()).then(|| TriggerChange {
                source: view_name.clone(),
                source_kind: ForgeQueryEffectTriggerSourceKind::ComputedView,
                aspect_touches: changed_aspects.into_iter().collect(),
            })
        }
    }
}
enum EffectConditionOutcome {
    Delivered(ForgeQueryEffectPayload),
    Suppressed(String),
    Failed(String),
}
fn evaluate_condition(
    condition: &ForgeQueryEffectCondition,
    changed_aspects: &[ForgeQueryAspectTouch],
) -> EffectConditionOutcome {
    match condition {
        ForgeQueryEffectCondition::Always => {
            EffectConditionOutcome::Delivered(ForgeQueryEffectPayload::always(changed_aspects))
        }
        ForgeQueryEffectCondition::Expression(expression) => {
            if expression.failure_posture()
                == ForgeQueryEffectExpressionFailurePosture::DeterministicFailure
            {
                return EffectConditionOutcome::Failed(format!(
                    "expression `{}` reported deterministic failure",
                    expression.descriptor()
                ));
            }
            let input_hits = changed_aspects
                .iter()
                .any(|aspect| aspects_match(expression.input_aspect_touches(), aspect));
            if !input_hits {
                return EffectConditionOutcome::Suppressed(format!(
                    "expression `{}` inputs were not changed",
                    expression.descriptor()
                ));
            }
            EffectConditionOutcome::Delivered(ForgeQueryEffectPayload::expression(
                expression.descriptor(),
                expression.input_aspect_touches(),
                expression.output_aspect_touches(),
                changed_aspects,
            ))
        }
    }
}
