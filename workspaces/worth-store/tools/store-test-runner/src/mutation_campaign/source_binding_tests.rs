use std::collections::BTreeSet;

use super::{mutations, ControlledMutation, MutationTarget};

#[test]
fn every_mutant_is_bound_to_one_current_source_seam() {
    let workspace = std::path::Path::new(env!("CARGO_MANIFEST_DIR"))
        .parent()
        .and_then(std::path::Path::parent)
        .unwrap();
    let mut identities = BTreeSet::new();
    for mutation in mutations() {
        assert!(
            identities.insert(mutation.id),
            "duplicate mutant {}",
            mutation.id
        );
    }
    let failures = binding_failures(mutations(), |source| {
        let path = workspace.join(source);
        std::fs::read_to_string(&path)
            .map_err(|error| format!("cannot read {}: {error}", path.display()))
    });
    assert!(
        failures.is_empty(),
        "mutation source binding failures:\n{}",
        failures.join("\n")
    );
}

#[test]
fn binding_audit_reports_every_stale_seam_in_one_execution() {
    let first = fixture_mutation(201, "missing-first");
    let second = fixture_mutation(202, "missing-second");

    let failures = binding_failures([&first, &second], |_| Ok("current source".to_owned()));

    assert_eq!(failures.len(), 2);
    assert!(failures[0].contains("mutant 201"));
    assert!(failures[1].contains("mutant 202"));
}

#[test]
fn mutation_source_binding_follows_lf_and_crlf_without_changing_the_seam() {
    let mutation = &mutations()[0];
    let lf_source = mutation.needle.to_owned();
    let crlf_source = mutation.needle.replace('\n', "\r\n");

    assert_eq!(
        lf_source
            .matches(mutation.source_needle(&lf_source).as_ref())
            .count(),
        1
    );
    assert_eq!(
        crlf_source
            .matches(mutation.source_needle(&crlf_source).as_ref())
            .count(),
        1
    );
    let crlf_replacement = mutation.source_replacement(&crlf_source);
    assert_eq!(
        crlf_replacement.matches("\r\n").count(),
        mutation.replacement.matches('\n').count()
    );
    assert!(!crlf_replacement.replace("\r\n", "").contains('\n'));
}

#[test]
fn mutation_source_binding_uses_the_matched_seams_line_ending_in_a_mixed_file() {
    let mutation = &mutations()[0];
    let mixed_source = format!("{}\r\nmixed trailer", mutation.needle);

    assert_eq!(mutation.source_needle(&mixed_source), mutation.needle);
    assert_eq!(
        mutation.source_replacement(&mixed_source),
        mutation.replacement
    );
}

#[test]
fn mutation_source_binding_rejects_the_same_seam_in_both_line_endings() {
    let mutation = &mutations()[0];
    let crlf = mutation.needle.replace('\n', "\r\n");
    let mixed_duplicate = format!("{}\r\n{crlf}", mutation.needle);

    assert_eq!(mutation.source_occurrences(&mixed_duplicate), 2);
}

fn binding_failures<'mutation>(
    mutations: impl IntoIterator<Item = &'mutation ControlledMutation>,
    mut read_source: impl FnMut(&str) -> Result<String, String>,
) -> Vec<String> {
    mutations
        .into_iter()
        .filter_map(|mutation| match read_source(mutation.source) {
            Ok(source) => {
                let occurrences = mutation.source_occurrences(&source);
                (occurrences != 1).then(|| {
                    format!(
                        "mutant {} must bind exactly once in {}, found {occurrences}",
                        mutation.id, mutation.source
                    )
                })
            }
            Err(error) => Some(format!("mutant {}: {error}", mutation.id)),
        })
        .collect()
}

const fn fixture_mutation(id: u8, needle: &'static str) -> ControlledMutation {
    ControlledMutation {
        id,
        predicate: "binding-audit-fixture",
        source: "fixture.rs",
        needle,
        replacement: "replacement",
        package: "fixture",
        target: MutationTarget::Library,
        selector: "fixture",
    }
}
