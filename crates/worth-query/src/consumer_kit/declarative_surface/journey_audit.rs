use std::collections::BTreeMap;

use super::{
    worth_query_consumer_journey_rows, WorthQueryConsumerJourneyAudit,
    WorthQueryConsumerJourneyFinding, WorthQueryConsumerJourneyFindingKind,
    WorthQueryConsumerJourneySource,
};

pub fn audit_consumer_journey_sources(
    sources: &[WorthQueryConsumerJourneySource<'_>],
) -> WorthQueryConsumerJourneyAudit {
    let source_text = sources
        .iter()
        .map(|source| (source.path(), source.text().replace("\r\n", "\n")))
        .collect::<BTreeMap<_, _>>();
    let mut classified_journey_count = 0;
    let mut findings = Vec::new();

    for row in worth_query_consumer_journey_rows() {
        let Some(text) = source_text.get(row.source_path()) else {
            findings.push(WorthQueryConsumerJourneyFinding::new(
                WorthQueryConsumerJourneyFindingKind::MissingSource,
                row,
            ));
            continue;
        };
        let normalized_probe = row.source_probe().replace("\r\n", "\n");
        match text.match_indices(&normalized_probe).count() {
            0 => findings.push(WorthQueryConsumerJourneyFinding::new(
                WorthQueryConsumerJourneyFindingKind::MissingSourceProbe,
                row,
            )),
            1 => classified_journey_count += 1,
            _ => findings.push(WorthQueryConsumerJourneyFinding::new(
                WorthQueryConsumerJourneyFindingKind::AmbiguousSourceProbe,
                row,
            )),
        }
    }

    WorthQueryConsumerJourneyAudit::new(classified_journey_count, findings)
}

#[cfg(test)]
pub(super) fn workspace_consumer_journey_audit() -> WorthQueryConsumerJourneyAudit {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query must remain below the workspace root");
    let source_text = worth_query_consumer_journey_rows()
        .iter()
        .map(|row| {
            let path = workspace_root.join(row.source_path());
            let text = std::fs::read_to_string(&path)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", path.display()));
            (row.source_path(), text)
        })
        .collect::<Vec<_>>();
    let sources = source_text
        .iter()
        .map(|(path, text)| WorthQueryConsumerJourneySource::new(path, text))
        .collect::<Vec<_>>();
    audit_consumer_journey_sources(&sources)
}
