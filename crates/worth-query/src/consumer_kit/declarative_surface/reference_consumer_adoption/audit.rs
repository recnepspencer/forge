use std::collections::BTreeMap;

use super::{
    worth_query_reference_consumer_adoption_rows, worth_query_reference_consumer_deleted_residue,
    WorthQueryReferenceConsumerAdoptionAudit, WorthQueryReferenceConsumerAdoptionFinding,
    WorthQueryReferenceConsumerAdoptionFindingKind, WorthQueryReferenceConsumerSource,
};

pub fn audit_reference_consumer_adoption_sources(
    sources: &[WorthQueryReferenceConsumerSource<'_>],
) -> WorthQueryReferenceConsumerAdoptionAudit {
    let sources = sources
        .iter()
        .map(|source| (source.path(), source.text().replace("\r\n", "\n")))
        .collect::<BTreeMap<_, _>>();
    let mut findings = Vec::new();
    let mut adopted_consumer_count = 0;

    for row in worth_query_reference_consumer_adoption_rows() {
        let Some(text) = sources.get(row.source_path()) else {
            findings.push(WorthQueryReferenceConsumerAdoptionFinding::current(
                WorthQueryReferenceConsumerAdoptionFindingKind::MissingCurrentSource,
                row,
            ));
            continue;
        };
        match text.match_indices(row.current_probe()).count() {
            0 => findings.push(WorthQueryReferenceConsumerAdoptionFinding::current(
                WorthQueryReferenceConsumerAdoptionFindingKind::MissingCurrentProbe,
                row,
            )),
            1 => adopted_consumer_count += 1,
            _ => findings.push(WorthQueryReferenceConsumerAdoptionFinding::current(
                WorthQueryReferenceConsumerAdoptionFindingKind::AmbiguousCurrentProbe,
                row,
            )),
        }
    }

    let mut deleted_residue_count = 0;
    for residue in worth_query_reference_consumer_deleted_residue() {
        let Some(text) = sources.get(residue.source_path()) else {
            continue;
        };
        if text.contains(residue.probe()) {
            findings.push(WorthQueryReferenceConsumerAdoptionFinding::residue(residue));
        } else {
            deleted_residue_count += 1;
        }
    }

    let before_ceremony_count = worth_query_reference_consumer_adoption_rows()
        .iter()
        .map(|row| row.before().ceremony_count())
        .sum();
    let after_ceremony_count = worth_query_reference_consumer_adoption_rows()
        .iter()
        .map(|row| row.after().ceremony_count())
        .sum();
    WorthQueryReferenceConsumerAdoptionAudit::new(
        adopted_consumer_count,
        deleted_residue_count,
        before_ceremony_count,
        after_ceremony_count,
        findings,
    )
}

#[cfg(test)]
pub(crate) fn workspace_reference_consumer_adoption_audit(
) -> WorthQueryReferenceConsumerAdoptionAudit {
    let crate_root = std::path::Path::new(env!("CARGO_MANIFEST_DIR"));
    let workspace_root = crate_root
        .parent()
        .and_then(std::path::Path::parent)
        .expect("worth-query must remain below the workspace root");
    let paths = worth_query_reference_consumer_adoption_rows()
        .iter()
        .map(|row| row.source_path())
        .chain(
            worth_query_reference_consumer_deleted_residue()
                .iter()
                .map(|row| row.source_path()),
        )
        .collect::<std::collections::BTreeSet<_>>();
    let text = paths
        .into_iter()
        .map(|path| {
            let absolute = workspace_root.join(path);
            let source = std::fs::read_to_string(&absolute)
                .unwrap_or_else(|error| panic!("failed to read {}: {error}", absolute.display()));
            (path, source)
        })
        .collect::<Vec<_>>();
    let sources = text
        .iter()
        .map(|(path, text)| WorthQueryReferenceConsumerSource::new(path, text))
        .collect::<Vec<_>>();
    audit_reference_consumer_adoption_sources(&sources)
}
