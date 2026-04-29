use super::*;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(in crate::runtime) enum ForgeQueryComputedAdmissionError {
    MissingUpstreamLive { upstream: String },
    MissingUpstreamComputed { upstream: String },
    SelfDependency,
    Cycle { upstream: String },
}

impl ForgeQueryComputedAdmissionError {
    pub(in crate::runtime) fn message(&self) -> String {
        match self {
            Self::MissingUpstreamLive { upstream } => {
                format!("live upstream '{upstream}' is not declared")
            }
            Self::MissingUpstreamComputed { upstream } => {
                format!("computed upstream '{upstream}' is not declared")
            }
            Self::SelfDependency => "computed declaration may not depend on itself".to_string(),
            Self::Cycle { upstream } => {
                format!("computed declaration would create a cycle through '{upstream}'")
            }
        }
    }
}

pub(in crate::runtime) struct ForgeQueryDerivedViewRuntime {
    pub(in crate::runtime) declaration: ForgeQueryDerivedView,
    pub(in crate::runtime) patches: Vec<ForgeQueryDerivedPatch>,
    pub(in crate::runtime) materialization: ForgeQueryDerivedViewMaterialization,
    pub(in crate::runtime) maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
}

impl ForgeQueryDerivedViewRuntime {
    pub(in crate::runtime) fn new(
        declaration: ForgeQueryDerivedView,
        maintainer: Option<Box<dyn ForgeQueryDerivedViewMaintainer>>,
    ) -> Self {
        Self {
            declaration,
            patches: Vec::new(),
            materialization: ForgeQueryDerivedViewMaterialization::default(),
            maintainer,
        }
    }
}

#[derive(Default)]
pub(in crate::runtime) struct ForgeQueryComputedDependencyIndex {
    live_to_computed: BTreeMap<String, BTreeSet<String>>,
    computed_to_dependents: BTreeMap<String, BTreeSet<String>>,
    unscoped_authoritative_computed: BTreeSet<String>,
}

impl ForgeQueryComputedDependencyIndex {
    pub(in crate::runtime) fn register(&mut self, declaration: &ForgeQueryDerivedView) {
        self.unregister(declaration.name());
        let view_name = declaration.name().to_string();
        if declaration.upstream_live_views().is_empty()
            && declaration.upstream_derived_views().is_empty()
        {
            self.unscoped_authoritative_computed
                .insert(view_name.clone());
        }
        for live_view in declaration.upstream_live_views() {
            self.live_to_computed
                .entry(live_view.clone())
                .or_default()
                .insert(view_name.clone());
        }
        for upstream_computed in declaration.upstream_derived_views() {
            self.computed_to_dependents
                .entry(upstream_computed.clone())
                .or_default()
                .insert(view_name.clone());
        }
    }

    fn unregister(&mut self, view_name: &str) {
        self.unscoped_authoritative_computed.remove(view_name);
        remove_from_index(&mut self.live_to_computed, view_name);
        remove_from_index(&mut self.computed_to_dependents, view_name);
    }

    pub(in crate::runtime) fn live_candidates(
        &self,
        live_view_names: impl IntoIterator<Item = String>,
    ) -> BTreeSet<String> {
        let mut candidates = self.unscoped_authoritative_computed.clone();
        for live_view_name in live_view_names {
            if let Some(computed_views) = self.live_to_computed.get(&live_view_name) {
                candidates.extend(computed_views.iter().cloned());
            }
        }
        candidates
    }

    pub(in crate::runtime) fn dependents(
        &self,
        computed_view_name: &str,
    ) -> impl Iterator<Item = String> + '_ {
        self.computed_to_dependents
            .get(computed_view_name)
            .into_iter()
            .flatten()
            .cloned()
    }
}

fn remove_from_index(index: &mut BTreeMap<String, BTreeSet<String>>, view_name: &str) {
    let empty_keys: Vec<String> = index
        .iter_mut()
        .filter_map(|(key, values)| {
            values.remove(view_name);
            values.is_empty().then(|| key.clone())
        })
        .collect();
    for key in empty_keys {
        index.remove(&key);
    }
}
