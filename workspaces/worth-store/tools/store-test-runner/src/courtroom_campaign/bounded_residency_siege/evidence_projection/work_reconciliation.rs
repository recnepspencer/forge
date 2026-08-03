use serde::ser::{Serialize, SerializeMap, Serializer};

use super::super::protocol::BoundedResidencyWorkReconciliationObservation;

#[path = "work_reconciliation/record.rs"]
mod record;
#[path = "work_reconciliation/signal_binding.rs"]
mod signal_binding;

pub(super) struct WorkReconciliationProjection<'evidence>(
    pub(super) &'evidence BoundedResidencyWorkReconciliationObservation,
);

impl Serialize for WorkReconciliationProjection<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let evidence = self.0;
        let mut map = serializer.serialize_map(Some(12))?;
        map.serialize_entry("causal_overflow", &evidence.causal_overflow)?;
        map.serialize_entry("terminal_overflow", &evidence.terminal_overflow)?;
        map.serialize_entry("safe_evidence_elided", &evidence.safe_evidence_elided)?;
        map.serialize_entry("faults", &evidence.faults)?;
        map.serialize_entry("source_loads", &evidence.source_loads)?;
        map.serialize_entry("exact_writebacks", &evidence.exact_writebacks)?;
        map.serialize_entry(
            "identified_metadata_reads",
            &evidence.identified_metadata_reads,
        )?;
        map.serialize_entry(
            "identified_positioned_reads",
            &evidence.identified_positioned_reads,
        )?;
        map.serialize_entry(
            "identified_positioned_writes",
            &evidence.identified_positioned_writes,
        )?;
        map.serialize_entry("terminal_fates", &TerminalFates(evidence))?;
        map.serialize_entry(
            "signal_bindings",
            &signal_binding::SignalBindings(&evidence.signal_bindings),
        )?;
        map.serialize_entry("records", &record::Records(&evidence.records))?;
        map.end()
    }
}

struct TerminalFates<'evidence>(&'evidence BoundedResidencyWorkReconciliationObservation);

impl Serialize for TerminalFates<'_> {
    fn serialize<S>(&self, serializer: S) -> Result<S::Ok, S::Error>
    where
        S: Serializer,
    {
        let mut map = serializer.serialize_map(Some(2))?;
        map.serialize_entry("settled", &self.0.settled_terminal_fates)?;
        map.serialize_entry(
            "continued_after_consumer_cancellation",
            &self.0.continued_terminal_fates,
        )?;
        map.end()
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::courtroom_campaign::bounded_residency_siege::protocol::{
        BoundedResidencyMediaRole, BoundedResidencySignalAspectRole,
        BoundedResidencySignalBindingObservation, BoundedResidencySignalFamilySet,
        BoundedResidencyWorkEffectFate, BoundedResidencyWorkFamily,
        BoundedResidencyWorkRecordObservation, BoundedResidencyWorkRecovery,
        BoundedResidencyWorkTerminalFate,
    };

    #[test]
    fn projection_preserves_every_identity_receipt_fate_and_terminal_field() {
        let evidence = BoundedResidencyWorkReconciliationObservation {
            causal_overflow: 0,
            terminal_overflow: 0,
            safe_evidence_elided: 0,
            faults: 1,
            source_loads: 1,
            exact_writebacks: 0,
            identified_metadata_reads: 0,
            identified_positioned_reads: 1,
            identified_positioned_writes: 0,
            settled_terminal_fates: 1,
            continued_terminal_fates: 0,
            signal_bindings: vec![BoundedResidencySignalBindingObservation {
                digest: [7; 32],
                aspect_key: "store.physical.record.frame-read-basis".to_owned(),
                role: BoundedResidencySignalAspectRole::Dependency,
                families: BoundedResidencySignalFamilySet {
                    read_fault: true,
                    exact_writeback: false,
                    publication: false,
                    lifecycle: false,
                },
                partition: Some("store.physical.record.frame".to_owned()),
            }]
            .into_boxed_slice(),
            records: vec![BoundedResidencyWorkRecordObservation {
                store: [7; 16],
                runtime: 11,
                generation: 13,
                operation: 17,
                family: BoundedResidencyWorkFamily::ArtifactRangeRead,
                backend_operation: 19,
                backend_role: BoundedResidencyMediaRole::PositionedRead,
                effect_fate: BoundedResidencyWorkEffectFate::ReadCompleted,
                recovery: BoundedResidencyWorkRecovery::NoEffect,
                route: crate::courtroom_campaign::bounded_residency_siege::protocol::exact_route_fixture(
                    17,
                    BoundedResidencyWorkFamily::ArtifactRangeRead,
                    [7; 32],
                ),
                terminal: BoundedResidencyWorkTerminalFate::Settled,
            }]
            .into_boxed_slice(),
        };
        let encoded = serde_json::to_value(WorkReconciliationProjection(&evidence)).unwrap();
        assert_eq!(encoded["faults"], 1);
        assert_eq!(encoded["identified_metadata_reads"], 0);
        assert_eq!(encoded["terminal_fates"]["settled"], 1);
        assert_eq!(
            encoded["signal_bindings"][0]["digest"],
            "0707070707070707070707070707070707070707070707070707070707070707"
        );
        assert_eq!(
            encoded["signal_bindings"][0]["aspect_key"],
            "store.physical.record.frame-read-basis"
        );
        assert_eq!(encoded["signal_bindings"][0]["role"], "dependency");
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["read_fault"],
            true
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["exact_writeback"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["publication"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["families"]["lifecycle"],
            false
        );
        assert_eq!(
            encoded["signal_bindings"][0]["partition"],
            "store.physical.record.frame"
        );
        assert_eq!(
            encoded["terminal_fates"]["continued_after_consumer_cancellation"],
            0
        );
        assert_eq!(
            encoded["records"][0]["store"],
            "07070707070707070707070707070707"
        );
        assert_eq!(encoded["records"][0]["runtime"], 11);
        assert_eq!(encoded["records"][0]["generation"], 13);
        assert_eq!(encoded["records"][0]["operation"], 17);
        assert_eq!(encoded["records"][0]["family"], "artifact-range-read");
        assert_eq!(encoded["records"][0]["backend_operation"], 19);
        assert_eq!(encoded["records"][0]["backend_role"], "positioned-read");
        assert_eq!(encoded["records"][0]["effect_fate"], "read-completed");
        assert_eq!(encoded["records"][0]["recovery"], "no-effect");
        assert_eq!(encoded["records"][0]["terminal"], "settled");
    }
}
