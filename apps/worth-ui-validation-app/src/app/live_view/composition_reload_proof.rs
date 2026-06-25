mod counters;
mod rows;

use super::proof::ValidationLiveViewProjectionProof;
use crate::app::ValidationWorkbenchApp;
use worth_ui::facade::{WorthUiLiveViewProjectionRebindReceipt, WorthUiMountedProductViewReceipt};

pub use counters::ValidationLiveViewCompositionReloadCounters;
pub use rows::{
    ValidationLiveViewCompositionRebindDecision, ValidationLiveViewCompositionRebindRow,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationLiveViewCompositionReloadProof {
    prior_product_view: WorthUiMountedProductViewReceipt,
    next_product_view: WorthUiMountedProductViewReceipt,
    projection_rebind: WorthUiLiveViewProjectionRebindReceipt,
    rows: Vec<ValidationLiveViewCompositionRebindRow>,
    counters: ValidationLiveViewCompositionReloadCounters,
}

impl ValidationWorkbenchApp {
    pub fn hot_reload_live_view_source_with_composition_proof(
        &mut self,
        source_text: impl Into<String>,
    ) -> Result<ValidationLiveViewCompositionReloadProof, String> {
        let prior = self.live_view_projection_proof()?;
        let next = self.hot_reload_live_view_source(source_text)?;
        ValidationLiveViewCompositionReloadProof::from_projection_proofs(&prior, &next)
    }
}

impl ValidationLiveViewCompositionReloadProof {
    fn from_projection_proofs(
        prior: &ValidationLiveViewProjectionProof,
        next: &ValidationLiveViewProjectionProof,
    ) -> Result<Self, String> {
        let projection_rebind = next
            .last_rebind()
            .cloned()
            .ok_or_else(|| "live-view reload proof missing projection rebind receipt".to_owned())?;
        let prior_product_view = prior.mounted_product_view().clone();
        let next_product_view = next.mounted_product_view().clone();
        let rows = rows::composition_rebind_rows(&prior_product_view, &next_product_view);
        let counters = ValidationLiveViewCompositionReloadCounters::from_rows(&rows);
        Ok(Self {
            prior_product_view,
            next_product_view,
            projection_rebind,
            rows,
            counters,
        })
    }

    pub fn prior_product_view(&self) -> &WorthUiMountedProductViewReceipt {
        &self.prior_product_view
    }

    pub fn next_product_view(&self) -> &WorthUiMountedProductViewReceipt {
        &self.next_product_view
    }

    pub fn projection_rebind(&self) -> &WorthUiLiveViewProjectionRebindReceipt {
        &self.projection_rebind
    }

    pub fn rows(&self) -> &[ValidationLiveViewCompositionRebindRow] {
        &self.rows
    }

    pub fn counters(&self) -> ValidationLiveViewCompositionReloadCounters {
        self.counters
    }
}
