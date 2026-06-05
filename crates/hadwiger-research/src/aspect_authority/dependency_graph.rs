use std::collections::{BTreeMap, BTreeSet};

use super::aspect_kinds::{
    require_non_empty, HadwigerAspectAuthorityError, HadwigerAspectKind, HadwigerAspectPosture,
};
use super::aspect_records::HadwigerAspectRecord;
use super::closure_reports::{HadwigerDependencyClosureBlocker, HadwigerDependencyClosureReport};
use super::dependency_edges::{
    HadwigerAspectDependencyEdge, HadwigerAspectDependencyRole, HadwigerAspectInvalidationScope,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AspectDependencyGraph {
    graph_id: String,
    aspects: BTreeMap<HadwigerAspectKind, HadwigerAspectRecord>,
    edges: BTreeSet<HadwigerAspectDependencyEdge>,
}

impl AspectDependencyGraph {
    pub fn builder(graph_id: impl Into<String>) -> AspectDependencyGraphBuilder {
        AspectDependencyGraphBuilder::new(graph_id)
    }

    pub fn graph_id(&self) -> &str {
        &self.graph_id
    }

    pub fn aspects(&self) -> &BTreeMap<HadwigerAspectKind, HadwigerAspectRecord> {
        &self.aspects
    }

    pub fn edges(&self) -> &BTreeSet<HadwigerAspectDependencyEdge> {
        &self.edges
    }

    pub fn evaluate_closure(
        &self,
        root_aspect: HadwigerAspectKind,
    ) -> HadwigerDependencyClosureReport {
        let mut required = BTreeSet::new();
        let mut present = BTreeSet::new();
        let mut present_tokens = BTreeSet::new();
        let mut blockers = Vec::new();
        let mut edge_scopes = Vec::new();
        let mut visited = BTreeSet::new();
        self.visit_closure(
            root_aspect,
            &mut required,
            &mut present,
            &mut present_tokens,
            &mut blockers,
            &mut edge_scopes,
            &mut visited,
        );
        HadwigerDependencyClosureReport::new(
            self.graph_id.clone(),
            root_aspect,
            required.into_iter().collect(),
            present.into_iter().collect(),
            present_tokens.into_iter().collect(),
            blockers,
            &edge_scopes,
        )
    }

    fn visit_closure(
        &self,
        aspect_kind: HadwigerAspectKind,
        required: &mut BTreeSet<HadwigerAspectKind>,
        present: &mut BTreeSet<HadwigerAspectKind>,
        present_tokens: &mut BTreeSet<String>,
        blockers: &mut Vec<HadwigerDependencyClosureBlocker>,
        edge_scopes: &mut Vec<HadwigerAspectInvalidationScope>,
        visited: &mut BTreeSet<HadwigerAspectKind>,
    ) {
        if !visited.insert(aspect_kind) {
            return;
        }
        for edge in self
            .edges
            .iter()
            .filter(|edge| edge.required_by() == aspect_kind)
        {
            required.insert(edge.required_aspect());
            edge_scopes.push(edge.invalidation_scope());
            match self.aspects.get(&edge.required_aspect()) {
                Some(record) => {
                    present.insert(record.aspect_kind());
                    let record_token = record.stable_token();
                    present_tokens.insert(record_token.clone());
                    if !edge.is_satisfied_by(record.aspect_posture()) {
                        blockers.push(HadwigerDependencyClosureBlocker::new(
                            record.aspect_kind(),
                            record.aspect_posture(),
                            Some(edge.clone()),
                            Some(record_token),
                            "required aspect posture does not satisfy dependency",
                        ));
                    }
                }
                None => blockers.push(HadwigerDependencyClosureBlocker::new(
                    edge.required_aspect(),
                    HadwigerAspectPosture::Missing,
                    Some(edge.clone()),
                    None,
                    "required aspect is absent from dependency graph",
                )),
            }
            self.visit_closure(
                edge.required_aspect(),
                required,
                present,
                present_tokens,
                blockers,
                edge_scopes,
                visited,
            );
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct AspectDependencyGraphBuilder {
    graph_id: String,
    aspects: BTreeMap<HadwigerAspectKind, HadwigerAspectRecord>,
    edges: BTreeSet<HadwigerAspectDependencyEdge>,
}

impl AspectDependencyGraphBuilder {
    fn new(graph_id: impl Into<String>) -> Self {
        Self {
            graph_id: graph_id.into(),
            aspects: BTreeMap::new(),
            edges: BTreeSet::new(),
        }
    }

    pub fn with_aspect(
        mut self,
        aspect: impl Into<HadwigerAspectRecord>,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        let aspect = aspect.into();
        let aspect_kind = aspect.aspect_kind();
        if self.aspects.insert(aspect_kind, aspect).is_some() {
            return Err(HadwigerAspectAuthorityError::DuplicateAspect { aspect_kind });
        }
        Ok(self)
    }

    pub fn requires(
        self,
        required_by: HadwigerAspectKind,
        required_aspect: HadwigerAspectKind,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        self.with_dependency(
            required_by,
            required_aspect,
            HadwigerAspectDependencyRole::MathematicalRequirement,
            HadwigerAspectInvalidationScope::ConservativeEscalation,
        )
    }

    pub fn with_dependency(
        mut self,
        required_by: HadwigerAspectKind,
        required_aspect: HadwigerAspectKind,
        dependency_role: HadwigerAspectDependencyRole,
        invalidation_scope: HadwigerAspectInvalidationScope,
    ) -> Result<Self, HadwigerAspectAuthorityError> {
        let edge = HadwigerAspectDependencyEdge::new(
            required_by,
            required_aspect,
            dependency_role,
            invalidation_scope,
        )?;
        if !self.edges.insert(edge) {
            return Err(HadwigerAspectAuthorityError::DuplicateDependency {
                required_by,
                required: required_aspect,
            });
        }
        Ok(self)
    }

    pub fn finish(self) -> Result<AspectDependencyGraph, HadwigerAspectAuthorityError> {
        let graph_id = require_non_empty(self.graph_id, "aspect_dependency_graph_id")?;
        if self.aspects.is_empty() && self.edges.is_empty() {
            return Err(HadwigerAspectAuthorityError::EmptyClosureGraph);
        }
        reject_cycles(&self.edges)?;
        Ok(AspectDependencyGraph {
            graph_id,
            aspects: self.aspects,
            edges: self.edges,
        })
    }
}

fn reject_cycles(
    edges: &BTreeSet<HadwigerAspectDependencyEdge>,
) -> Result<(), HadwigerAspectAuthorityError> {
    for edge in edges {
        let mut visited = BTreeSet::new();
        if reaches(
            edge.required_aspect(),
            edge.required_by(),
            edges,
            &mut visited,
        ) {
            return Err(HadwigerAspectAuthorityError::CyclicDependency {
                aspect_kind: edge.required_by(),
            });
        }
    }
    Ok(())
}

fn reaches(
    current: HadwigerAspectKind,
    target: HadwigerAspectKind,
    edges: &BTreeSet<HadwigerAspectDependencyEdge>,
    visited: &mut BTreeSet<HadwigerAspectKind>,
) -> bool {
    if current == target {
        return true;
    }
    if !visited.insert(current) {
        return false;
    }
    edges
        .iter()
        .filter(|edge| edge.required_by() == current)
        .any(|edge| reaches(edge.required_aspect(), target, edges, visited))
}
