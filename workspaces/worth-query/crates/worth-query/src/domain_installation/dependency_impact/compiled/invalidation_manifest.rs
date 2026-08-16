use crate::domain_installation::operation_authority_chain::WorthQueryOperationAuthorityBasis;
use std::collections::BTreeMap;

use super::{
    WorthQueryCompiledSemanticAspectDependency, WorthQuerySemanticAspectDependencyView,
    WorthQuerySemanticDependencyRole,
};

#[path = "invalidation_manifest/bound_primary_match.rs"]
mod bound_primary_match;
use bound_primary_match::bound_delivery_matches;
#[cfg(test)]
#[path = "invalidation_manifest/structural_manifest_tests.rs"]
mod structural_manifest_tests;
#[path = "invalidation_manifest/structural_roles.rs"]
mod structural_roles;

#[derive(Debug)]
struct InstalledBridgeImpact {
    dependency: worth_query_installation::facade::WorthQuerySemanticTruthDependency,
    direct_rules: Vec<InstalledDirectImpactRule>,
    structural_membership: bool,
    signal_consequences: Vec<WorthQuerySemanticDependencyRole>,
}

#[derive(Debug)]
struct InstalledDirectImpactRule {
    role: WorthQuerySemanticDependencyRole,
    projection: InstalledProjection,
}

type ConditionalTruthKey = (
    worth_query_installation::facade::WorthQueryConditionalNodeLocation,
    usize,
);

#[derive(Debug)]
enum InstalledProjection {
    WholeAspect,
    Paths(Vec<worth_foundational::facade::CanonicalFieldPath>),
}

#[derive(Debug)]
pub struct WorthQueryInstalledInvalidationManifest {
    pub(crate) affinity: WorthQueryOperationAuthorityBasis,
    dependency_count: usize,
    role_counts: BTreeMap<WorthQuerySemanticDependencyRole, usize>,
    conditional_truth: BTreeMap<ConditionalTruthKey, InstalledBridgeImpact>,
}

impl WorthQueryInstalledInvalidationManifest {
    pub(super) fn compile(
        affinity: &WorthQueryOperationAuthorityBasis,
        dependencies: &[WorthQueryCompiledSemanticAspectDependency],
        impact_index: &super::impact_index::WorthQuerySemanticImpactIndex,
    ) -> Self {
        let mut role_counts = BTreeMap::new();
        let mut conditional_truth = BTreeMap::new();
        for dependency in dependencies {
            *role_counts.entry(dependency.role()).or_insert(0) += 1;
            if let (
                Some(location),
                WorthQuerySemanticAspectDependencyView::ConditionalTruth(truth),
            ) = (dependency.conditional_location(), dependency.source())
            {
                let ordinal = match &dependency.locus {
                    super::dependency_locus::WorthQuerySemanticAspectDependencyLocus::ConditionalTruth {
                        dependency_ordinal,
                        ..
                    } => *dependency_ordinal,
                    _ => continue,
                };
                let truth_key = (location.clone(), ordinal);
                conditional_truth.insert(
                    truth_key,
                    InstalledBridgeImpact {
                        dependency: truth.clone(),
                        direct_rules: direct_impact_rules(truth, dependencies),
                        structural_membership: impact_index.structural_membership(),
                        signal_consequences: impact_index
                            .conditional_consequence_roles(location, ordinal)
                            .unwrap_or_default(),
                    },
                );
            }
        }
        Self {
            affinity: affinity.clone(),
            dependency_count: dependencies.len(),
            role_counts,
            conditional_truth,
        }
    }

    pub fn operation_identity(&self) -> &str {
        &self.affinity.operation_identity
    }

    pub const fn installation_generation(&self) -> u64 {
        self.affinity.installation_generation
    }

    pub const fn dependency_count(&self) -> usize {
        self.dependency_count
    }

    pub fn role_count(&self, role: WorthQuerySemanticDependencyRole) -> usize {
        self.role_counts.get(&role).copied().unwrap_or(0)
    }

    pub fn conditional_truth_count(&self) -> usize {
        self.conditional_truth.len()
    }

    pub(crate) fn admits_bridge_dependency(
        &self,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        candidate: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
    ) -> bool {
        self.conditional_truth
            .get(&(location.clone(), candidate.dependency_ordinal()))
            .is_some_and(|installed| exact_dependency_match(&installed.dependency, candidate))
    }

