use worth_ui::facade::{
    WorthUiAuthoredLiveViewDocument, WorthUiGraphBackedLiveViewProjectionReceipt,
    WorthUiLiveViewConditionalProjectionReceipt, WorthUiLiveViewControlProjectionReceipt,
    WorthUiLiveViewDeclarationReceipt, WorthUiLiveViewInteractionIntentReceipt,
    WorthUiLiveViewPayloadProjectionReceipt, WorthUiLiveViewProjectionAdmissionReceipt,
    WorthUiLiveViewProjectionAdmissionReport, WorthUiLiveViewProjectionRebindReceipt,
    WorthUiLiveViewProjectionRenderPlan, WorthUiLiveViewReadinessProjectionReceipt,
    WorthUiLiveViewStateEditIntent, WorthUiLiveViewStateValue, WorthUiMountedProductViewReceipt,
};

use super::LIVE_VIEW_ID;
use crate::app::ValidationWorkbenchApp;
use crate::ValidationWorkbenchAuthoredInputs;

#[derive(Clone, Debug)]
pub struct ValidationLiveViewProjectionProof {
    declaration: WorthUiLiveViewDeclarationReceipt,
    graph_backed_projection: WorthUiGraphBackedLiveViewProjectionReceipt,
    projection: WorthUiLiveViewProjectionAdmissionReceipt,
    render_plan: WorthUiLiveViewProjectionRenderPlan,
    mounted_product_view: WorthUiMountedProductViewReceipt,
    pub(super) last_rebind: Option<WorthUiLiveViewProjectionRebindReceipt>,
    controls: Vec<WorthUiLiveViewControlProjectionReceipt>,
    conditionals: Vec<WorthUiLiveViewConditionalProjectionReceipt>,
    readinesses: Vec<WorthUiLiveViewReadinessProjectionReceipt>,
    payloads: Vec<WorthUiLiveViewPayloadProjectionReceipt>,
    interactions: Vec<WorthUiLiveViewInteractionIntentReceipt>,
}

impl ValidationWorkbenchApp {
    pub fn live_view_state_proof(&self) -> Result<WorthUiLiveViewDeclarationReceipt, String> {
        let document = self.live_view_document.as_ref().map_err(Clone::clone)?;
        let declaration = document
            .declaration(LIVE_VIEW_ID)
            .ok_or_else(|| format!("live view declaration {LIVE_VIEW_ID} is missing"))?;
        let target = self
            .workbench()
            .runtime()
            .bind_visible_live_view_target(
                self.workbench().page_host_plan(),
                declaration.target_slot(),
            )
            .map_err(|denial| format!("live view target denied: {denial:?}"))?;
        self.workbench()
            .runtime()
            .admit_authored_live_view_declaration(declaration, target)
            .map_err(|report| format!("live view admission denied: {:?}", report.denials()))
    }

    pub fn live_view_control_edit_intent(
        &self,
        binding_id: &str,
        value: WorthUiLiveViewStateValue,
    ) -> Result<WorthUiLiveViewStateEditIntent, String> {
        let receipt = self.live_view_state_proof()?;
        let binding = receipt
            .binding(binding_id)
            .ok_or_else(|| format!("live view binding {binding_id} is missing"))?;
        Ok(binding.edit(value))
    }

    pub fn live_view_projection_proof(&self) -> Result<ValidationLiveViewProjectionProof, String> {
        if let Err(denial) = &self.live_view_document {
            return Err(denial.clone());
        }
        self.live_view_projection_proof_typed()
            .map_err(|report| format!("live view projections denied: {:?}", report.denials()))
    }

    pub fn live_view_projection_proof_typed(
        &self,
    ) -> Result<ValidationLiveViewProjectionProof, WorthUiLiveViewProjectionAdmissionReport> {
        let document = self
            .live_view_document
            .as_ref()
            .expect("live view document must parse before typed projection admission");
        let authored = document
            .declaration(LIVE_VIEW_ID)
            .expect("live view declaration must exist before typed projection admission");
        let declaration = self
            .live_view_state_proof()
            .expect("live view declaration must admit before typed projection admission");
        let projection = self
            .workbench()
            .runtime()
            .admit_graph_backed_authored_live_view_projections(&declaration, authored)
            .map_err(|report| report)?;
        let render_plan = self
            .workbench()
            .runtime()
            .plan_live_view_projection_render(projection.projection());
        let mounted_product_view = self
            .workbench()
            .runtime()
            .mount_live_view_product_projection_for_page(
                self.workbench().page_host_plan(),
                &projection,
            )
            .expect("validation live-view root must resolve through page authority");
        let projection_receipt = projection.projection().clone();
        Ok(ValidationLiveViewProjectionProof {
            declaration,
            controls: projection.controls().to_vec(),
            conditionals: projection.conditionals().to_vec(),
            readinesses: projection.readinesses().to_vec(),
            payloads: projection.payloads().to_vec(),
            interactions: projection.interactions().to_vec(),
            projection: projection_receipt,
            graph_backed_projection: projection,
            render_plan,
            mounted_product_view,
            last_rebind: None,
        })
    }
}

pub(crate) fn prepare_live_view_document(
    authored_inputs: &ValidationWorkbenchAuthoredInputs,
) -> Result<WorthUiAuthoredLiveViewDocument, String> {
    let authored_source = authored_inputs
        .live_view()
        .ok_or_else(|| "live view source is missing from authored inputs".to_owned())?;
    WorthUiAuthoredLiveViewDocument::parse(authored_source.source_text()).map_err(|denial| {
        format!(
            "live view source denied at line {}: {}",
            denial.line(),
            denial.message()
        )
    })
}

impl ValidationLiveViewProjectionProof {
    pub fn declaration(&self) -> &WorthUiLiveViewDeclarationReceipt {
        &self.declaration
    }

    pub fn controls(&self) -> &[WorthUiLiveViewControlProjectionReceipt] {
        &self.controls
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

    pub fn projection(&self) -> &WorthUiLiveViewProjectionAdmissionReceipt {
        &self.projection
    }

    pub fn graph_backed_projection(&self) -> &WorthUiGraphBackedLiveViewProjectionReceipt {
        &self.graph_backed_projection
    }

    pub fn render_plan(&self) -> &WorthUiLiveViewProjectionRenderPlan {
        &self.render_plan
    }

    pub fn mounted_product_view(&self) -> &WorthUiMountedProductViewReceipt {
        &self.mounted_product_view
    }

    pub fn last_rebind(&self) -> Option<&WorthUiLiveViewProjectionRebindReceipt> {
        self.last_rebind.as_ref()
    }
}
