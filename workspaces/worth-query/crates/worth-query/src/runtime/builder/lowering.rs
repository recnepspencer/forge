#[cfg(test)]
use super::registrations_from_relational_invariant_catalog;
use super::{
    graph_obligation_registration_error, WorthQueryBridgeBackedRuntimeBackend,
    WorthQueryGraphObligationRegistrationCatalog, WorthQueryRuntimeBackend,
    WorthQueryRuntimeBuilder, WorthQueryRuntimeError,
};
#[cfg(test)]
use worth_relational::facade::runtime::InvariantCatalog;

impl WorthQueryRuntimeBuilder {
    #[cfg(test)]
    pub(super) fn queue_relational_schema_contract_obligations(
        &mut self,
        catalog: &InvariantCatalog,
    ) {
        match registrations_from_relational_invariant_catalog(catalog) {
            Ok(registrations) => self
                .queued_graph_obligation_registrations
                .extend(registrations),
            Err(error) => {
                self.graph_obligation_registration_catalog =
                    Some(Err(graph_obligation_registration_error(
                        "relational_schema_contract_obligation_lowering",
                        error,
                    )));
            }
        }
    }

    pub(super) fn assemble_graph_obligation_registration_catalog(
        &mut self,
    ) -> Result<(), WorthQueryRuntimeError> {
        if self.graph_obligation_registration_catalog.is_some() {
            return Ok(());
        }
        let queued = std::mem::take(&mut self.queued_graph_obligation_registrations);
        let catalog = WorthQueryGraphObligationRegistrationCatalog::from_registrations(
            queued.into_explicit_registrations(),
        )
        .map_err(|error| {
            graph_obligation_registration_error(
                "graph_obligation_registration_catalog_assembly",
                error,
            )
        })?;
        self.graph_obligation_registration_catalog = Some(Ok(catalog));
        Ok(())
    }

    pub(super) fn lower_queued_invariant_registrations_into_backend_parts(
        &mut self,
    ) -> Result<(), WorthQueryRuntimeError> {
        if self.queued_invariant_registrations.is_empty() {
            return Ok(());
        }
        if self.backend_parts.has_relational_runtime() {
            return Err(WorthQueryRuntimeError::InvariantRegistration {
                stage: "relational_runtime_authority_selection",
                message: "queued Query-owned invariant registrations conflict with an explicitly supplied relational runtime; choose one authority path".to_string(),
            });
        }
        let queued = std::mem::take(&mut self.queued_invariant_registrations);
        self.backend_parts = std::mem::take(&mut self.backend_parts)
            .relational_runtime(queued.lower_into_relational_runtime());
        Ok(())
    }

    pub(super) fn lower_bridge_backed_backend_from_parts(
        &mut self,
    ) -> Result<Box<dyn WorthQueryRuntimeBackend>, WorthQueryRuntimeError> {
        let bootstrap = std::mem::take(&mut self.backend_parts).lower_bridge_backed_bootstrap()?;
        Ok(Box::new(
            WorthQueryBridgeBackedRuntimeBackend::from_validated_bootstrap(bootstrap),
        ))
    }
}
