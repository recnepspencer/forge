use crate::boundary::errors::WorthSignalJsError;
use crate::boundary::signals_model::InputOptions;
use crate::expression::model::SignalValue;
use crate::recipe::model::SourceSpec;

use super::super::super::aspects::{
    aspect_mask_from_list, defaulted_produced_aspects, initial_aspect_version,
};
use super::super::super::state::{CatalogEntry, StoredSource, WebSignalKind};
use super::super::super::RuntimeCore;

impl RuntimeCore {
    pub fn define_source(&mut self, spec: SourceSpec) -> Result<(), WorthSignalJsError> {
        self.ensure_unique_id(&spec.id)?;
        let source_id = spec.id.clone();
        let produced_aspects = defaulted_produced_aspects(spec.produces_aspects.as_deref());
        let node = self
            .runtime
            .graph_mut()
            .node()
            .produces_aspects(aspect_mask_from_list(&produced_aspects))
            .build();
        self.catalog.insert(
            source_id.clone(),
            CatalogEntry {
                node,
                produced_aspects: produced_aspects.clone(),
            },
        );
        self.nodes_by_id.insert(node, source_id.clone());
        let mut store = self.lock_store()?;
        store.sources.insert(
            source_id.clone(),
            StoredSource {
                value: spec.initial,
                version: initial_aspect_version(&produced_aspects),
            },
        );
        drop(store);

        let evaluator = self.evaluator();
        self.runtime
            .read(node, &self.store, &evaluator)
            .map_err(WorthSignalJsError::from)?;
        self.runtime.clear_live_branch_mutation_residue();
        Ok(())
    }

    pub fn define_web_input(
        &mut self,
        id: String,
        initial: SignalValue,
        options: Option<InputOptions>,
    ) -> Result<(), WorthSignalJsError> {
        self.define_source(SourceSpec {
            id: id.clone(),
            initial,
            produces_aspects: options.and_then(|options| options.produces_aspects),
        })?;
        self.web_signals.insert(id, WebSignalKind::Input);
        Ok(())
    }
}
