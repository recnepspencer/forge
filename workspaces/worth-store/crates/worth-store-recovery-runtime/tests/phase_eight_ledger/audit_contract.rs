use std::collections::BTreeSet;
use std::path::Path;

const REQUIRED_CERTIFICATIONS: [(&str, &str, &str); 4] = [
    (
        "fresh current-tree Phase 8 QA-loop after repair",
        "gpt-5.6-luna max",
        "/root/c8_phase8_qa_loop_",
    ),
    (
        "fresh current-tree Phase 8 QA-tests after repair",
        "gpt-5.6-luna max",
        "/root/c8_phase8_qa_tests_",
    ),
    (
        "fresh current-tree Phase 8 code-quality after repair",
        "gpt-5.6-luna max",
        "/root/c8_phase8_code_quality_",
    ),
    (
        "fresh current-tree Phase 8 Sol-high gate after all independent reviews",
        "gpt-5.6-sol high",
        "/root/c8_phase8_sol_gate_",
    ),
];

pub(super) fn finding_rows(document: &str) -> BTreeSet<String> {
    section_rows(
        document,
        "## Phase 8 finding history",
        "## Independent audit history",
    )
    .filter(|columns| {
        columns
            .first()
            .is_some_and(|value| value.starts_with("C8-P8-F"))
    })
    .map(|columns| columns.join("|"))
    .collect()
}

pub(super) fn validate(document: &str) {
    let rows = section_rows(document, "## Independent audit history", "\u{0}")
        .filter(|columns| {
            columns
                .first()
                .is_some_and(|value| *value != "Reviewer" && *value != "---")
        })
        .collect::<Vec<_>>();
    assert!(!rows.is_empty(), "audit history cannot disappear");
    for row in &rows {
        assert_eq!(row.len(), 6, "audit row must retain all six columns");
        assert!(!row[0].is_empty(), "audit row reviewer cannot be empty");
        assert!(!row[1].is_empty(), "audit row model cannot be empty");
        assert!(!row[2].is_empty(), "audit row scope cannot be empty");
        assert!(matches!(row[3], "CLEAN" | "NOT CLEAN"));
        assert!(
            !row[4].is_empty(),
            "audit row finding field cannot be empty"
        );
        assert!(!row[5].is_empty(), "audit row posture cannot be empty");
    }
    let certification_reviewers = REQUIRED_CERTIFICATIONS
        .into_iter()
        .map(|(scope, model, reviewer_prefix)| {
            rows.iter()
                .find(|row| {
                    row[1] == model
                        && row[2] == scope
                        && row[3] == "CLEAN"
                        && row[0].starts_with(reviewer_prefix)
                        && row[0].len() > reviewer_prefix.len()
                })
                .unwrap_or_else(|| panic!("missing clean certification row: {scope}"))[0]
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        certification_reviewers.len(),
        REQUIRED_CERTIFICATIONS.len(),
        "Phase 8 certification rows must come from four fresh reviewer instances"
    );
}

pub(super) fn validate_external_audits(root: &Path, source_closure_digest: &str) {
    let path = root.join("_docs/worth-store/physical-reconstruction-c8-qa-audits.csv");
    let document = std::fs::read_to_string(&path)
        .unwrap_or_else(|error| panic!("read external Phase 8 audit history {path:?}: {error}"));
    let mut lines = document.lines();
    assert_eq!(
        lines.next(),
        Some("reviewer,model,revision,source_snapshot,prompt,finding_ids,disposition,verification"),
        "external audit history header drifted"
    );
    let rows = lines
        .filter(|line| !line.is_empty())
        .map(|line| {
            let columns = line.split(',').collect::<Vec<_>>();
            assert_eq!(
                columns.len(),
                8,
                "external audit row must retain all eight columns"
            );
            columns
        })
        .collect::<Vec<_>>();
    let certification_reviewers = REQUIRED_CERTIFICATIONS
        .into_iter()
        .map(|(scope, model, reviewer_prefix)| {
            rows.iter()
                .find(|row| {
                    row[1] == model
                        && row[4] == scope
                        && row[0].starts_with(reviewer_prefix)
                        && row[0].len() > reviewer_prefix.len()
                        && row[3] == source_closure_digest
                        && row[6] == "clean current-tree certification"
                        && row[5] == "none"
                })
                .unwrap_or_else(|| panic!("missing clean external certification row: {scope}"))[0]
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(
        certification_reviewers.len(),
        REQUIRED_CERTIFICATIONS.len(),
        "external Phase 8 audits must come from four fresh reviewer instances"
    );
}

pub(super) fn validate_correspondence(root: &Path, document: &str) {
    let markdown = section_rows(document, "## Independent audit history", "\u{0}")
        .filter(|row| {
            REQUIRED_CERTIFICATIONS
                .iter()
                .any(|(scope, model, prefix)| {
                    row.get(1) == Some(model)
                        && row.get(2) == Some(scope)
                        && row.first().is_some_and(|reviewer| {
                            reviewer.starts_with(prefix) && reviewer.len() > prefix.len()
                        })
                })
        })
        .map(|row| {
            (
                row[0].to_owned(),
                row[1].to_owned(),
                row[2].to_owned(),
                row[3].to_owned(),
            )
        })
        .collect::<BTreeSet<_>>();
    let csv = std::fs::read_to_string(
        root.join("_docs/worth-store/physical-reconstruction-c8-qa-audits.csv"),
    )
    .unwrap();
    let external = csv
        .lines()
        .skip(1)
        .filter_map(|line| {
            let columns = line.split(',').collect::<Vec<_>>();
            (columns.len() == 8
                && REQUIRED_CERTIFICATIONS
                    .iter()
                    .any(|(scope, model, prefix)| {
                        columns[1] == *model
                            && columns[4] == *scope
                            && columns[0].starts_with(prefix)
                            && columns[0].len() > prefix.len()
                    }))
            .then_some((
                columns[0].to_owned(),
                columns[1].to_owned(),
                columns[4].to_owned(),
                "CLEAN".to_owned(),
            ))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(markdown, external, "Phase 8 audit artifacts diverged");
}

fn section_rows<'a>(
    document: &'a str,
    start_heading: &str,
    end_heading: &str,
) -> impl Iterator<Item = Vec<&'a str>> {
    let start = document.find(start_heading).unwrap();
    let end = document[start + start_heading.len()..]
        .find(end_heading)
        .map_or(document.len(), |offset| {
            start + start_heading.len() + offset
        });
    document[start..end].lines().filter_map(|line| {
        line.strip_prefix("| ").map(|row| {
            row.trim_end_matches(" |")
                .split(" | ")
                .map(str::trim)
                .collect()
        })
    })
}
