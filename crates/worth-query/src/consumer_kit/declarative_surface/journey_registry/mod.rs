mod consumers;
mod query_capabilities;

use super::{WorthQueryConsumerJourneyRow as Row, WorthQueryDeclarativeCapabilityFamily as Family};

pub fn worth_query_consumer_journey_rows() -> &'static [Row] {
    static ROWS: std::sync::OnceLock<Vec<Row>> = std::sync::OnceLock::new();
    ROWS.get_or_init(|| {
        query_capabilities::rows()
            .iter()
            .chain(consumers::rows())
            .copied()
            .collect()
    })
}

pub(super) struct JourneyEntry {
    pub(super) id: &'static str,
    pub(super) consumer: &'static str,
    pub(super) source: &'static str,
    pub(super) probe: &'static str,
    pub(super) family: Family,
    pub(super) meaning: JourneyMeaning,
    pub(super) evidence: JourneyEvidence,
    pub(super) cutover: JourneyCutover,
}

pub(super) struct JourneyMeaning {
    pub(super) intent: &'static str,
    pub(super) context: &'static str,
    pub(super) capability: &'static str,
    pub(super) phase_chain: &'static str,
}

pub(super) struct JourneyEvidence {
    pub(super) result: &'static str,
    pub(super) receipts: &'static str,
    pub(super) diagnostics: &'static str,
    pub(super) counters: &'static str,
}

pub(super) struct JourneyCutover {
    pub(super) local_ceremony: &'static str,
    pub(super) replacement: &'static str,
}

pub(super) const fn row(entry: JourneyEntry) -> Row {
    Row::new(
        entry.id,
        entry.consumer,
        entry.source,
        entry.probe,
        entry.family,
        entry.meaning.intent,
        entry.meaning.context,
        entry.meaning.capability,
        entry.meaning.phase_chain,
        entry.evidence.result,
        entry.evidence.receipts,
        entry.evidence.diagnostics,
        entry.evidence.counters,
        entry.cutover.local_ceremony,
        entry.cutover.replacement,
    )
}
