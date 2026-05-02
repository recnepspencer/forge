use std::fmt;

use forge_query::facade::{
    ForgeQueryComputedBuilder, ForgeQueryDerivedView, ForgeQueryDerivedViewHandle,
    ForgeQueryLiveView, ForgeQueryLiveViewBuilder, ForgeQueryRuntimeError,
    ForgeQueryWorkspaceLiveViewDeclaration,
};

use super::{
    WorthQueryAspectPath, WorthQueryCollection, WorthQueryLiveField, WorthQuerySchemaBasis,
};

#[derive(Debug)]
pub enum WorthQueryDeclarationError {
    EmptySurfaceName,
    ForgeQuery(ForgeQueryRuntimeError),
}

impl fmt::Display for WorthQueryDeclarationError {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            Self::EmptySurfaceName => {
                write!(
                    f,
                    "worth query declarations require a non-empty surface name"
                )
            }
            Self::ForgeQuery(error) => error.fmt(f),
        }
    }
}

impl std::error::Error for WorthQueryDeclarationError {}

impl From<ForgeQueryRuntimeError> for WorthQueryDeclarationError {
    fn from(value: ForgeQueryRuntimeError) -> Self {
        Self::ForgeQuery(value)
    }
}

pub struct WorthQueryLiveDeclarationBuilder {
    surface_name: String,
    collection: WorthQueryCollection,
    schema_basis: WorthQuerySchemaBasis,
    inner: ForgeQueryLiveViewBuilder,
}

impl WorthQueryLiveDeclarationBuilder {
    pub fn new(
        surface_name: impl Into<String>,
        collection: WorthQueryCollection,
        schema_basis: WorthQuerySchemaBasis,
    ) -> Self {
        let surface_name = surface_name.into();
        Self {
            inner: ForgeQueryLiveViewBuilder::surface(surface_name.clone()),
            surface_name,
            collection,
            schema_basis,
        }
    }

    pub fn table(mut self) -> Self {
        self.inner = self.inner.table();
        self
    }

    pub fn list_splice(mut self) -> Self {
        self.inner = self.inner.list_splice();
        self
    }

    pub fn detail(mut self) -> Self {
        self.inner = self.inner.detail();
        self
    }

    pub fn grouped_by(mut self, aspect: WorthQueryAspectPath) -> Self {
        self.inner = self.inner.grouped_by(aspect.as_str());
        self
    }

    pub fn select(mut self, aspects: impl IntoIterator<Item = WorthQueryAspectPath>) -> Self {
        self.inner = self
            .inner
            .select(aspects.into_iter().map(|aspect| aspect.as_str()));
        self
    }

    pub fn select_fields(mut self, fields: impl IntoIterator<Item = WorthQueryLiveField>) -> Self {
        self.inner = self
            .inner
            .select(fields.into_iter().map(|field| field.delivered_name()));
        self
    }

    pub fn order_by(mut self, aspect: WorthQueryAspectPath) -> Self {
        self.inner = self.inner.order_by(aspect.as_str());
        self
    }

    pub fn order_by_field(mut self, field: WorthQueryLiveField) -> Self {
        self.inner = self.inner.order_by(field.delivered_name());
        self
    }

    pub fn build(
        self,
    ) -> Result<ForgeQueryWorkspaceLiveViewDeclaration, WorthQueryDeclarationError> {
        ensure_surface_name(&self.surface_name)?;
        self.inner
            .from(self.collection.as_str())
            .schema_basis(self.schema_basis.as_str())
            .build()
            .map_err(Into::into)
    }
}

pub struct WorthQueryComputedDeclarationBuilder {
    surface_name: String,
    inner: ForgeQueryComputedBuilder,
}

impl WorthQueryComputedDeclarationBuilder {
    pub fn new(surface_name: impl Into<String>) -> Self {
        let surface_name = surface_name.into();
        Self {
            inner: ForgeQueryComputedBuilder::surface(surface_name.clone()),
            surface_name,
        }
    }

    pub fn reads(mut self, aspects: impl IntoIterator<Item = WorthQueryAspectPath>) -> Self {
        self.inner = self
            .inner
            .reads(aspects.into_iter().map(|aspect| aspect.as_str()));
        self
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = WorthQueryAspectPath>) -> Self {
        self.inner = self
            .inner
            .produces(aspects.into_iter().map(|aspect| aspect.as_str()));
        self
    }

    pub fn depends_on_live<T>(mut self, view: &ForgeQueryLiveView<T>) -> Self {
        self.inner = self.inner.depends_on_live(view);
        self
    }

    pub fn depends_on_computed<T>(mut self, view: &ForgeQueryDerivedViewHandle<T>) -> Self {
        self.inner = self.inner.depends_on_computed(view);
        self
    }

    pub fn whole_refresh_fallback(mut self) -> Self {
        self.inner = self.inner.whole_refresh_fallback();
        self
    }

    pub fn build(self) -> Result<ForgeQueryDerivedView, WorthQueryDeclarationError> {
        ensure_surface_name(&self.surface_name)?;
        self.inner.build().map_err(Into::into)
    }
}

fn ensure_surface_name(surface_name: &str) -> Result<(), WorthQueryDeclarationError> {
    if surface_name.trim().is_empty() {
        return Err(WorthQueryDeclarationError::EmptySurfaceName);
    }
    Ok(())
}
