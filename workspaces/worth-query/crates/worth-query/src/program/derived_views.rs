use std::collections::BTreeSet;

use crate::runtime::WorthQueryAspectTouch;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryDerivedView {
    name: String,
    dependency_aspects: Vec<WorthQueryAspectTouch>,
    produced_aspects: Vec<WorthQueryAspectTouch>,
    upstream_live_views: Vec<String>,
    upstream_derived_views: Vec<String>,
    incremental: bool,
}

impl WorthQueryDerivedView {
    pub fn new(
        name: impl Into<String>,
        dependency_aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
    ) -> Self {
        Self {
            name: name.into(),
            dependency_aspects: unique_derived_view_aspects(dependency_aspects),
            produced_aspects: Vec::new(),
            upstream_live_views: Vec::new(),
            upstream_derived_views: Vec::new(),
            incremental: true,
        }
    }

    pub fn produces(mut self, aspects: impl IntoIterator<Item = WorthQueryAspectTouch>) -> Self {
        self.produced_aspects = unique_derived_view_aspects(aspects);
        self
    }

    pub fn depends_on_live<T>(mut self, view: &crate::runtime::WorthQueryLiveView<T>) -> Self {
        self.upstream_live_views.push(view.name().to_string());
        self
    }

    pub fn depends_on_derived<T>(
        mut self,
        view: &crate::runtime::WorthQueryDerivedViewHandle<T>,
    ) -> Self {
        self.upstream_derived_views.push(view.name().to_string());
        self
    }

    pub(crate) fn depends_on_live_name_from_workspace_declaration(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.upstream_live_views.push(name.into());
        self
    }

    pub(crate) fn depends_on_derived_name_from_workspace_declaration(
        mut self,
        name: impl Into<String>,
    ) -> Self {
        self.upstream_derived_views.push(name.into());
        self
    }

    pub fn whole_refresh_fallback(mut self) -> Self {
        self.incremental = false;
        self
    }

    pub fn name(&self) -> &str {
        &self.name
    }

    pub fn dependency_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
        &self.dependency_aspects
    }

    pub fn produced_aspect_touches(&self) -> &[WorthQueryAspectTouch] {
        &self.produced_aspects
    }

    pub fn upstream_live_views(&self) -> &[String] {
        &self.upstream_live_views
    }

    pub fn upstream_derived_views(&self) -> &[String] {
        &self.upstream_derived_views
    }

    pub fn incremental(&self) -> bool {
        self.incremental
    }
}

fn unique_derived_view_aspects(
    aspects: impl IntoIterator<Item = WorthQueryAspectTouch>,
) -> Vec<WorthQueryAspectTouch> {
    let mut touches = BTreeSet::new();
    for touch in aspects {
        touches.insert(touch);
    }
    touches.into_iter().collect()
}
