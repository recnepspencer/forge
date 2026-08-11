use worth_signal::facade::{Aspect, AspectVersion, DependencyEdge, NodeId};

use crate::boundary::errors::WorthSignalJsError;
use crate::expression::model::SignalValue;
use crate::recipe::model::{RecipeReadSpec, RecipeSpec, WasmAspectId};

use super::super::super::aspects::{
    aspect_mask_from_list, defaulted_produced_aspects, resolve_selected_aspects,
};
use super::super::super::state::{
    CatalogEntry, StoredRecipe, StoredRecipeDefinition, StoredRecipeOrigin,
};
use super::super::super::RuntimeCore;

struct RecipeDependencyProjection {
    read_aspects: Vec<Aspect>,
    dependencies: Vec<DependencyEdge>,
}

struct RecipeGraphPublication {
    node: NodeId,
    produced_aspects: Vec<Aspect>,
}

impl RuntimeCore {
    pub fn define_recipe(&mut self, spec: RecipeSpec) -> Result<(), WorthSignalJsError> {
        let id = spec.id.clone();
        self.insert_recipe_definition(
            id,
            spec.reads.clone(),
            spec.produces_aspects.clone(),
            StoredRecipeOrigin::ExprSpec,
            StoredRecipeDefinition::Expr(spec),
        )
    }

    pub(super) fn insert_recipe_definition(
        &mut self,
        id: String,
        reads: Vec<RecipeReadSpec>,
        produces_aspects_spec: Option<Vec<WasmAspectId>>,
        origin: StoredRecipeOrigin,
        definition: StoredRecipeDefinition,
    ) -> Result<(), WorthSignalJsError> {
        self.admit_recipe_definition(&id, &reads)?;
        let RecipeDependencyProjection {
            read_aspects,
            dependencies,
        } = self.project_recipe_dependencies(&reads)?;
        let graph_publication = self.publish_recipe_graph(
            &read_aspects,
            produces_aspects_spec.as_deref(),
            dependencies,
        )?;
        self.publish_recipe_catalog(
            &id,
            graph_publication.node,
            graph_publication.produced_aspects,
        );
        self.publish_recipe_store(id, definition, origin)
    }

    fn admit_recipe_definition(
        &self,
        id: &str,
        reads: &[RecipeReadSpec],
    ) -> Result<(), WorthSignalJsError> {
        self.ensure_unique_id(id)?;
        self.ensure_known_reads(reads)
    }

    fn project_recipe_dependencies(
        &self,
        reads: &[RecipeReadSpec],
    ) -> Result<RecipeDependencyProjection, WorthSignalJsError> {
        let mut read_aspects = Vec::new();
        let mut dependencies = Vec::new();
        for read in reads {
            let entry = self.catalog.get(read.id()).ok_or_else(|| {
                WorthSignalJsError::invalid_input(format!("unknown read `{}`", read.id()))
            })?;
            let aspects = resolve_selected_aspects(read.aspect_spec())?;
            read_aspects.extend(aspects.iter().copied());
            for aspect in aspects {
                let edge = match read.scope() {
                    Some(scope) => {
                        DependencyEdge::with_partition_scope(entry.node, aspect, scope.clone())
                    }
                    None => DependencyEdge::new(entry.node, aspect),
                };
                dependencies.push(edge);
            }
        }
        read_aspects.sort_by_key(|aspect| aspect.id());
        read_aspects.dedup_by_key(|aspect| aspect.id());
        Ok(RecipeDependencyProjection {
            read_aspects,
            dependencies,
        })
    }

    fn publish_recipe_graph(
        &mut self,
        read_aspects: &[Aspect],
        produces_aspects_spec: Option<&[WasmAspectId]>,
        dependencies: Vec<DependencyEdge>,
    ) -> Result<RecipeGraphPublication, WorthSignalJsError> {
        let produced_aspects = defaulted_produced_aspects(produces_aspects_spec);
        let mut graph = self.runtime.graph_mut();
        let mut builder = graph
            .node()
            .on_demand()
            .produces_aspects(aspect_mask_from_list(&produced_aspects));
        if !read_aspects.is_empty() {
            builder = builder.reads_aspects(aspect_mask_from_list(read_aspects));
        }
        let node = builder.build();
        graph
            .set_dependencies(node, dependencies)
            .map_err(WorthSignalJsError::from)?;
        drop(graph);
        Ok(RecipeGraphPublication {
            node,
            produced_aspects,
        })
    }

    fn publish_recipe_catalog(&mut self, id: &str, node: NodeId, produced_aspects: Vec<Aspect>) {
        self.catalog.insert(
            id.to_owned(),
            CatalogEntry {
                node,
                produced_aspects,
            },
        );
        self.nodes_by_id.insert(node, id.to_owned());
    }

    fn publish_recipe_store(
        &mut self,
        id: String,
        definition: StoredRecipeDefinition,
        origin: StoredRecipeOrigin,
    ) -> Result<(), WorthSignalJsError> {
        let mut store = self.lock_store()?;
        store.recipes.insert(
            id,
            StoredRecipe {
                definition,
                origin,
                value: SignalValue::Null,
                version: AspectVersion::zero(),
                initialized: false,
                output_identity: None,
            },
        );
        Ok(())
    }

    pub(super) fn ensure_unique_id(&self, id: &str) -> Result<(), WorthSignalJsError> {
        if self.catalog.contains_key(id) {
            return Err(WorthSignalJsError::invalid_input(format!(
                "signal id `{id}` already exists"
            )));
        }
        Ok(())
    }

    fn ensure_known_reads(&self, reads: &[RecipeReadSpec]) -> Result<(), WorthSignalJsError> {
        for read in reads {
            if !self.catalog.contains_key(read.id()) {
                return Err(WorthSignalJsError::invalid_input(format!(
                    "unknown read `{}`",
                    read.id()
                )));
            }
        }
        Ok(())
    }
}
