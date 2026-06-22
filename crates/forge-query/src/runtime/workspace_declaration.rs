use std::collections::{BTreeMap, BTreeSet};

#[path = "workspace_declaration_schema.rs"]
mod workspace_declaration_schema;
#[path = "workspace_live_view_declaration.rs"]
mod workspace_live_view_declaration;

use super::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, ForgeQueryDerivedViewHandle,
    ForgeQueryEffectCondition, ForgeQueryEffectDeclaration, ForgeQueryEffectTrigger,
    ForgeQueryLiveView, ForgeQueryRuntimeError, QuerySchemaView,
};
use crate::authoring::{AspectFieldKey, TraversalSelector};
use crate::declarative_live::DeclarativeProjectionField;
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::program::ForgeQueryDerivedView;
use crate::runtime::ForgeQueryAspectTouch;
use crate::schema_view::SchemaFieldKind;
use forge_foundational::facade::AspectKey;
use workspace_declaration_schema::{schema_field_view, schema_relation_view};
pub use workspace_live_view_declaration::ForgeQueryWorkspaceLiveViewDeclaration;

pub struct ForgeQueryLiveViewBuilder {
    surface_name: String,
    collection: Option<String>,
    view_shape: DeclarativeLiveViewShape,
    projection: Vec<AspectFieldKey>,
    ordering: Option<AspectFieldKey>,
    schema_relations: Vec<(String, u8)>,
    schema_basis_marker: Option<String>,
}

impl ForgeQueryLiveViewBuilder {
    pub fn surface(surface_name: impl Into<String>) -> Self {
        Self::new(surface_name)
    }

    pub(in crate::runtime) fn new(surface_name: impl Into<String>) -> Self {
        Self {
            surface_name: surface_name.into(),
            collection: None,
            view_shape: DeclarativeLiveViewShape::table(),
            projection: Vec::new(),
            ordering: None,
            schema_relations: Vec::new(),
            schema_basis_marker: None,
        }
    }

    pub fn from(mut self, collection: impl Into<String>) -> Self {
        self.collection = Some(collection.into());
        self
    }

    pub fn table(mut self) -> Self {
        self.view_shape = DeclarativeLiveViewShape::table();
        self
    }

    pub fn list_splice(mut self) -> Self {
        self.view_shape = DeclarativeLiveViewShape::list_splice();
        self
    }

    pub fn detail(mut self) -> Self {
        self.view_shape = DeclarativeLiveViewShape::detail();
        self
    }

    pub fn grouped_by(mut self, aspect: AspectKey) -> Self {
        self.view_shape = DeclarativeLiveViewShape::kanban_grouped(aspect);
        self
    }

    pub fn select(mut self, fields: impl IntoIterator<Item = AspectFieldKey>) -> Self {
        self.projection.extend(fields);
        self
    }

    pub fn order_by(mut self, field: AspectFieldKey) -> Self {
        self.ordering = Some(field);
        self
    }

    pub fn allow_traversal_relation(mut self, relation: impl Into<String>, max_depth: u8) -> Self {
        self.schema_relations.push((relation.into(), max_depth));
        self
    }

    pub fn schema_basis(mut self, basis_marker: impl Into<String>) -> Self {
        self.schema_basis_marker = Some(basis_marker.into());
        self
    }

    pub fn as_surface(mut self, label: impl Into<String>) -> Self {
        self.surface_name = label.into();
        self
    }

