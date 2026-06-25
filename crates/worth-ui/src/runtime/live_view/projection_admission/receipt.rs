use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt,
    WorthUiAuthoredCompositionAccessibilityAssociationDeclaration, WorthUiCompositionContextDenial,
    WorthUiCompositionContextPropagationReceipt, WorthUiCompositionGraphAdmissionDenial,
    WorthUiCompositionParticipationDenial, WorthUiCompositionSourceAdmissionDenial,
    WorthUiFlowLayoutReceipt, WorthUiLiveViewConditionalProjectionDenial,
    WorthUiLiveViewConditionalProjectionReceipt, WorthUiLiveViewControlProjectionDenial,
    WorthUiLiveViewControlProjectionReceipt, WorthUiLiveViewInteractionIntentDenial,
    WorthUiLiveViewInteractionIntentReceipt, WorthUiLiveViewPayloadProjectionDenial,
    WorthUiLiveViewPayloadProjectionReceipt, WorthUiLiveViewReadinessProjectionDenial,
    WorthUiLiveViewReadinessProjectionReceipt, WorthUiPrimitiveContentReceipt,
    WorthUiPrimitiveContentValueDenialReceipt, WorthUiStatefulAppearanceRecipeReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewProjectionAdmissionReceipt {
    live_view_id: String,
    view_flow_layout: WorthUiFlowLayoutReceipt,
    view_appearance: WorthUiStatefulAppearanceRecipeReceipt,
    controls: Vec<WorthUiLiveViewControlProjectionReceipt>,
    conditionals: Vec<WorthUiLiveViewConditionalProjectionReceipt>,
    readinesses: Vec<WorthUiLiveViewReadinessProjectionReceipt>,
    payloads: Vec<WorthUiLiveViewPayloadProjectionReceipt>,
    interactions: Vec<WorthUiLiveViewInteractionIntentReceipt>,
    contents: Vec<WorthUiPrimitiveContentReceipt>,
    composition_graph: Option<WorthUiAdmittedCompositionGraphReceipt>,
    accessibility_associations: Vec<WorthUiAuthoredCompositionAccessibilityAssociationDeclaration>,
    counters: WorthUiLiveViewProjectionAdmissionCounters,
    admission_digest: u64,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiGraphBackedLiveViewProjectionReceipt {
    projection: WorthUiLiveViewProjectionAdmissionReceipt,
    composition_graph: WorthUiAdmittedCompositionGraphReceipt,
    context_propagation: WorthUiCompositionContextPropagationReceipt,
}

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewProjectionAdmissionCounters {
    control_count: usize,
    conditional_count: usize,
    readiness_count: usize,
    payload_count: usize,
    interaction_count: usize,
    denial_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewProjectionAdmissionReport {
    denials: Vec<WorthUiLiveViewProjectionAdmissionDenial>,
    counters: WorthUiLiveViewProjectionAdmissionCounters,
    denial_set_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthUiLiveViewProjectionAdmissionDenial {
    Control(WorthUiLiveViewControlProjectionDenial),
    Conditional(WorthUiLiveViewConditionalProjectionDenial),
    Readiness(WorthUiLiveViewReadinessProjectionDenial),
    Payload(WorthUiLiveViewPayloadProjectionDenial),
    Interaction(WorthUiLiveViewInteractionIntentDenial),
    Composition(WorthUiCompositionGraphAdmissionDenial),
    CompositionSource(WorthUiCompositionSourceAdmissionDenial),
    CompositionContext(WorthUiCompositionContextDenial),
    CompositionParticipation(WorthUiCompositionParticipationDenial),
    PrimitiveContent(WorthUiPrimitiveContentValueDenialReceipt),
    PrimitiveFlowLayout {
        live_view_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
    PrimitiveAppearanceState {
        live_view_id: String,
        prop_key: String,
        raw_value: String,
        expected: String,
        denial_digest: u64,
    },
}

impl WorthUiLiveViewProjectionAdmissionReceipt {
    pub(in crate::runtime::live_view) fn new(
        live_view_id: impl Into<String>,
        view_flow_layout: WorthUiFlowLayoutReceipt,
        view_appearance: WorthUiStatefulAppearanceRecipeReceipt,
        controls: Vec<WorthUiLiveViewControlProjectionReceipt>,
        conditionals: Vec<WorthUiLiveViewConditionalProjectionReceipt>,
        readinesses: Vec<WorthUiLiveViewReadinessProjectionReceipt>,
        payloads: Vec<WorthUiLiveViewPayloadProjectionReceipt>,
        interactions: Vec<WorthUiLiveViewInteractionIntentReceipt>,
        contents: Vec<WorthUiPrimitiveContentReceipt>,
        composition_graph: Option<WorthUiAdmittedCompositionGraphReceipt>,
        accessibility_associations: Vec<
            WorthUiAuthoredCompositionAccessibilityAssociationDeclaration,
        >,
        counters: WorthUiLiveViewProjectionAdmissionCounters,
    ) -> Self {
        let digest_basis = std::iter::once(view_flow_layout.receipt_digest().to_string())
            .chain(std::iter::once(
                view_appearance.receipt_digest().to_string(),
            ))
            .chain(
                controls
                    .iter()
                    .map(|control| control.control_projection_digest().to_string()),
            )
            .chain(
                conditionals
                    .iter()
                    .map(|conditional| conditional.conditional_projection_digest().to_string()),
            )
            .chain(
                readinesses
                    .iter()
                    .map(|readiness| readiness.readiness_digest().to_string()),
            )
            .chain(
                payloads
                    .iter()
                    .map(|payload| payload.payload_projection_digest().to_string()),
            )
            .chain(
                interactions
                    .iter()
                    .map(|interaction| interaction.interaction_intent_digest().to_string()),
            )
            .chain(
                contents
                    .iter()
                    .map(|content| content.receipt_digest().to_string()),
            )
            .chain(
                composition_graph
                    .iter()
                    .map(|graph| graph.receipt_digest().to_string()),
            );
        let admission_digest = digest_parts(digest_basis);
        Self {
            live_view_id: live_view_id.into(),
            view_flow_layout,
            view_appearance,
            controls,
            conditionals,
            readinesses,
            payloads,
            interactions,
            contents,
            composition_graph,
            accessibility_associations,
            counters,
            admission_digest,
        }
    }

    pub fn controls(&self) -> &[WorthUiLiveViewControlProjectionReceipt] {
        &self.controls
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn view_flow_layout(&self) -> &WorthUiFlowLayoutReceipt {
        &self.view_flow_layout
    }

    pub fn view_appearance(&self) -> &WorthUiStatefulAppearanceRecipeReceipt {
        &self.view_appearance
    }

    pub fn conditionals(&self) -> &[WorthUiLiveViewConditionalProjectionReceipt] {
        &self.conditionals
    }

    pub fn readinesses(&self) -> &[WorthUiLiveViewReadinessProjectionReceipt] {
        &self.readinesses
    }

    pub fn payloads(&self) -> &[WorthUiLiveViewPayloadProjectionReceipt] {
        &self.payloads
    }

    pub fn interactions(&self) -> &[WorthUiLiveViewInteractionIntentReceipt] {
        &self.interactions
    }

    pub fn content_receipts(&self) -> &[WorthUiPrimitiveContentReceipt] {
        &self.contents
    }

    pub fn composition_graph(&self) -> Option<&WorthUiAdmittedCompositionGraphReceipt> {
        self.composition_graph.as_ref()
    }

    pub fn accessibility_associations(
        &self,
    ) -> &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration] {
        &self.accessibility_associations
    }

    pub fn counters(&self) -> WorthUiLiveViewProjectionAdmissionCounters {
        self.counters
    }

    pub fn admission_digest(&self) -> u64 {
        self.admission_digest
    }
}

impl WorthUiGraphBackedLiveViewProjectionReceipt {
    pub(in crate::runtime::live_view) fn new(
        projection: WorthUiLiveViewProjectionAdmissionReceipt,
        composition_graph: WorthUiAdmittedCompositionGraphReceipt,
        context_propagation: WorthUiCompositionContextPropagationReceipt,
    ) -> Self {
        Self {
            projection,
            composition_graph,
            context_propagation,
        }
    }

    pub fn projection(&self) -> &WorthUiLiveViewProjectionAdmissionReceipt {
        &self.projection
    }

    pub fn composition_graph(&self) -> &WorthUiAdmittedCompositionGraphReceipt {
        &self.composition_graph
    }

    pub fn context_propagation(&self) -> &WorthUiCompositionContextPropagationReceipt {
        &self.context_propagation
    }

    pub fn controls(&self) -> &[WorthUiLiveViewControlProjectionReceipt] {
        self.projection.controls()
    }

    pub fn live_view_id(&self) -> &str {
        self.projection.live_view_id()
    }

    pub fn view_flow_layout(&self) -> &WorthUiFlowLayoutReceipt {
        self.projection.view_flow_layout()
    }

    pub fn view_appearance(&self) -> &WorthUiStatefulAppearanceRecipeReceipt {
        self.projection.view_appearance()
    }

    pub fn conditionals(&self) -> &[WorthUiLiveViewConditionalProjectionReceipt] {
        self.projection.conditionals()
    }

    pub fn readinesses(&self) -> &[WorthUiLiveViewReadinessProjectionReceipt] {
        self.projection.readinesses()
    }

    pub fn payloads(&self) -> &[WorthUiLiveViewPayloadProjectionReceipt] {
        self.projection.payloads()
    }

    pub fn interactions(&self) -> &[WorthUiLiveViewInteractionIntentReceipt] {
        self.projection.interactions()
    }

    pub fn content_receipts(&self) -> &[WorthUiPrimitiveContentReceipt] {
        self.projection.content_receipts()
    }

    pub fn accessibility_associations(
        &self,
    ) -> &[WorthUiAuthoredCompositionAccessibilityAssociationDeclaration] {
        self.projection.accessibility_associations()
    }

    pub fn content_receipt_for_subject(
        &self,
        subject_id: &str,
    ) -> Option<&WorthUiPrimitiveContentReceipt> {
        self.content_receipts()
            .iter()
            .find(|receipt| receipt.dependency_fact().identity() == subject_id)
    }

    pub fn counters(&self) -> WorthUiLiveViewProjectionAdmissionCounters {
        self.projection.counters()
    }

    pub fn admission_digest(&self) -> u64 {
        self.projection.admission_digest()
    }
}

impl WorthUiLiveViewProjectionAdmissionCounters {
    pub(in crate::runtime::live_view) fn new(
        control_count: usize,
        conditional_count: usize,
        readiness_count: usize,
        payload_count: usize,
        interaction_count: usize,
        denial_count: usize,
    ) -> Self {
        Self {
            control_count,
            conditional_count,
            readiness_count,
            payload_count,
            interaction_count,
            denial_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn control_count(self) -> usize {
        self.control_count
    }

    pub fn conditional_count(self) -> usize {
        self.conditional_count
    }

    pub fn readiness_count(self) -> usize {
        self.readiness_count
    }

    pub fn payload_count(self) -> usize {
        self.payload_count
    }

    pub fn interaction_count(self) -> usize {
        self.interaction_count
    }

    pub fn denial_count(self) -> usize {
        self.denial_count
    }

    pub fn source_reparse_count(self) -> usize {
        self.source_reparse_count
    }

    pub fn renderer_parse_count(self) -> usize {
        self.renderer_parse_count
    }
}

impl WorthUiLiveViewProjectionAdmissionReport {
    pub(in crate::runtime::live_view) fn denied(
        denials: Vec<WorthUiLiveViewProjectionAdmissionDenial>,
        counters: WorthUiLiveViewProjectionAdmissionCounters,
    ) -> Self {
        let denial_set_digest = digest_parts(denials.iter().map(|denial| denial.code()));
        Self {
            denials,
            counters,
            denial_set_digest,
        }
    }

    pub fn denials(&self) -> &[WorthUiLiveViewProjectionAdmissionDenial] {
        &self.denials
    }

    pub fn counters(&self) -> WorthUiLiveViewProjectionAdmissionCounters {
        self.counters
    }

    pub fn denial_set_digest(&self) -> u64 {
        self.denial_set_digest
    }
}

impl WorthUiLiveViewProjectionAdmissionDenial {
    pub fn code(&self) -> &'static str {
        match self {
            Self::Control(denial) => denial.code(),
            Self::Conditional(denial) => denial.code(),
            Self::Readiness(denial) => denial.code(),
            Self::Payload(denial) => denial.code(),
            Self::Interaction(denial) => denial.code(),
            Self::Composition(_) => "live_view.composition_graph_denied",
            Self::CompositionSource(_) => "live_view.composition_source_denied",
            Self::CompositionContext(_) => "live_view.composition_context_denied",
            Self::CompositionParticipation(_) => "live_view.composition_participation_denied",
            Self::PrimitiveContent(_) => "live_view.primitive_content_denied",
            Self::PrimitiveFlowLayout { .. } => "live_view.primitive_flow_layout_denied",
            Self::PrimitiveAppearanceState { .. } => "live_view.primitive_appearance_denied",
        }
    }
}
