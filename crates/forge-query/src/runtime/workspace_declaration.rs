use std::collections::BTreeSet;

use super::{
    DeclarativeLiveQueryRequest, DeclarativeLiveViewShape, ForgeQueryDerivedViewHandle,
    ForgeQueryEffectCondition, ForgeQueryEffectDeclaration, ForgeQueryEffectTrigger,
    ForgeQueryLiveView, ForgeQueryRuntimeError, QuerySchemaView,
};
use crate::declarative_live::DeclarativeProjectionField;
use crate::memory_workspace::ForgeQueryWorkspaceError;
use crate::program::ForgeQueryDerivedView;
use crate::schema_view::{SchemaFieldKind, SchemaFieldView};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryWorkspaceLiveViewDeclaration {
    request: DeclarativeLiveQueryRequest,
    schema_view: QuerySchemaView,
}

impl ForgeQueryWorkspaceLiveViewDeclaration {
    pub fn from_request(
        request: DeclarativeLiveQueryRequest,
        schema_view: QuerySchemaView,
    ) -> Self {
        Self {
            request,
            schema_view,
        }
    }

    pub fn request(&self) -> &DeclarativeLiveQueryRequest {
        &self.request
    }

    pub fn schema_view(&self) -> &QuerySchemaView {
        &self.schema_view
    }

    pub(in crate::runtime) fn into_parts(self) -> (DeclarativeLiveQueryRequest, QuerySchemaView) {
        (self.request, self.schema_view)
    }
}

pub struct ForgeQueryLiveViewBuilder {
    surface_name: String,
    collection: Option<String>,
    view_shape: DeclarativeLiveViewShape,
    projection: Vec<String>,
    ordering: Option<String>,
    schema_basis_marker: Option<String>,
}

impl ForgeQueryLiveViewBuilder {
    pub(in crate::runtime) fn new(surface_name: impl Into<String>) -> Self {
        Self {
            surface_name: surface_name.into(),
            collection: None,
            view_shape: DeclarativeLiveViewShape::table(),
            projection: Vec::new(),
            ordering: None,
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

    pub fn grouped_by(mut self, aspect: impl Into<String>) -> Self {
        self.view_shape = DeclarativeLiveViewShape::kanban_grouped(aspect);
        self
    }

    pub fn select(mut self, aspects: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.projection.extend(aspects.into_iter().map(Into::into));
        self
    }

    pub fn order_by(mut self, aspect: impl Into<String>) -> Self {
        self.ordering = Some(aspect.into());
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
            projected.push("identity.id".to_string());
        }
        let mut request = DeclarativeLiveQueryRequest::new(collection, self.view_shape);
        let mut schema_fields = BTreeSet::new();
        for aspect in &projected {
            let (section, field) = split_aspect(aspect)?;
            request = request
                .project(DeclarativeProjectionField::new(section, field).delivered_as(aspect));
            schema_fields.insert((section.to_string(), field.to_string()));
        }
        if let Some(ordering) = self.ordering {
            let (section, field) = split_aspect(&ordering)?;
            request = request.order_by(DeclarativeProjectionField::new(section, field));
            schema_fields.insert((section.to_string(), field.to_string()));
        }
        let schema_view = QuerySchemaView::new(
            self.schema_basis_marker.unwrap_or_else(|| {
                format!(
                    "workspace-live-view:{}:{}",
                    self.surface_name,
                    projected.join("|")
                )
            }),
            schema_fields.into_iter().map(|(section, field)| {
                SchemaFieldView::new(section, field, SchemaFieldKind::String)
            }),
            std::iter::empty(),
        );
        Ok(ForgeQueryWorkspaceLiveViewDeclaration::from_request(
            request,
            schema_view,
        ))
    }
}

pub struct ForgeQueryComputedBuilder {
    view_name: String,
    reads: Vec<String>,
    produces: Vec<String>,
    upstream_live_views: Vec<String>,
    upstream_computed_views: Vec<String>,
    incremental: bool,
}

impl ForgeQueryComputedBuilder {
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

    pub fn reads(mut self, aspects: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.reads.extend(aspects.into_iter().map(Into::into));
        self
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = impl Into<String>>) -> Self {
        self.produces.extend(aspects.into_iter().map(Into::into));
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
            view = view.depends_on_live_name(upstream);
        }
        for upstream in self.upstream_computed_views {
            view = view.depends_on_derived_name(upstream);
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
        aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.trigger = Some(ForgeQueryEffectTrigger::live_view(view, aspects));
        self
    }

    pub fn when_computed<T>(
        mut self,
        view: &ForgeQueryDerivedViewHandle<T>,
        aspects: impl IntoIterator<Item = impl Into<String>>,
    ) -> Self {
        self.trigger = Some(ForgeQueryEffectTrigger::computed_view(view, aspects));
        self
    }

    pub fn condition_expression(
        mut self,
        descriptor: impl Into<String>,
        input_aspects: impl IntoIterator<Item = impl Into<String>>,
        output_aspects: impl IntoIterator<Item = impl Into<String>>,
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

fn split_aspect(aspect_path: &str) -> Result<(&str, &str), ForgeQueryRuntimeError> {
    let (aspect, field) = aspect_path.split_once('.').ok_or_else(|| {
        workspace_error(format!(
            "workspace aspect path `{aspect_path}` must use aspect.field form"
        ))
    })?;
    if aspect.trim().is_empty() || field.trim().is_empty() {
        return Err(workspace_error(format!(
            "workspace aspect path `{aspect_path}` must use non-empty aspect.field segments"
        )));
    }
    Ok((aspect, field))
}

fn non_empty(value: Option<String>, message: &str) -> Result<String, ForgeQueryRuntimeError> {
    let value = value.ok_or_else(|| workspace_error(message))?;
    if value.trim().is_empty() {
        return Err(workspace_error(message));
    }
    Ok(value)
}

fn workspace_error(message: impl Into<String>) -> ForgeQueryRuntimeError {
    ForgeQueryRuntimeError::Workspace(ForgeQueryWorkspaceError::new(message))
}
