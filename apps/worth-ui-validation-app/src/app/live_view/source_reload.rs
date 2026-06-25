use super::proof::{prepare_live_view_document, ValidationLiveViewProjectionProof};
use crate::app::ValidationWorkbenchApp;

impl ValidationWorkbenchApp {
    pub fn hot_reload_live_view_source(
        &mut self,
        source_text: impl Into<String>,
    ) -> Result<ValidationLiveViewProjectionProof, String> {
        let prior = self.live_view_projection_proof()?;
        self.replace_live_view_source(source_text)?;
        let mut next = self.live_view_projection_proof()?;
        let rebind = self
            .workbench()
            .runtime()
            .rebind_live_view_projections(prior.projection(), next.projection());
        next.last_rebind = Some(rebind);
        Ok(next)
    }

    pub fn replace_live_view_source_for_test(
        &mut self,
        source_text: impl Into<String>,
    ) -> Result<(), String> {
        self.replace_live_view_source(source_text)
    }

    fn replace_live_view_source(&mut self, source_text: impl Into<String>) -> Result<(), String> {
        let next_inputs = self.baseline_authored_inputs.clone().with_live_view_source(
            crate::reload::ValidationLiveViewSource::new(source_text.into()),
        );
        let next_document = prepare_live_view_document(&next_inputs)?;
        self.baseline_authored_inputs = next_inputs;
        self.live_view_document = Ok(next_document);
        Ok(())
    }
}
