use serde::{Deserialize, Serialize};

use super::{
    DiagnosticsArtifactKind, DiagnosticsDeliveryClass, DiagnosticsScope, RelationalArtifactPolicy,
    RelationalDiagnosticArtifact,
};

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct RelationalDiagnosticsProfile {
    pub capture_failures: bool,
    pub capture_rollbacks: bool,
    pub capture_comparisons: bool,
    pub detailed_traces_enabled: bool,
    pub collect_all_invariant_failures: bool,
    pub max_entries_per_artifact: usize,
    pub allow_deferred_hot_artifacts: bool,
    pub allow_reconstructable_hot_artifacts: bool,
}

impl Default for RelationalDiagnosticsProfile {
    fn default() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: true,
            detailed_traces_enabled: false,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 256,
            allow_deferred_hot_artifacts: true,
            allow_reconstructable_hot_artifacts: true,
        }
    }
}

impl RelationalDiagnosticsProfile {
    pub fn geometry_operational_hot_path() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: false,
            detailed_traces_enabled: false,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 64,
            allow_deferred_hot_artifacts: false,
            allow_reconstructable_hot_artifacts: false,
        }
    }

    pub fn geometry_rich_certification() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: true,
            detailed_traces_enabled: true,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 768,
            allow_deferred_hot_artifacts: false,
            allow_reconstructable_hot_artifacts: false,
        }
    }

    pub fn chip_operational_hot_path() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: false,
            detailed_traces_enabled: false,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 48,
            allow_deferred_hot_artifacts: false,
            allow_reconstructable_hot_artifacts: false,
        }
    }

    pub fn chip_rich_certification() -> Self {
        Self {
            capture_failures: true,
            capture_rollbacks: true,
            capture_comparisons: true,
            detailed_traces_enabled: true,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 256,
            allow_deferred_hot_artifacts: true,
            allow_reconstructable_hot_artifacts: true,
        }
    }

    fn scope_is_hot(scope: DiagnosticsScope) -> bool {
        matches!(
            scope,
            DiagnosticsScope::Transaction
                | DiagnosticsScope::Snapshot
                | DiagnosticsScope::PatchPublication
                | DiagnosticsScope::QueryPlanning
                | DiagnosticsScope::Invariant
        )
    }

    pub fn artifact_policy(
        &self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
    ) -> RelationalArtifactPolicy {
        let delivery_class = match kind {
            DiagnosticsArtifactKind::MinimalSummary => match scope {
                DiagnosticsScope::Replay => DiagnosticsDeliveryClass::ReconstructableFromReplay,
                _ => DiagnosticsDeliveryClass::MustBeHot,
            },
            DiagnosticsArtifactKind::DetailedTrace => DiagnosticsDeliveryClass::CanDefer,
            DiagnosticsArtifactKind::Failure => DiagnosticsDeliveryClass::MustBeHot,
            DiagnosticsArtifactKind::Rollback => DiagnosticsDeliveryClass::MustBeHot,
            DiagnosticsArtifactKind::Comparison => {
                DiagnosticsDeliveryClass::ReconstructableFromReplay
            }
        };

        let mut enabled = match kind {
            DiagnosticsArtifactKind::MinimalSummary => true,
            DiagnosticsArtifactKind::DetailedTrace => self.detailed_traces_enabled,
            DiagnosticsArtifactKind::Failure => self.capture_failures,
            DiagnosticsArtifactKind::Rollback => self.capture_rollbacks,
            DiagnosticsArtifactKind::Comparison => self.capture_comparisons,
        };

        let mut max_entries = match delivery_class {
            DiagnosticsDeliveryClass::MustBeHot => self.max_entries_per_artifact.max(1),
            DiagnosticsDeliveryClass::CanDefer
            | DiagnosticsDeliveryClass::ReconstructableFromReplay => self.max_entries_per_artifact,
        };

        if Self::scope_is_hot(scope) {
            if matches!(delivery_class, DiagnosticsDeliveryClass::CanDefer)
                && !self.allow_deferred_hot_artifacts
            {
                enabled = false;
                max_entries = 0;
            }
            if matches!(
                delivery_class,
                DiagnosticsDeliveryClass::ReconstructableFromReplay
            ) && !self.allow_reconstructable_hot_artifacts
            {
                enabled = false;
                max_entries = 0;
            }
        }

        match kind {
            DiagnosticsArtifactKind::MinimalSummary => {}
            DiagnosticsArtifactKind::DetailedTrace => {
                if !self.detailed_traces_enabled {
                    enabled = false;
                    max_entries = 0;
                }
            }
            DiagnosticsArtifactKind::Failure => {
                if !self.capture_failures {
                    enabled = false;
                    max_entries = 0;
                }
            }
            DiagnosticsArtifactKind::Rollback => {
                if !self.capture_rollbacks {
                    enabled = false;
                    max_entries = 0;
                }
            }
            DiagnosticsArtifactKind::Comparison => {
                if !self.capture_comparisons {
                    enabled = false;
                    max_entries = 0;
                } else if !self.detailed_traces_enabled {
                    max_entries = max_entries.min(64);
                }
            }
        }

        if !enabled {
            max_entries = 0;
        }

        RelationalArtifactPolicy {
            delivery_class,
            enabled,
            max_entries,
        }
    }

    pub fn should_capture_artifact(
        &self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
    ) -> bool {
        self.artifact_policy(scope, kind).enabled
    }

    pub fn delivery_class(
        &self,
        scope: DiagnosticsScope,
        kind: DiagnosticsArtifactKind,
    ) -> DiagnosticsDeliveryClass {
        self.artifact_policy(scope, kind).delivery_class
    }

    pub fn max_entries_for(&self, scope: DiagnosticsScope, kind: DiagnosticsArtifactKind) -> usize {
        self.artifact_policy(scope, kind).max_entries
    }

    pub fn filter_artifact(
        &self,
        artifact: RelationalDiagnosticArtifact,
    ) -> Option<RelationalDiagnosticArtifact> {
        let mut artifact = artifact.canonicalized();
        let policy = self.artifact_policy(artifact.scope, artifact.kind);
        if !policy.enabled {
            return None;
        }
        artifact.entries.truncate(policy.max_entries);
        if artifact.entries.is_empty() {
            None
        } else {
            Some(artifact)
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::diagnostics::data::{
        DeterminismExpectation, DiagnosticCode, RelationalDiagnosticFields,
        RelationalDiagnosticValue, RelationalDiagnosticsEntry,
    };

    #[test]
    fn diagnostics_profile_classifies_delivery_classes_explicitly() {
        let profile = RelationalDiagnosticsProfile::default();

        assert_eq!(
            profile.delivery_class(
                DiagnosticsScope::Transaction,
                DiagnosticsArtifactKind::MinimalSummary,
            ),
            DiagnosticsDeliveryClass::MustBeHot
        );
        assert_eq!(
            profile.delivery_class(
                DiagnosticsScope::History,
                DiagnosticsArtifactKind::DetailedTrace,
            ),
            DiagnosticsDeliveryClass::CanDefer
        );
        assert_eq!(
            profile.delivery_class(
                DiagnosticsScope::Replay,
                DiagnosticsArtifactKind::Comparison,
            ),
            DiagnosticsDeliveryClass::ReconstructableFromReplay
        );
    }

    #[test]
    fn diagnostics_profile_suppresses_optional_artifacts_when_disabled() {
        let profile = RelationalDiagnosticsProfile {
            capture_failures: false,
            capture_rollbacks: false,
            capture_comparisons: false,
            detailed_traces_enabled: false,
            collect_all_invariant_failures: false,
            max_entries_per_artifact: 0,
            allow_deferred_hot_artifacts: false,
            allow_reconstructable_hot_artifacts: false,
        };

        assert!(profile.should_capture_artifact(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::MinimalSummary,
        ));
        assert!(!profile.should_capture_artifact(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::DetailedTrace,
        ));
        assert!(!profile.should_capture_artifact(
            DiagnosticsScope::Replay,
            DiagnosticsArtifactKind::Comparison,
        ));
        assert!(!profile.should_capture_artifact(
            DiagnosticsScope::Invariant,
            DiagnosticsArtifactKind::Failure,
        ));
        assert_eq!(
            profile.max_entries_for(
                DiagnosticsScope::Transaction,
                DiagnosticsArtifactKind::MinimalSummary,
            ),
            1
        );
    }

    #[test]
    fn diagnostics_profile_named_policies_match_geometry_and_chip_intent() {
        let geometry_hot = RelationalDiagnosticsProfile::geometry_operational_hot_path();
        let geometry_rich = RelationalDiagnosticsProfile::geometry_rich_certification();
        let chip_hot = RelationalDiagnosticsProfile::chip_operational_hot_path();
        let chip_rich = RelationalDiagnosticsProfile::chip_rich_certification();

        assert!(!geometry_hot.detailed_traces_enabled);
        assert!(geometry_rich.detailed_traces_enabled);
        assert!(!geometry_hot.capture_comparisons);
        assert!(geometry_rich.capture_comparisons);
        assert!(geometry_rich.max_entries_per_artifact > geometry_hot.max_entries_per_artifact);
        assert!(!geometry_hot.allow_deferred_hot_artifacts);
        assert!(!geometry_rich.allow_deferred_hot_artifacts);
        assert!(!geometry_rich.allow_reconstructable_hot_artifacts);

        assert!(!chip_hot.detailed_traces_enabled);
        assert!(chip_rich.detailed_traces_enabled);
        assert!(!chip_hot.capture_comparisons);
        assert!(chip_rich.capture_comparisons);
        assert!(chip_rich.max_entries_per_artifact > chip_hot.max_entries_per_artifact);
        assert!(chip_rich.allow_deferred_hot_artifacts);
        assert!(chip_rich.allow_reconstructable_hot_artifacts);
    }

    #[test]
    fn diagnostics_profile_exposes_explicit_artifact_policy_table() {
        let geometry_hot = RelationalDiagnosticsProfile::geometry_operational_hot_path();
        let geometry_rich = RelationalDiagnosticsProfile::geometry_rich_certification();

        let hot_trace = geometry_hot.artifact_policy(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::DetailedTrace,
        );
        assert_eq!(hot_trace.delivery_class, DiagnosticsDeliveryClass::CanDefer);
        assert!(!hot_trace.enabled);
        assert_eq!(hot_trace.max_entries, 0);

        let hot_summary = geometry_hot.artifact_policy(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::MinimalSummary,
        );
        assert_eq!(
            hot_summary.delivery_class,
            DiagnosticsDeliveryClass::MustBeHot
        );
        assert!(hot_summary.enabled);
        assert_eq!(hot_summary.max_entries, 64);

        let rich_comparison = geometry_rich.artifact_policy(
            DiagnosticsScope::Replay,
            DiagnosticsArtifactKind::Comparison,
        );
        assert_eq!(
            rich_comparison.delivery_class,
            DiagnosticsDeliveryClass::ReconstructableFromReplay
        );
        assert!(rich_comparison.enabled);
        assert_eq!(rich_comparison.max_entries, 768);

        let rich_hot_trace = geometry_rich.artifact_policy(
            DiagnosticsScope::Transaction,
            DiagnosticsArtifactKind::DetailedTrace,
        );
        assert_eq!(
            rich_hot_trace.delivery_class,
            DiagnosticsDeliveryClass::CanDefer
        );
        assert!(!rich_hot_trace.enabled);
        assert_eq!(rich_hot_trace.max_entries, 0);
    }

    #[test]
    fn filter_artifact_canonicalizes_nested_field_order() {
        let profile = RelationalDiagnosticsProfile::default();
        let artifact = RelationalDiagnosticArtifact::new(
            DiagnosticsScope::History,
            DiagnosticsArtifactKind::MinimalSummary,
            DeterminismExpectation::Required,
            vec![RelationalDiagnosticsEntry {
                code: DiagnosticCode::CommitPublished,
                message: "ordered".to_string(),
                fields: RelationalDiagnosticFields::from_diagnostic_value(
                    RelationalDiagnosticValue::object([
                        (
                            "b",
                            RelationalDiagnosticValue::object([
                                ("z", RelationalDiagnosticValue::Unsigned(2)),
                                ("a", RelationalDiagnosticValue::Unsigned(1)),
                            ]),
                        ),
                        ("a", RelationalDiagnosticValue::Unsigned(1)),
                    ]),
                ),
            }],
        );

        let filtered = profile
            .filter_artifact(artifact)
            .expect("artifact should remain enabled");

        let expected = serde_json::json!({
            "a": 1,
            "b": {"a": 1, "z": 2},
        });
        assert_eq!(filtered.entries[0].fields, expected);
    }
}