    pub fn build(self) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, ForgeQueryRuntimeError> {
        let collection = non_empty(
            self.collection,
            "workspace live_view builder requires a non-empty collection via from(...)",
        )?;
        let mut projected = self.projection;
        if projected.is_empty() {
            projected.push(
                AspectFieldKey::new("identity", "id")
                    .map_err(|error| workspace_error(format!("{error:?}")))?,
            );
        }
        let mut request = DeclarativeLiveQueryRequest::new(collection, self.view_shape);
        let mut schema_fields = BTreeSet::new();
        for aspect in &projected {
            let section = aspect.aspect().as_str();
            let field = aspect.field().as_str();
            let delivered_name = terminal_aspect_field_key_projection(aspect);
            request = request.project(
                DeclarativeProjectionField::from_authoring_parts(section, field)
                    .delivered_as(delivered_name),
            );
            schema_fields.insert((section.to_string(), field.to_string()));
        }
        if let Some(ordering) = self.ordering {
            let section = ordering.aspect().as_str();
            let field = ordering.field().as_str();
            request = request.order_by(DeclarativeProjectionField::from_authoring_parts(
                section, field,
            ));
            schema_fields.insert((section.to_string(), field.to_string()));
        }
        let schema_relations = normalize_schema_relations(self.schema_relations)?;
        for (relation, max_depth) in &schema_relations {
            request = request.traverse(
                TraversalSelector::bounded(relation.clone(), *max_depth)
                    .map_err(|error| workspace_error(format!("{error:?}")))?,
            );
        }
        let schema_field_views = schema_fields
            .into_iter()
            .map(|(section, field)| schema_field_view(section, field, SchemaFieldKind::String))
            .collect::<Result<Vec<_>, ForgeQueryRuntimeError>>()?;
        let schema_relation_views = schema_relations
            .into_iter()
            .map(|(relation, max_depth)| schema_relation_view(relation, max_depth))
            .collect::<Result<Vec<_>, ForgeQueryRuntimeError>>()?;
        let schema_view = QuerySchemaView::new(
            self.schema_basis_marker.unwrap_or_else(|| {
                format!(
                    "workspace-live-view:{}:{}",
                    self.surface_name,
                    projected
                        .iter()
                        .map(terminal_aspect_field_key_projection)
                        .collect::<Vec<_>>()
                        .join("|")
                )
            }),
            schema_field_views,
            schema_relation_views,
        );
        Ok(ForgeQueryWorkspaceLiveViewDeclaration::from_request(
            request,
            schema_view,
        ))
    }
}

pub struct ForgeQueryComputedBuilder {
    view_name: String,
    reads: Vec<ForgeQueryAspectTouch>,
    produces: Vec<ForgeQueryAspectTouch>,
    upstream_live_views: Vec<String>,
    upstream_computed_views: Vec<String>,
    incremental: bool,
}

impl ForgeQueryComputedBuilder {
    pub fn surface(view_name: impl Into<String>) -> Self {
        Self::new(view_name)
    }

    pub(in crate::runtime) fn new(view_name: impl Into<String>) -> Self {
        Self {
            view_name: view_name.into(),
            reads: Vec::new(),
            produces: Vec::new(),
            upstream_live_views: Vec::new(),
            upstream_computed_views: Vec::new(),
            incremental: true,
        }
    }

    pub fn reads(mut self, aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>) -> Self {
        self.reads.extend(aspects);
        self
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>) -> Self {
        self.produces.extend(aspects);
        self
    }

    pub fn depends_on_live<T>(mut self, view: &ForgeQueryLiveView<T>) -> Self {
        self.upstream_live_views.push(view.name().to_string());
        self
    }

    pub fn depends_on_computed<T>(mut self, view: &ForgeQueryDerivedViewHandle<T>) -> Self {
        self.upstream_computed_views.push(view.name().to_string());
        self
    }

    pub fn whole_refresh_fallback(mut self) -> Self {
        self.incremental = false;
        self
    }

    pub fn build(self) -> Result<ForgeQueryDerivedView, ForgeQueryRuntimeError> {
        let mut view =
            ForgeQueryDerivedView::new(self.view_name, self.reads).produces(self.produces);
        for upstream in self.upstream_live_views {
            view = view.depends_on_live_name_from_workspace_declaration(upstream);
        }
        for upstream in self.upstream_computed_views {
            view = view.depends_on_derived_name_from_workspace_declaration(upstream);
        }
        if !self.incremental {
            view = view.whole_refresh_fallback();
        }
        Ok(view)
    }
}

