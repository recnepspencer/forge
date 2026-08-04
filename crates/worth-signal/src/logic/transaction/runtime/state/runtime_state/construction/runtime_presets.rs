use crate::data::graph::SignalGraph;

use crate::schema::data::SignalSchemaRegistry;

use super::super::SignalRuntime;

impl<Ctx> SignalRuntime<(), (), (), Ctx, ()> {
    /// Build a runtime with the recommended default setup for a typed app context.
    ///
    /// This defaults to the richer development diagnostics profile rather than
    /// the lean operational one.
    pub fn build(graph: SignalGraph) -> Self {
        Self::development(graph)
    }

    /// Build a runtime with the recommended default setup and a first-class schema registry.
    pub fn build_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        Self::development_with_schema(graph, schema_registry)
    }

    /// Build a runtime with the development policy preset.
    pub fn development(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .development_policy()
            .build()
    }

    /// Build a runtime with the development policy preset and a first-class schema registry.
    pub fn development_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .development_policy()
            .build()
    }

    /// Build a runtime with the operational policy preset.
    pub fn operational(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .operational_policy()
            .build()
    }

    /// Build a runtime with the operational policy preset and a first-class schema registry.
    pub fn operational_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .operational_policy()
            .build()
    }

    /// Build a runtime with the web-development policy preset.
    pub fn web_development(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .web_development_policy()
            .build()
    }

    /// Build a runtime with the web-development policy preset and a first-class schema registry.
    pub fn web_development_with_schema(
        graph: SignalGraph,
        schema_registry: SignalSchemaRegistry,
    ) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .web_development_policy()
            .build()
    }

    /// Build a runtime with the fintech policy preset.
    pub fn fintech(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .fintech_policy()
            .build()
    }

    /// Build a runtime with the fintech policy preset and a first-class schema registry.
    pub fn fintech_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .fintech_policy()
            .build()
    }

    /// Build a runtime with the forensic policy preset.
    pub fn forensic(graph: SignalGraph) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .forensic_policy()
            .build()
    }

    /// Build a runtime with the forensic policy preset and a first-class schema registry.
    pub fn forensic_with_schema(graph: SignalGraph, schema_registry: SignalSchemaRegistry) -> Self {
        SignalRuntime::<(), (), (), (), ()>::builder(graph)
            .with_context::<Ctx>()
            .with_kernel_defaults()
            .schema_registry(schema_registry)
            .forensic_policy()
            .build()
    }
}
