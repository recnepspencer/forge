//! P2-5 Replay / Audit Bridge
//!
//! Exposes compatibility mappings and deterministic bridge records from
//! versioned audit payloads into the counterfactual replay system.
//!
//! INV-3 Guard for NURBS: The compatibility gate explicitly rejects
//! exact replay for entities that originated from `GeometricIntersection`
//! (unsupported origin) unless a proper counterfactual override is available.

use serde::{Deserialize, Serialize};
use forge_core::tracing::DecisionId;
use crate::audit::schema::AuditBundleManifest;
use crate::audit::schema::AUDIT_SCHEMA_VERSION;
use forge_core::tracing::TraceFingerprint;

/// Compatibility rating for a trace bundle against the replay system.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayCompatibility {
    /// Exact deterministic replay is fully supported.
    Compatible,
    /// Counterfactual (policy/trace override) replay is supported,
    /// but exact reproducibility without overrides is not guaranteed.
    CounterfactualOnly,
    /// Cannot replay because a topology snapshot is missing but required.
    RequiresSnapshot,
    /// Cannot replay because no witnesses were provided.
    RequiresWitness,
    /// The audit schema version is incompatible with this bridge.
    SchemaVersionMismatch {
        recorded: u32,
        supported: u32,
    },
    /// The operation payload version is unsupported.
    UnsupportedOperationVersion {
        version: u32,
    },
    /// The origin of an entity in the operation cannot be forward-linked
    /// natively (e.g. `GeometricIntersection` from NURBS).
    UnsupportedEntityOrigin {
        origin: String,
    },
}

/// Category of a provided replay witness.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub enum ReplayWitnessKind {
    /// A forced override policy decision.
    PolicyDecision,
    /// An explicit provenance trace snippet.
    ProvenanceTrace,
    /// Complete topological state snapshot.
    TopologySnapshot,
    /// Requires evaluation of a geometric intersection (e.g. NURBS).
    GeometricIntersection,
}

/// A reference to a witness used for replay gating and execution.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayWitnessRef {
    pub decision_id: DecisionId,
    pub witness_kind: ReplayWitnessKind,
    pub scope_id: Option<String>,
}

/// Serialized canonical summary of a bridge mapping from an audit record.
#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct ReplayBridgeRecord {
    pub schema_version: u32,
    pub operation_id: String,
    pub operation_type: String,
    pub operation_version: u32,
    pub compatibility: ReplayCompatibility,
    pub witnesses: Vec<ReplayWitnessRef>,
    pub trace_fingerprint: Option<TraceFingerprint>,
}