pub struct ForgeQueryEffectBuilder {
    effect_name: String,
    trigger: Option<ForgeQueryEffectTrigger>,
    condition: ForgeQueryEffectCondition,
    action: Option<ForgeQueryEffectBuilderAction>,
    meaningful_change_suppression: bool,
}

enum ForgeQueryEffectBuilderAction {
    Deliver(String),
    WriteIntent(String),
}

impl ForgeQueryEffectBuilder {
    pub(in crate::runtime) fn new(effect_name: impl Into<String>) -> Self {
        Self {
            effect_name: effect_name.into(),
            trigger: None,
            condition: ForgeQueryEffectCondition::always(),
            action: None,
            meaningful_change_suppression: false,
        }
    }

    pub fn when_live<T>(
        mut self,
        view: &ForgeQueryLiveView<T>,
        aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        self.trigger = Some(ForgeQueryEffectTrigger::live_view(view, aspects));
        self
    }

    pub fn when_computed<T>(
        mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
        aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        self.trigger = Some(ForgeQueryEffectTrigger::computed_view(view, aspects));
        self
    }

    pub fn condition_expression(
        mut self,
        descriptor: impl Into<String>,
        input_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
        output_aspects: impl IntoIterator<Item = ForgeQueryAspectTouch>,
    ) -> Self {
        self.condition =
            ForgeQueryEffectCondition::expression(descriptor, input_aspects, output_aspects);
        self
    }

    pub fn deliver(mut self, target: impl Into<String>) -> Self {
        self.action = Some(ForgeQueryEffectBuilderAction::Deliver(target.into()));
        self
    }

    pub fn write_intent(mut self, strategy: impl Into<String>) -> Self {
        self.action = Some(ForgeQueryEffectBuilderAction::WriteIntent(strategy.into()));
        self
    }

    pub fn meaningful_change_suppression(mut self) -> Self {
        self.meaningful_change_suppression = true;
        self
    }

    pub fn build(self) -> Result<ForgeQueryEffectDeclaration, ForgeQueryRuntimeError> {
        let trigger = self.trigger.ok_or_else(|| {
            workspace_error("workspace effect builder requires a trigger via when_live(...) or when_computed(...)")
        })?;
        let action = self.action.ok_or_else(|| {
            workspace_error("workspace effect builder requires deliver(...) or write_intent(...)")
        })?;
        let declaration = match action {
            ForgeQueryEffectBuilderAction::Deliver(target) => {
                ForgeQueryEffectDeclaration::deliver(self.effect_name, trigger, target)
            }
            ForgeQueryEffectBuilderAction::WriteIntent(strategy) => {
                ForgeQueryEffectDeclaration::write_intent(self.effect_name, trigger, strategy)
            }
        }
        .with_condition(self.condition);
        Ok(if self.meaningful_change_suppression {
            declaration.with_meaningful_change_suppression()
        } else {
            declaration
        })
    }
}

fn terminal_aspect_field_key_projection(field: &AspectFieldKey) -> String {
    format!("{}.{}", field.aspect().as_str(), field.field().as_str())
}

fn non_empty(value: Option<String>, message: &str) -> Result<String, ForgeQueryRuntimeError> {
    let value = value.ok_or_else(|| workspace_error(message))?;
    if value.trim().is_empty() {
        return Err(workspace_error(message));
    }
    Ok(value)
}

fn normalize_schema_relations(
    relations: Vec<(String, u8)>,
) -> Result<Vec<(String, u8)>, ForgeQueryRuntimeError> {
    let mut declared = BTreeMap::<String, u8>::new();
    for (relation, max_depth) in relations {
        if max_depth == 0 {
            return Err(workspace_error(format!(
                "workspace traversal relation `{relation}` must declare a non-zero max depth"
            )));
        }
        if declared.insert(relation.clone(), max_depth).is_some() {
            return Err(workspace_error(format!(
                "workspace traversal relation `{relation}` may only be declared once per live view"
            )));
        }
    }
    Ok(declared.into_iter().collect())
}

fn workspace_error(message: impl Into<String>) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(message))
}

#[cfg(test)]
#[path = "workspace_declaration_tests.rs"]
mod tests;
