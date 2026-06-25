use super::super::query_admission_projection::WorthGraphReadAdmissionCapabilityGapKind;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthGraphReadAdmissionGapCapLedgerRow {
    kind: WorthGraphReadAdmissionCapabilityGapKind,
    owner: &'static str,
    introduced_in: &'static str,
    must_not_exceed_count: usize,
    blocker: &'static str,
    removal_trigger: &'static str,
}

impl WorthGraphReadAdmissionGapCapLedgerRow {
    pub const fn new(
        kind: WorthGraphReadAdmissionCapabilityGapKind,
        owner: &'static str,
        introduced_in: &'static str,
        must_not_exceed_count: usize,
        blocker: &'static str,
        removal_trigger: &'static str,
    ) -> Self {
        Self {
            kind,
            owner,
            introduced_in,
            must_not_exceed_count,
            blocker,
            removal_trigger,
        }
    }

    pub const fn kind(&self) -> WorthGraphReadAdmissionCapabilityGapKind {
        self.kind
    }

    pub const fn owner(&self) -> &'static str {
        self.owner
    }

    pub const fn introduced_in(&self) -> &'static str {
        self.introduced_in
    }

    pub const fn must_not_exceed_count(&self) -> usize {
        self.must_not_exceed_count
    }

    pub const fn blocker(&self) -> &'static str {
        self.blocker
    }

    pub const fn removal_trigger(&self) -> &'static str {
        self.removal_trigger
    }

    pub fn digest_part(&self) -> String {
        format!(
            "{}:{}:{}:{}:{}:{}",
            self.kind.as_str(),
            self.owner,
            self.introduced_in,
            self.must_not_exceed_count,
            self.blocker,
            self.removal_trigger
        )
    }
}

pub const ADMISSION_GAP_CAP_LEDGER: [WorthGraphReadAdmissionGapCapLedgerRow; 7] = [
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::RequirementDerivationBlocked,
        "worth_graph_read_declarations",
        "touched_graph_milestone_7_phase_5",
        5,
        "Phase 4 has not yet lowered catalog records into real ForgeQueryReadFamily artifacts.",
        "Remove this cap when Phase 4 emits real ForgeQueryReadFamily artifacts for every declaration.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::MissingQueryReadFamilyArtifact,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Real Query read-family artifact is missing before admission.",
        "Replace anchor-only requirement records with Query-owned read-family artifacts.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::PersistentIndexRequired,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Query admission requires persistent graph index support.",
        "Provide Query persistent index support or convert the declaration to an admitted posture.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::PagedStreamingRequired,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Query admission requires paged streaming support.",
        "Provide Query paged streaming support or convert the declaration to an admitted posture.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::AsyncMaterializationRequired,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Query admission requires async materialization support.",
        "Provide Query async materialization support or convert the declaration to an admitted posture.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::StoreBackedCapabilityRequired,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Query admission requires store-backed graph index support.",
        "Provide Query store-backed graph index support or convert the declaration to an admitted posture.",
    ),
    WorthGraphReadAdmissionGapCapLedgerRow::new(
        WorthGraphReadAdmissionCapabilityGapKind::AccessCapabilityRegistrationRequired,
        "forge_query",
        "touched_graph_milestone_7_phase_5",
        0,
        "Query admission requires access capability registration.",
        "Register the Query access capability or convert the declaration to an admitted posture.",
    ),
];

pub fn admission_gap_cap_ledger_row(
    kind: WorthGraphReadAdmissionCapabilityGapKind,
) -> Option<&'static WorthGraphReadAdmissionGapCapLedgerRow> {
    ADMISSION_GAP_CAP_LEDGER
        .iter()
        .find(|row| row.kind() == kind)
}