/// Core mapping function: determines if an audit manifest and its accompanying
/// witnesses can be routed into the counterfactual replay pipeline.
pub fn build_replay_bridge_record(
    manifest: &AuditBundleManifest,
    trace_fingerprint: Option<TraceFingerprint>,
    witnesses: Vec<ReplayWitnessRef>,
) -> ReplayBridgeRecord {
    let compatibility = if manifest.schema_version != AUDIT_SCHEMA_VERSION {
        ReplayCompatibility::SchemaVersionMismatch {
            recorded: manifest.schema_version,
            supported: AUDIT_SCHEMA_VERSION,
        }
    } else if manifest.operation_version == 0 {
        ReplayCompatibility::UnsupportedOperationVersion {
            version: manifest.operation_version,
        }
    } else if witnesses.is_empty() {
        ReplayCompatibility::RequiresWitness
    } else if witnesses.iter().any(|w| w.witness_kind == ReplayWitnessKind::GeometricIntersection) {
        // NURBS INV-3 Guard: Any GeometricIntersection witness flags the record as
        // unsupported origin for exact replay. The presence of *any* PolicyDecision
        // witness anywhere in the list (regardless of order or position) overrides
        // this and downgrades to CounterfactualOnly. Without a PolicyDecision override,
        // exact replay is architecturally forbidden for intersection-derived entities.
        if witnesses.iter().any(|w| w.witness_kind == ReplayWitnessKind::PolicyDecision) {
            ReplayCompatibility::CounterfactualOnly
        } else {
            ReplayCompatibility::UnsupportedEntityOrigin {
                origin: "GeometricIntersection".to_string(),
            }
        }
    } else if witnesses.iter().any(|w| w.witness_kind == ReplayWitnessKind::TopologySnapshot) {
        ReplayCompatibility::Compatible
    } else if witnesses.iter().any(|w| w.witness_kind == ReplayWitnessKind::ProvenanceTrace)
        && !witnesses.iter().any(|w| w.witness_kind == ReplayWitnessKind::PolicyDecision)
    {
        // ProvenanceTrace is present but no topology snapshot: replay requires
        // a full TopologySnapshot to reconstruct the exact pre-op state.
        ReplayCompatibility::RequiresSnapshot
    } else {
        ReplayCompatibility::CounterfactualOnly
    };

    ReplayBridgeRecord {
        schema_version: manifest.schema_version,
        operation_id: manifest.operation_id.clone(),
        operation_type: manifest.operation_type.clone(),
        operation_version: manifest.operation_version,
        compatibility,
        witnesses,
        trace_fingerprint,
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::audit::schema::AuditBundleFiles;

    fn test_manifest(schema_version: u32, op_version: u32) -> AuditBundleManifest {
        AuditBundleManifest {
            schema_version,
            operation_id: "test_op_1".to_string(),
            operation_type: "region_merge".to_string(),
            operation_version: op_version,
            created_at_unix_millis: 10000,
            files: AuditBundleFiles::default(),
        }
    }

    #[test]
    fn replay_bridge_identifies_missing_witness_vs_schema_mismatch_distinctly() {
        // Missing witnesses test
        let manifest_good = test_manifest(AUDIT_SCHEMA_VERSION, 1);
        let record_no_witness = build_replay_bridge_record(&manifest_good, None, vec![]);
        assert_eq!(record_no_witness.compatibility, ReplayCompatibility::RequiresWitness);

        // Schema mismatch test (takes precedence over missing witnesses if both are valid)
        let manifest_bad_schema = test_manifest(AUDIT_SCHEMA_VERSION + 1, 1);
        let record_bad_schema = build_replay_bridge_record(&manifest_bad_schema, None, vec![]);
        assert_eq!(
            record_bad_schema.compatibility,
            ReplayCompatibility::SchemaVersionMismatch {
                recorded: AUDIT_SCHEMA_VERSION + 1,
                supported: AUDIT_SCHEMA_VERSION,
            }
        );
    }

    #[test]
    fn replay_bridge_output_is_deterministic_for_identical_audit_record() {
        let manifest = test_manifest(AUDIT_SCHEMA_VERSION, 1);
        let witnesses = vec![ReplayWitnessRef {
            decision_id: DecisionId(5),
            witness_kind: ReplayWitnessKind::TopologySnapshot,
            scope_id: None,
        }];
        
        let record_1 = build_replay_bridge_record(&manifest, None, witnesses.clone());
        let record_2 = build_replay_bridge_record(&manifest, None, witnesses);
        
        assert_eq!(record_1, record_2);
    }

    #[test]
    fn replay_bridge_preserves_typed_error_summary_in_failure_path() {
        let manifest = test_manifest(AUDIT_SCHEMA_VERSION, 0); // version 0 triggers Unsupported
        let record = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(1),
                witness_kind: ReplayWitnessKind::TopologySnapshot,
                scope_id: None,
            }
        ]);
        
        assert_eq!(
            record.compatibility,
            ReplayCompatibility::UnsupportedOperationVersion { version: 0 }
        );
    }

    #[test]
    fn replay_bridge_distinguishes_exact_compatible_vs_counterfactual_only() {
        let manifest = test_manifest(AUDIT_SCHEMA_VERSION, 1);
        
        // Exact compatible with TopologySnapshot
        let record_exact = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(1),
                witness_kind: ReplayWitnessKind::TopologySnapshot,
                scope_id: None,
            }
        ]);
        assert_eq!(record_exact.compatibility, ReplayCompatibility::Compatible);
        
        // Counterfactual only with PolicyDecision
        let record_cf = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(2),
                witness_kind: ReplayWitnessKind::PolicyDecision,
                scope_id: None,
            }
        ]);
        assert_eq!(record_cf.compatibility, ReplayCompatibility::CounterfactualOnly);
    }

    #[test]
    fn replay_bridge_respects_nurbs_invariant_for_geometric_intersection() {
        let manifest = test_manifest(AUDIT_SCHEMA_VERSION, 1);
        
        // Geometric intersection without policy override downgrades to Unsupported
        let record_nurbs = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(1),
                witness_kind: ReplayWitnessKind::GeometricIntersection,
                scope_id: None,
            }
        ]);
        
        match record_nurbs.compatibility {
            ReplayCompatibility::UnsupportedEntityOrigin { origin } => {
                assert_eq!(origin, "GeometricIntersection");
            }
            _ => panic!("Expected UnsupportedEntityOrigin"),
        }
        
        // Geometric intersection with policy override upgrades to CounterfactualOnly
        let record_overridden = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(1),
                witness_kind: ReplayWitnessKind::GeometricIntersection,
                scope_id: None,
            },
            ReplayWitnessRef {
                decision_id: DecisionId(2),
                witness_kind: ReplayWitnessKind::PolicyDecision,
                scope_id: None,
            }
        ]);
        
        assert_eq!(record_overridden.compatibility, ReplayCompatibility::CounterfactualOnly);
    }

    #[test]
    fn replay_bridge_requires_snapshot_when_only_provenance_trace_present() {
        let manifest = test_manifest(AUDIT_SCHEMA_VERSION, 1);

        // Only a ProvenanceTrace witness — no TopologySnapshot, no PolicyDecision.
        // The bridge must signal that a full snapshot is needed to reconstruct the pre-op state.
        let record = build_replay_bridge_record(&manifest, None, vec![
            ReplayWitnessRef {
                decision_id: DecisionId(1),
                witness_kind: ReplayWitnessKind::ProvenanceTrace,
                scope_id: Some("op-0001/trace.json".to_string()),
            }
        ]);

        assert_eq!(
            record.compatibility,
            ReplayCompatibility::RequiresSnapshot,
            "ProvenanceTrace-only witness must require a TopologySnapshot for exact replay"
        );
    }
}
