use worth_spatial::facade::planar_boolean_events::{
    PlanarBooleanEventLedgerLookupExecutionDenial,
    PlanarBooleanEventLedgerLookupExecutionDenialKind,
    PlanarBooleanEventLedgerLookupExecutionPacket, PlanarBooleanEventLedgerLookupExecutionWitness,
    PlanarBooleanEventLedgerReceipt,
};
use worth_spatial::facade::workload_vocabulary::CompleteWorkloadEvidenceLedger;
use worth_spatial::facade::{
    evidence_lookup_execution::EvidenceLookupExecutionReceipt,
    evidence_lookup_family_catalog::EvidenceLookupDiagnosticWitnessShape,
    evidence_lookup_plan_selection::EvidenceLookupSelectedPlan,
};

#[test]
fn spatial_public_api_exports_event_ledger_lookup_execution_witness_surface() {
    let _: fn(
        &PlanarBooleanEventLedgerReceipt,
        &CompleteWorkloadEvidenceLedger,
    ) -> Result<
        PlanarBooleanEventLedgerLookupExecutionPacket,
        PlanarBooleanEventLedgerLookupExecutionDenial,
    > = PlanarBooleanEventLedgerLookupExecutionPacket::admit;
    let _: fn(
        &PlanarBooleanEventLedgerLookupExecutionPacket,
    ) -> &PlanarBooleanEventLedgerLookupExecutionWitness =
        PlanarBooleanEventLedgerLookupExecutionPacket::witness;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionPacket) -> &str =
        PlanarBooleanEventLedgerLookupExecutionPacket::selected_family_identity;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionPacket) -> &str =
        PlanarBooleanEventLedgerLookupExecutionPacket::selected_family_declaration_digest;
    let _: fn(
        &PlanarBooleanEventLedgerLookupExecutionPacket,
    ) -> &EvidenceLookupDiagnosticWitnessShape =
        PlanarBooleanEventLedgerLookupExecutionPacket::selected_family_diagnostic_witness_shape;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionPacket) -> &EvidenceLookupSelectedPlan =
        PlanarBooleanEventLedgerLookupExecutionPacket::selected_plan;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionPacket) -> &EvidenceLookupExecutionReceipt =
        PlanarBooleanEventLedgerLookupExecutionPacket::execution_receipt;

    let _: fn(
        &PlanarBooleanEventLedgerReceipt,
        &CompleteWorkloadEvidenceLedger,
    ) -> Result<
        PlanarBooleanEventLedgerLookupExecutionWitness,
        PlanarBooleanEventLedgerLookupExecutionDenial,
    > = PlanarBooleanEventLedgerLookupExecutionWitness::admit;

    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::event_ledger_identity;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::spatial_touch_digest;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::stage_index_identity;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::selected_plan_digest;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::execution_receipt_digest;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::lookup_product_output_digest;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionWitness) -> &str =
        PlanarBooleanEventLedgerLookupExecutionWitness::evidence_ledger_basis_digest;

    let _: fn(
        &PlanarBooleanEventLedgerLookupExecutionDenial,
    ) -> PlanarBooleanEventLedgerLookupExecutionDenialKind =
        PlanarBooleanEventLedgerLookupExecutionDenial::kind;
    let _: fn(&PlanarBooleanEventLedgerLookupExecutionDenial) -> &str =
        PlanarBooleanEventLedgerLookupExecutionDenial::detail;
}