    pub(crate) fn select_bridge_roles(
        &self,
        location: &worth_query_installation::facade::WorthQueryConditionalNodeLocation,
        candidate: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
        changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
        include_signal_consequences: bool,
    ) -> Option<(Vec<WorthQuerySemanticDependencyRole>, usize)> {
        let installed = self
            .conditional_truth
            .get(&(location.clone(), candidate.dependency_ordinal()))?;
        if !exact_dependency_match(&installed.dependency, candidate) {
            return None;
        }
        let mut roles = Vec::new();
        let mut lookups = 1;
        for delivered_change in changes {
            let Some(change) = delivered_change.semantic_change() else {
                structural_roles::append(
                    installed.structural_membership,
                    candidate,
                    delivered_change,
                    &mut roles,
                );
                continue;
            };
            lookups += 1;
            if !change_matches_dependency(change, &installed.dependency) {
                continue;
            }
            for rule in &installed.direct_rules {
                lookups += 1;
                if rule.projection.matches(change.effective_field_path()) {
                    roles.push(rule.role);
                }
            }
        }
        if include_signal_consequences {
            roles.extend_from_slice(&installed.signal_consequences);
            lookups += 1;
        }
        roles.sort_unstable();
        roles.dedup();
        Some((roles, lookups))
    }

    /// Selects consumer-side roles for a delivery from a separately installed
    /// primary operation. Source operation/location identity is intentionally
    /// not part of this lookup: the primary-runtime seal owns provenance,
    /// while this manifest owns the consumer's semantic dependency match.
    pub(crate) fn select_bound_primary_roles(
        &self,
        consumer_dependencies: &[&worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate],
        delivered: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
        changes: &[worth_runtime_bridge::facade::BridgeDeliveredCorrespondenceChange],
        include_signal_consequences: bool,
        candidate_index_lookups: usize,
    ) -> Option<(Vec<WorthQuerySemanticDependencyRole>, usize)> {
        let mut roles = Vec::new();
        let mut lookups = candidate_index_lookups;
        let mut matched_dependency = false;
        for &consumer in consumer_dependencies {
            let location = crate::domain_installation::conditional_execution::query_location_from_bridge_candidate(consumer);
            let key = (location, consumer.dependency_ordinal());
            let Some(installed) = self.conditional_truth.get(&key) else {
                continue;
            };
            lookups += 1;
            if !exact_dependency_match(&installed.dependency, consumer)
                || !bound_delivery_matches(consumer, delivered, changes)
            {
                continue;
            }
            matched_dependency = true;
            for delivered_change in changes {
                if !bound_primary_match::bound_change_matches(consumer, delivered, delivered_change)
                {
                    continue;
                }
                let Some(change) = delivered_change.semantic_change() else {
                    structural_roles::append(
                        installed.structural_membership,
                        delivered,
                        delivered_change,
                        &mut roles,
                    );
                    continue;
                };
                lookups += 1;
                if !change_matches_dependency(change, &installed.dependency) {
                    continue;
                }
                for rule in &installed.direct_rules {
                    lookups += 1;
                    if rule.projection.matches(change.effective_field_path()) {
                        roles.push(rule.role);
                    }
                }
            }
            if include_signal_consequences {
                roles.extend_from_slice(&installed.signal_consequences);
                lookups += 1;
            }
        }
        if !matched_dependency {
            return None;
        }
        roles.sort_unstable();
        roles.dedup();
        Some((roles, lookups))
    }
}

impl InstalledProjection {
    fn from_mask(
        mask: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
    ) -> Self {
        if mask.is_whole_aspect() {
            Self::WholeAspect
        } else {
            Self::Paths(mask.paths().to_vec())
        }
    }

    fn exact_path(path: &worth_foundational::facade::CanonicalFieldPath) -> Self {
        Self::Paths(vec![path.clone()])
    }

    fn matches(&self, changed: Option<&worth_foundational::facade::CanonicalFieldPath>) -> bool {
        match (self, changed) {
            (Self::WholeAspect, _) | (_, None) => true,
            (Self::Paths(paths), Some(changed)) => {
                paths.iter().any(|path| paths_overlap(path, changed))
            }
        }
    }
}

