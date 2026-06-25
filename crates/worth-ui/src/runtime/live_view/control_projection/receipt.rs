use crate::capability::ComponentId;
use crate::runtime::live_view::digest::digest_parts;
use crate::runtime::{
    WorthUiFlowLayoutReceipt, WorthUiLiveViewStateBindingReceipt,
    WorthUiPrimitiveEventGeometryReceipt, WorthUiQueryGraphExecutionReceipt,
    WorthUiStatefulAppearanceRecipeReceipt,
};

use super::declaration::{
    WorthUiLiveViewControlOptionDeclaration, WorthUiLiveViewControlOptionsSource,
    WorthUiLiveViewControlProjectionDeclaration, WorthUiLiveViewControlProjectionKind,
};

#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct WorthUiLiveViewControlProjectionAdmissionCounters {
    control_count: usize,
    option_source_count: usize,
    denial_count: usize,
    source_reparse_count: usize,
    renderer_parse_count: usize,
}

#[derive(Clone, Debug, PartialEq)]
pub struct WorthUiLiveViewControlProjectionReceipt {
    live_view_id: String,
    control_id: String,
    component_id: ComponentId,
    binding: WorthUiLiveViewStateBindingReceipt,
    kind: WorthUiLiveViewControlProjectionKind,
    label: String,
    options: Option<WorthUiLiveViewControlOptionsReceipt>,
    flow_layout: WorthUiFlowLayoutReceipt,
    appearance: WorthUiStatefulAppearanceRecipeReceipt,
    event_geometry: WorthUiPrimitiveEventGeometryReceipt,
    graph_execution: WorthUiQueryGraphExecutionReceipt,
    control_projection_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlOptionsReceipt {
    source_id: String,
    options: Vec<WorthUiLiveViewControlOptionReceipt>,
    options_digest: u64,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiLiveViewControlOptionReceipt {
    value: String,
    label: String,
}

impl WorthUiLiveViewControlProjectionAdmissionCounters {
    pub(crate) fn new(
        control_count: usize,
        option_source_count: usize,
        denial_count: usize,
    ) -> Self {
        Self {
            control_count,
            option_source_count,
            denial_count,
            source_reparse_count: 0,
            renderer_parse_count: 0,
        }
    }

    pub fn control_count(self) -> usize {
        self.control_count
    }

    pub fn option_source_count(self) -> usize {
        self.option_source_count
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

impl WorthUiLiveViewControlProjectionReceipt {
    pub(crate) fn new(
        live_view_id: &str,
        declaration: &WorthUiLiveViewControlProjectionDeclaration,
        component_id: ComponentId,
        binding: WorthUiLiveViewStateBindingReceipt,
        flow_layout: WorthUiFlowLayoutReceipt,
        appearance: WorthUiStatefulAppearanceRecipeReceipt,
        event_geometry: WorthUiPrimitiveEventGeometryReceipt,
        graph_execution: WorthUiQueryGraphExecutionReceipt,
    ) -> Self {
        let options = declaration
            .options()
            .and_then(WorthUiLiveViewControlOptionsReceipt::from_source);
        let option_digest = options
            .as_ref()
            .map(|options| options.options_digest().to_string())
            .unwrap_or_else(|| "no_options".to_owned());
        let control_projection_digest = digest_parts([
            live_view_id,
            declaration.control_id(),
            component_id.as_str(),
            binding.binding_digest().to_string().as_str(),
            declaration.kind().token(),
            declaration.label(),
            option_digest.as_str(),
            flow_layout.receipt_digest().to_string().as_str(),
            appearance.receipt_digest().to_string().as_str(),
            event_geometry.receipt_digest().to_string().as_str(),
        ]);
        Self {
            live_view_id: live_view_id.to_owned(),
            control_id: declaration.control_id().to_owned(),
            component_id,
            binding,
            kind: declaration.kind().clone(),
            label: declaration.label().to_owned(),
            options,
            flow_layout,
            appearance,
            event_geometry,
            graph_execution,
            control_projection_digest,
        }
    }

    pub fn live_view_id(&self) -> &str {
        &self.live_view_id
    }

    pub fn control_id(&self) -> &str {
        &self.control_id
    }

    pub fn component_id(&self) -> &ComponentId {
        &self.component_id
    }

    pub fn binding(&self) -> &WorthUiLiveViewStateBindingReceipt {
        &self.binding
    }

    pub fn kind(&self) -> &WorthUiLiveViewControlProjectionKind {
        &self.kind
    }

    pub fn label(&self) -> &str {
        &self.label
    }

    pub fn options(&self) -> Option<&WorthUiLiveViewControlOptionsReceipt> {
        self.options.as_ref()
    }

    pub fn flow_layout(&self) -> &WorthUiFlowLayoutReceipt {
        &self.flow_layout
    }

    pub fn appearance(&self) -> &WorthUiStatefulAppearanceRecipeReceipt {
        &self.appearance
    }

    pub fn event_geometry(&self) -> &WorthUiPrimitiveEventGeometryReceipt {
        &self.event_geometry
    }

    pub fn query_graph_execution(&self) -> &WorthUiQueryGraphExecutionReceipt {
        &self.graph_execution
    }

    pub fn control_projection_digest(&self) -> u64 {
        self.control_projection_digest
    }
}

impl WorthUiLiveViewControlOptionsReceipt {
    fn from_source(source: &WorthUiLiveViewControlOptionsSource) -> Option<Self> {
        let WorthUiLiveViewControlOptionsSource::Static { source_id, options } = source else {
            return None;
        };
        let options = options
            .iter()
            .map(WorthUiLiveViewControlOptionReceipt::from_declaration)
            .collect::<Vec<_>>();
        let options_digest = digest_parts(
            std::iter::once(source_id.as_str()).chain(
                options
                    .iter()
                    .flat_map(|option| [option.value.as_str(), option.label.as_str()]),
            ),
        );
        Some(Self {
            source_id: source_id.to_owned(),
            options,
            options_digest,
        })
    }

    pub fn source_id(&self) -> &str {
        &self.source_id
    }

    pub fn options(&self) -> &[WorthUiLiveViewControlOptionReceipt] {
        &self.options
    }

    pub fn options_digest(&self) -> u64 {
        self.options_digest
    }
}

impl WorthUiLiveViewControlOptionReceipt {
    fn from_declaration(declaration: &WorthUiLiveViewControlOptionDeclaration) -> Self {
        Self {
            value: declaration.value().to_owned(),
            label: declaration.label().to_owned(),
        }
    }

    pub fn value(&self) -> &str {
        &self.value
    }

    pub fn label(&self) -> &str {
        &self.label
    }
}
