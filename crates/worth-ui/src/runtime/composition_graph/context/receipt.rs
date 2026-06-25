use super::super::digest::digest_parts;
use super::counters::WorthUiCompositionContextCounters;
use super::definition::{
    WorthUiCompositionContextValue, WorthUiCompositionLocalePosture, WorthUiCompositionRuntimeMode,
    WorthUiCompositionTextDirection, WorthUiCompositionValidationPosture,
};
use crate::runtime::{
    WorthUiCompositionNodeId, WorthUiQueryGraphExecutionReceipt, WorthUiRuntimeFactId,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextPropagationReceipt {
    node_contexts: Vec<WorthUiCompositionNodeContextReceipt>,
    overrides: Vec<WorthUiCompositionContextOverrideReceipt>,
    affected_consumers: Vec<WorthUiCompositionContextAffectedConsumerRow>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    query_graph_execution: WorthUiQueryGraphExecutionReceipt,
    counters: WorthUiCompositionContextCounters,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionNodeContextReceipt {
    node_id: WorthUiCompositionNodeId,
    effective_context: WorthUiCompositionEffectiveContext,
    local_context_values: Vec<WorthUiCompositionContextValue>,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextOverrideReceipt {
    node_id: WorthUiCompositionNodeId,
    context_kind: &'static str,
    inherited_value: String,
    local_value: String,
    receipt_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionContextAffectedConsumerRow {
    changed_fact: WorthUiRuntimeFactId,
    consumer_fact: WorthUiRuntimeFactId,
    semantic_slice: &'static str,
    row_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompositionEffectiveContext {
    theme: Option<String>,
    density: Option<String>,
    text_direction: WorthUiCompositionTextDirection,
    locale: WorthUiCompositionLocalePosture,
    disabled: bool,
    inert: bool,
    validation: WorthUiCompositionValidationPosture,
    focus_scope: Option<String>,
    runtime_mode: WorthUiCompositionRuntimeMode,
}

impl Default for WorthUiCompositionEffectiveContext {
    fn default() -> Self {
        Self {
            theme: None,
            density: None,
            text_direction: WorthUiCompositionTextDirection::Auto,
            locale: WorthUiCompositionLocalePosture::Unsupported("unset".to_owned()),
            disabled: false,
            inert: false,
            validation: WorthUiCompositionValidationPosture::Unknown,
            focus_scope: None,
            runtime_mode: WorthUiCompositionRuntimeMode::Interactive,
        }
    }
}

impl WorthUiCompositionContextPropagationReceipt {
    pub(crate) fn new(
        node_contexts: Vec<WorthUiCompositionNodeContextReceipt>,
        overrides: Vec<WorthUiCompositionContextOverrideReceipt>,
        affected_consumers: Vec<WorthUiCompositionContextAffectedConsumerRow>,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
        query_graph_execution: WorthUiQueryGraphExecutionReceipt,
        graph_access_count: usize,
    ) -> Self {
        let mut consumed_facts = consumed_facts;
        consumed_facts.sort();
        consumed_facts.dedup();
        let counters = WorthUiCompositionContextCounters::new(
            node_contexts.len(),
            node_contexts
                .iter()
                .map(|receipt| receipt.local_context_values().len())
                .sum(),
            overrides.len(),
            affected_consumers.len(),
            query_graph_execution.selected_obligation_count(),
            graph_access_count,
        );
        let receipt_digest = digest_parts(
            ["composition_context_propagation".to_owned()]
                .into_iter()
                .chain(
                    node_contexts
                        .iter()
                        .map(|row| row.receipt_digest().to_string()),
                )
                .chain(overrides.iter().map(|row| row.receipt_digest().to_string()))
                .chain(
                    affected_consumers
                        .iter()
                        .map(|row| row.row_digest().to_string()),
                )
                .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned()))
                .chain(std::iter::once(
                    query_graph_execution.execution_digest().to_string(),
                )),
        );
        Self {
            node_contexts,
            overrides,
            affected_consumers,
            consumed_facts,
            query_graph_execution,
            counters,
            receipt_digest,
        }
    }

    pub fn node_contexts(&self) -> &[WorthUiCompositionNodeContextReceipt] {
        &self.node_contexts
    }

    pub fn context_for_node(&self, node_id: &str) -> Option<&WorthUiCompositionNodeContextReceipt> {
        self.node_contexts
            .iter()
            .find(|context| context.node_id().as_str() == node_id)
    }

    pub fn overrides(&self) -> &[WorthUiCompositionContextOverrideReceipt] {
        &self.overrides
    }

    pub fn affected_consumers(&self) -> &[WorthUiCompositionContextAffectedConsumerRow] {
        &self.affected_consumers
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.query_graph_execution
    }

    pub fn counters(&self) -> WorthUiCompositionContextCounters {
        self.counters
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionNodeContextReceipt {
    pub(crate) fn new(
        node_id: WorthUiCompositionNodeId,
        effective_context: WorthUiCompositionEffectiveContext,
        local_context_values: Vec<WorthUiCompositionContextValue>,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let receipt_digest = digest_parts(
            [
                "composition_node_context".to_owned(),
                node_id.as_str().to_owned(),
            ]
            .into_iter()
            .chain(effective_context.digest_parts())
            .chain(
                local_context_values
                    .iter()
                    .map(|value| format!("{}={}", value.kind_token(), value.value_token())),
            )
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            node_id,
            effective_context,
            local_context_values,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &WorthUiCompositionNodeId {
        &self.node_id
    }

    pub fn suppresses_interaction(&self) -> bool {
        self.effective_context.disabled || self.effective_context.inert
    }

    pub fn disabled(&self) -> bool {
        self.effective_context.disabled
    }

    pub fn inert(&self) -> bool {
        self.effective_context.inert
    }

    pub fn theme(&self) -> Option<&str> {
        self.effective_context.theme.as_deref()
    }

    pub fn density(&self) -> Option<&str> {
        self.effective_context.density.as_deref()
    }

    pub fn text_direction(&self) -> WorthUiCompositionTextDirection {
        self.effective_context.text_direction
    }

    pub fn locale(&self) -> &WorthUiCompositionLocalePosture {
        &self.effective_context.locale
    }

    pub fn validation(&self) -> WorthUiCompositionValidationPosture {
        self.effective_context.validation
    }

    pub fn runtime_mode(&self) -> WorthUiCompositionRuntimeMode {
        self.effective_context.runtime_mode
    }

    pub fn local_context_values(&self) -> &[WorthUiCompositionContextValue] {
        &self.local_context_values
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionContextOverrideReceipt {
    pub(crate) fn new(
        node_id: WorthUiCompositionNodeId,
        context_kind: &'static str,
        inherited_value: String,
        local_value: String,
    ) -> Self {
        let receipt_digest = digest_parts([
            "composition_context_override",
            node_id.as_str(),
            context_kind,
            &inherited_value,
            &local_value,
        ]);
        Self {
            node_id,
            context_kind,
            inherited_value,
            local_value,
            receipt_digest,
        }
    }

    pub fn node_id(&self) -> &WorthUiCompositionNodeId {
        &self.node_id
    }

    pub fn context_kind(&self) -> &'static str {
        self.context_kind
    }

    pub fn inherited_value(&self) -> &str {
        &self.inherited_value
    }

    pub fn local_value(&self) -> &str {
        &self.local_value
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiCompositionContextAffectedConsumerRow {
    pub(crate) fn new(
        changed_fact: WorthUiRuntimeFactId,
        consumer_fact: WorthUiRuntimeFactId,
    ) -> Self {
        let row_digest = digest_parts([
            "composition_context_affected_consumer",
            changed_fact.identity(),
            consumer_fact.identity(),
        ]);
        Self {
            changed_fact,
            consumer_fact,
            semantic_slice: "CompositionContext",
            row_digest,
        }
    }

    pub fn changed_fact(&self) -> &WorthUiRuntimeFactId {
        &self.changed_fact
    }

    pub fn consumer_fact(&self) -> &WorthUiRuntimeFactId {
        &self.consumer_fact
    }

    pub fn semantic_slice(&self) -> &'static str {
        self.semantic_slice
    }

    pub fn row_digest(&self) -> u64 {
        self.row_digest
    }
}

impl WorthUiCompositionEffectiveContext {
    pub(crate) fn apply(&mut self, value: &WorthUiCompositionContextValue) {
        match value {
            WorthUiCompositionContextValue::Theme(value) => self.theme = Some(value.clone()),
            WorthUiCompositionContextValue::Density(value) => self.density = Some(value.clone()),
            WorthUiCompositionContextValue::TextDirection(value) => self.text_direction = *value,
            WorthUiCompositionContextValue::Locale(value) => self.locale = value.clone(),
            WorthUiCompositionContextValue::Disabled(value) => self.disabled = *value,
            WorthUiCompositionContextValue::Inert(value) => self.inert = *value,
            WorthUiCompositionContextValue::Validation(value) => self.validation = *value,
            WorthUiCompositionContextValue::FocusScope(value) => {
                self.focus_scope = Some(value.clone())
            }
            WorthUiCompositionContextValue::RuntimeMode(value) => self.runtime_mode = *value,
        }
    }

    pub(crate) fn value_for_kind(&self, kind: &str) -> String {
        match kind {
            "theme" => self.theme.clone().unwrap_or_default(),
            "density" => self.density.clone().unwrap_or_default(),
            "text_direction" => self.text_direction.token().to_owned(),
            "locale" => self.locale.token().to_owned(),
            "disabled" => self.disabled.to_string(),
            "inert" => self.inert.to_string(),
            "validation" => self.validation.token().to_owned(),
            "focus_scope" => self.focus_scope.clone().unwrap_or_default(),
            "runtime_mode" => self.runtime_mode.token().to_owned(),
            _ => String::new(),
        }
    }

    fn digest_parts(&self) -> Vec<String> {
        vec![
            format!("theme={}", self.theme.clone().unwrap_or_default()),
            format!("density={}", self.density.clone().unwrap_or_default()),
            format!("direction={}", self.text_direction.token()),
            format!("locale={}", self.locale.token()),
            format!("disabled={}", self.disabled),
            format!("inert={}", self.inert),
            format!("validation={}", self.validation.token()),
            format!("focus={}", self.focus_scope.clone().unwrap_or_default()),
            format!("mode={}", self.runtime_mode.token()),
        ]
    }
}
