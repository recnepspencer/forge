use crate::identity::hash_parts;

use super::artifact::ForgeQueryCanonicalDeclarationArtifact;
use super::async_resource::{
    ForgeQueryAsyncDeclarationClause, ForgeQueryAsyncFailurePosture, ForgeQueryAsyncLoadingPosture,
    ForgeQueryAsyncSourceFamily,
};
use super::input::ForgeQueryDeclarationInput;
use super::temporal::ForgeQueryTemporalDeclarationClause;
use crate::application::ForgeQueryDomainEntryMarker;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryDeclarationFutureProjectionClass {
    Ordinary,
    Temporal,
    AsyncResource,
    TemporalAsync,
}

impl ForgeQueryDeclarationFutureProjectionClass {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Ordinary => "ordinary",
            Self::Temporal => "temporal",
            Self::AsyncResource => "async_resource",
            Self::TemporalAsync => "temporal_async",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryDeclarationFutureProjection {
    class: ForgeQueryDeclarationFutureProjectionClass,
    temporal_families: Vec<String>,
    async_source_families: Vec<ForgeQueryAsyncSourceFamily>,
    async_loading_postures: Vec<ForgeQueryAsyncLoadingPosture>,
    async_failure_postures: Vec<ForgeQueryAsyncFailurePosture>,
    requests_completion_lifecycle: bool,
    projection_digest: String,
}

impl ForgeQueryDeclarationFutureProjection {
    pub(crate) fn from_declaration<D, I>(
        declaration: &ForgeQueryCanonicalDeclarationArtifact<D, I>,
    ) -> Self
    where
        D: ForgeQueryDomainEntryMarker,
        I: ForgeQueryDeclarationInput<D>,
    {
        let temporal_families = declaration
            .temporal_clauses()
            .iter()
            .map(temporal_family_key)
            .collect::<Vec<_>>();
        let mut async_source_families = Vec::new();
        let mut async_loading_postures = Vec::new();
        let mut async_failure_postures = Vec::new();
        let mut requests_completion_lifecycle = false;

        for clause in declaration.async_resource_clauses() {
            match clause {
                ForgeQueryAsyncDeclarationClause::ResourceRequest {
                    source_family,
                    loading_posture,
                    failure_posture,
                    ..
                } => {
                    async_source_families.push(*source_family);
                    async_loading_postures.push(*loading_posture);
                    async_failure_postures.push(*failure_posture);
                }
                ForgeQueryAsyncDeclarationClause::CompletionRequest {
                    source_family,
                    failure_posture,
                    ..
                } => {
                    async_source_families.push(*source_family);
                    async_failure_postures.push(*failure_posture);
                    requests_completion_lifecycle = true;
                }
            }
        }

        async_source_families.sort();
        async_source_families.dedup();
        async_loading_postures.sort();
        async_loading_postures.dedup();
        async_failure_postures.sort();
        async_failure_postures.dedup();

        let class = match (
            !temporal_families.is_empty(),
            !async_source_families.is_empty() || requests_completion_lifecycle,
        ) {
            (false, false) => ForgeQueryDeclarationFutureProjectionClass::Ordinary,
            (true, false) => ForgeQueryDeclarationFutureProjectionClass::Temporal,
            (false, true) => ForgeQueryDeclarationFutureProjectionClass::AsyncResource,
            (true, true) => ForgeQueryDeclarationFutureProjectionClass::TemporalAsync,
        };

        let projection_digest = derive_projection_digest(
            class,
            &temporal_families,
            &async_source_families,
            &async_loading_postures,
            &async_failure_postures,
            requests_completion_lifecycle,
        );

        Self {
            class,
            temporal_families,
            async_source_families,
            async_loading_postures,
            async_failure_postures,
            requests_completion_lifecycle,
            projection_digest,
        }
    }

    pub fn class(&self) -> ForgeQueryDeclarationFutureProjectionClass {
        self.class
    }

    pub fn temporal_families(&self) -> &[String] {
        &self.temporal_families
    }

    pub fn async_source_families(&self) -> &[ForgeQueryAsyncSourceFamily] {
        &self.async_source_families
    }

    pub fn async_loading_postures(&self) -> &[ForgeQueryAsyncLoadingPosture] {
        &self.async_loading_postures
    }

    pub fn async_failure_postures(&self) -> &[ForgeQueryAsyncFailurePosture] {
        &self.async_failure_postures
    }

    pub fn requests_completion_lifecycle(&self) -> bool {
        self.requests_completion_lifecycle
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub fn retained_facts(&self) -> Vec<String> {
        let mut facts = vec![format!("future-projection-class:{}", self.class.as_str())];
        for family in &self.temporal_families {
            facts.push(format!("temporal-clause:{family}"));
        }
        for family in &self.async_source_families {
            facts.push(format!("async-source-family:{}", family.as_str()));
        }
        for posture in &self.async_loading_postures {
            facts.push(format!("async-loading-posture:{}", posture.as_str()));
        }
        for posture in &self.async_failure_postures {
            facts.push(format!("async-failure-posture:{}", posture.as_str()));
        }
        if self.requests_completion_lifecycle {
            facts.push("async-completion-lifecycle:requested".to_string());
        }
        facts
    }
}

fn temporal_family_key(clause: &ForgeQueryTemporalDeclarationClause) -> String {
    match clause {
        ForgeQueryTemporalDeclarationClause::StaleAfter { .. } => "stale-after".to_string(),
        ForgeQueryTemporalDeclarationClause::Interval { .. } => "interval".to_string(),
        ForgeQueryTemporalDeclarationClause::Deadline { .. } => "deadline".to_string(),
        ForgeQueryTemporalDeclarationClause::Window { kind, .. } => kind.as_str().to_string(),
    }
}

fn derive_projection_digest(
    class: ForgeQueryDeclarationFutureProjectionClass,
    temporal_families: &[String],
    async_source_families: &[ForgeQueryAsyncSourceFamily],
    async_loading_postures: &[ForgeQueryAsyncLoadingPosture],
    async_failure_postures: &[ForgeQueryAsyncFailurePosture],
    requests_completion_lifecycle: bool,
) -> String {
    let mut parts = vec![format!("class:{}", class.as_str())];
    for family in temporal_families {
        parts.push(format!("temporal:{family}"));
    }
    for family in async_source_families {
        parts.push(format!("async-source:{}", family.as_str()));
    }
    for posture in async_loading_postures {
        parts.push(format!("async-loading:{}", posture.as_str()));
    }
    for posture in async_failure_postures {
        parts.push(format!("async-failure:{}", posture.as_str()));
    }
    if requests_completion_lifecycle {
        parts.push("async-completion:true".to_string());
    }
    hash_parts(&parts)
}