fn direct_impact_rules(
    truth: &worth_query_installation::facade::WorthQuerySemanticTruthDependency,
    dependencies: &[WorthQueryCompiledSemanticAspectDependency],
) -> Vec<InstalledDirectImpactRule> {
    let mut rules = Vec::new();
    let has_window = dependencies.iter().any(|dependency| {
        matches!(
            dependency.source(),
            WorthQuerySemanticAspectDependencyView::CollectionWindowPolicy(
                worth_query_installation::facade::WorthQueryOperationWindowPolicy::ContinuationBounded
            )
        )
    });
    for dependency in dependencies {
        let projection = match dependency.source() {
            WorthQuerySemanticAspectDependencyView::NativeProjection(projection)
                if projection.contract() == truth.contract()
                    && masks_overlap(projection.mask(), truth.projection_mask()) =>
            {
                Some(InstalledProjection::from_mask(projection.mask()))
            }
            WorthQuerySemanticAspectDependencyView::CollectionField(field)
                if field.aspect_key() == truth.contract().key()
                    && mask_matches_path(truth.projection_mask(), field.field_path()) =>
            {
                Some(InstalledProjection::exact_path(field.field_path()))
            }
            _ => None,
        };
        if let Some(projection) = projection {
            rules.push(InstalledDirectImpactRule {
                role: dependency.role(),
                projection,
            });
            if has_window && dependency.role() == WorthQuerySemanticDependencyRole::Ordering {
                rules.push(InstalledDirectImpactRule {
                    role: WorthQuerySemanticDependencyRole::WindowBoundary,
                    projection: match dependency.source() {
                        WorthQuerySemanticAspectDependencyView::NativeProjection(projection) => {
                            InstalledProjection::from_mask(projection.mask())
                        }
                        WorthQuerySemanticAspectDependencyView::CollectionField(field) => {
                            InstalledProjection::exact_path(field.field_path())
                        }
                        _ => unreachable!("an indexed direct rule has a projection source"),
                    },
                });
            }
        }
    }
    rules
}

fn change_matches_dependency(
    change: &worth_runtime_bridge::facade::BridgeSemanticAspectChange,
    dependency: &worth_query_installation::facade::WorthQuerySemanticTruthDependency,
) -> bool {
    change.aspect_key() == dependency.contract().key()
        && change.aspect_identity() == dependency.contract().identity()
        && change.contract_revision() == dependency.contract().revision()
        && change.binding() == dependency.binding()
        && dependency
            .relevant_changes()
            .iter()
            .copied()
            .any(|kind| change.intersects_relevant_change(kind))
        && change
            .effective_field_path()
            .is_none_or(|path| mask_matches_path(dependency.projection_mask(), path))
}

fn masks_overlap(
    left: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
    right: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
) -> bool {
    left.is_whole_aspect()
        || right.is_whole_aspect()
        || left
            .paths()
            .iter()
            .any(|left| right.paths().iter().any(|right| paths_overlap(left, right)))
}

fn mask_matches_path(
    mask: &worth_foundational::facade::AspectMask<worth_foundational::facade::ProjectionMask>,
    path: &worth_foundational::facade::CanonicalFieldPath,
) -> bool {
    mask.is_whole_aspect()
        || mask
            .paths()
            .iter()
            .any(|candidate| paths_overlap(candidate, path))
}

fn paths_overlap(
    left: &worth_foundational::facade::CanonicalFieldPath,
    right: &worth_foundational::facade::CanonicalFieldPath,
) -> bool {
    let shared = left.fields().len().min(right.fields().len());
    left.fields()[..shared] == right.fields()[..shared]
}

fn exact_dependency_match(
    installed: &worth_query_installation::facade::WorthQuerySemanticTruthDependency,
    candidate: &worth_runtime_bridge::facade::BridgeSemanticDependencyCandidate,
) -> bool {
    installed.contract() == candidate.contract()
        && installed.projection_mask() == candidate.projection_mask()
        && installed.binding() == candidate.binding()
        && installed.relevant_changes() == candidate.relevant_changes()
        && locality_matches(installed.locality(), candidate.locality())
}

fn locality_matches(
    installed: &worth_query_installation::facade::WorthQuerySemanticLocality,
    candidate: &worth_runtime_bridge::facade::BridgeSemanticLocality,
) -> bool {
    use worth_query_installation::facade::WorthQuerySemanticLocality as Query;
    use worth_runtime_bridge::facade::BridgeSemanticLocality as Bridge;
    match (installed, candidate) {
        (Query::SourceRecord, Bridge::SourceRecord | Bridge::ManagedSourceRecord) => true,
        (Query::WholeLogicalGraph, Bridge::WholeLogicalGraph) => true,
        (Query::SourcePartition(left), Bridge::SourcePartition(right)) => {
            left.as_str() == right.as_str()
        }
        _ => false,
    }
}
