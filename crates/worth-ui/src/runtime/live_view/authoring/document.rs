use super::super::{
    WorthUiLiveViewConditionalProjectionDeclaration, WorthUiLiveViewControlProjectionDeclaration,
    WorthUiLiveViewInteractionIntentDeclaration, WorthUiLiveViewPayloadProjectionDeclaration,
    WorthUiLiveViewReadinessProjectionDeclaration,
};
use super::composition::WorthUiAuthoredCompositionDeclaration;
use super::parse::parse_live_view_document;
use crate::runtime::WorthUiPrimitiveSourceSpan;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredLiveViewDocument {
    pub(super) declarations: Vec<WorthUiAuthoredLiveViewDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredLiveViewDeclaration {
    pub(super) live_view_id: String,
    pub(super) target_slot: String,
    pub(super) primitive_props: Vec<WorthUiAuthoredLiveViewPrimitiveProp>,
    pub(super) bindings: Vec<WorthUiAuthoredLiveViewStateBinding>,
    pub(super) controls: Vec<WorthUiLiveViewControlProjectionDeclaration>,
    pub(super) conditionals: Vec<WorthUiLiveViewConditionalProjectionDeclaration>,
    pub(super) readinesses: Vec<WorthUiLiveViewReadinessProjectionDeclaration>,
    pub(super) payloads: Vec<WorthUiLiveViewPayloadProjectionDeclaration>,
    pub(super) interactions: Vec<WorthUiLiveViewInteractionIntentDeclaration>,
    pub(super) projections: Vec<WorthUiAuthoredLiveViewProjectionDeclaration>,
    pub(super) composition: Option<WorthUiAuthoredCompositionDeclaration>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredLiveViewPrimitiveProp {
    key: String,
    value: String,
    source_span: Option<WorthUiPrimitiveSourceSpan>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredLiveViewStateBinding {
    pub(super) binding_id: String,
    pub(super) state_fact: String,
    pub(super) value_kind: String,
    pub(super) access: String,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiAuthoredLiveViewProjectionDeclaration {
    Control(WorthUiLiveViewControlProjectionDeclaration),
    Conditional(WorthUiLiveViewConditionalProjectionDeclaration),
    Readiness(WorthUiLiveViewReadinessProjectionDeclaration),
    Payload(WorthUiLiveViewPayloadProjectionDeclaration),
    Interaction(WorthUiLiveViewInteractionIntentDeclaration),
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiAuthoredLiveViewParseDenial {
    line: usize,
    message: String,
}

impl WorthUiAuthoredLiveViewDocument {
    pub fn parse(source: &str) -> Result<Self, WorthUiAuthoredLiveViewParseDenial> {
        parse_live_view_document(source)
    }

    pub fn declarations(&self) -> &[WorthUiAuthoredLiveViewDeclaration] {
        &self.declarations
    }

    pub fn declaration(&self, live_view_id: &str) -> Option<&WorthUiAuthoredLiveViewDeclaration> {
        self.declarations
            .iter()
            .find(|declaration| declaration.live_view_id() == live_view_id)
    }
}

impl WorthUiAuthoredLiveViewDeclaration {
    pub(super) fn new(live_view_id: impl Into<String>) -> Self {
        Self {
            live_view_id: live_view_id.into(),
            target_slot: String::new(),
            primitive_props: Vec::new(),
            bindings: Vec::new(),
            controls: Vec::new(),
            conditionals: Vec::new(),
            readinesses: Vec::new(),
            payloads: Vec::new(),
            interactions: Vec::new(),
            projections: Vec::new(),
            composition: None,
        }
    }

    pub(super) fn set_target_slot(&mut self, target_slot: impl Into<String>) {
        self.target_slot = target_slot.into();
    }

    pub(super) fn push_primitive_prop(&mut self, prop: WorthUiAuthoredLiveViewPrimitiveProp) {
        self.primitive_props.push(prop);
    }

    pub(super) fn push_binding(&mut self, binding: WorthUiAuthoredLiveViewStateBinding) {
        self.bindings.push(binding);
    }

    pub(super) fn push_control(&mut self, control: WorthUiLiveViewControlProjectionDeclaration) {
        self.projections
            .push(WorthUiAuthoredLiveViewProjectionDeclaration::Control(
                control.clone(),
            ));
        self.controls.push(control);
    }

    pub(super) fn push_conditional(
        &mut self,
        conditional: WorthUiLiveViewConditionalProjectionDeclaration,
    ) {
        self.projections
            .push(WorthUiAuthoredLiveViewProjectionDeclaration::Conditional(
                conditional.clone(),
            ));
        self.conditionals.push(conditional);
    }

    pub(super) fn push_readiness(
        &mut self,
        readiness: WorthUiLiveViewReadinessProjectionDeclaration,
    ) {
        self.projections
            .push(WorthUiAuthoredLiveViewProjectionDeclaration::Readiness(
                readiness.clone(),
            ));
        self.readinesses.push(readiness);
    }

    pub(super) fn push_payload(&mut self, payload: WorthUiLiveViewPayloadProjectionDeclaration) {
        self.projections
            .push(WorthUiAuthoredLiveViewProjectionDeclaration::Payload(
                payload.clone(),
            ));
        self.payloads.push(payload);
    }

    pub(super) fn push_interaction(
        &mut self,
        interaction: WorthUiLiveViewInteractionIntentDeclaration,
    ) {
        self.projections
            .push(WorthUiAuthoredLiveViewProjectionDeclaration::Interaction(
                interaction.clone(),
            ));
        self.interactions.push(interaction);
    }

    pub(super) fn set_composition(&mut self, composition: WorthUiAuthoredCompositionDeclaration) {
        self.composition = Some(composition);
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn target_slot(&self) -> &str {
        &self.target_slot
    }

    pub fn primitive_props(&self) -> &[WorthUiAuthoredLiveViewPrimitiveProp] {
        &self.primitive_props
    }

    pub fn bindings(&self) -> &[WorthUiAuthoredLiveViewStateBinding] {
        &self.bindings
    }

    pub fn controls(&self) -> &[WorthUiLiveViewControlProjectionDeclaration] {
        &self.controls
    }

    pub fn conditionals(&self) -> &[WorthUiLiveViewConditionalProjectionDeclaration] {
        &self.conditionals
    }

    pub fn readinesses(&self) -> &[WorthUiLiveViewReadinessProjectionDeclaration] {
        &self.readinesses
    }

    pub fn payloads(&self) -> &[WorthUiLiveViewPayloadProjectionDeclaration] {
        &self.payloads
    }

    pub fn interactions(&self) -> &[WorthUiLiveViewInteractionIntentDeclaration] {
        &self.interactions
    }

    pub fn projections(&self) -> &[WorthUiAuthoredLiveViewProjectionDeclaration] {
        &self.projections
    }

    pub fn composition(&self) -> Option<&WorthUiAuthoredCompositionDeclaration> {
        self.composition.as_ref()
    }
}

impl WorthUiAuthoredLiveViewPrimitiveProp {
    pub fn new(
        key: impl Into<String>,
        value: impl Into<String>,
        source_span: Option<WorthUiPrimitiveSourceSpan>,
    ) -> Self {
        Self {
            key: key.into(),
            value: value.into(),
            source_span,
        }
    }

    pub fn key(&self) -> &str {
        &self.key
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn source_span(&self) -> Option<WorthUiPrimitiveSourceSpan> {
        self.source_span
    }
}

impl WorthUiAuthoredLiveViewStateBinding {
    pub(super) fn new(binding_id: impl Into<String>) -> Self {
        Self {
            binding_id: binding_id.into(),
            state_fact: String::new(),
            value_kind: String::new(),
            access: String::new(),
        }
    }

    pub(super) fn set_state_fact(&mut self, state_fact: impl Into<String>) {
        self.state_fact = state_fact.into();
    }

    pub(super) fn set_value_kind(&mut self, value_kind: impl Into<String>) {
        self.value_kind = value_kind.into();
    }

    pub(super) fn set_access(&mut self, access: impl Into<String>) {
        self.access = access.into();
    }

    pub fn binding_id(&self) -> &str {
        &self.binding_id
    }

    pub fn state_fact(&self) -> &str {
        &self.state_fact
    }

    pub fn value_kind(&self) -> &str {
        &self.value_kind
    }

    pub fn access(&self) -> &str {
        &self.access
    }
}

impl WorthUiAuthoredLiveViewParseDenial {
    pub(super) fn new(line: usize, message: impl Into<String>) -> Self {
        Self {
            line,
            message: message.into(),
        }
    }

    pub fn line(&self) -> usize {
        self.line
    }

    pub fn message(&self) -> &str {
        &self.message
    }
}
