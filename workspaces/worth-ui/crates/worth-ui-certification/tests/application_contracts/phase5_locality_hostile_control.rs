use std::process::{Command, Stdio};

use worth_ui_certification::scenario::phase5_locality_matrix::{
    cost_hostile_cases_for_axis, COST_HOSTILE_CASES,
};

const EVIDENCE_PREFIX: &str = "WORTH_UI_PHASE5_PRODUCTION_LOCALITY=";

#[test]
fn exact_owner_cost_mutants_are_convicted_by_performed_small_worlds() {
    let cases = [
        "content:32",
        "paint-value:1",
        "dpi:1",
        "upload-completion:1",
    ];
    let mut convictions = Vec::new();
    for case in cases {
        let output = Command::new(env!("CARGO_BIN_EXE_worth-ui-phase5-locality-matrix"))
            .env("WORTH_UI_PHASE5_MATRIX_CASE", case)
            .stderr(Stdio::inherit())
            .output()
            .expect("the prebuilt Phase 5 locality runner launches");
        assert!(
            output.status.success(),
            "hostile locality case {case} failed"
        );
        let stdout = String::from_utf8(output.stdout).expect("locality evidence is UTF-8");
        let payload = stdout
            .lines()
            .find_map(|line| line.strip_prefix(EVIDENCE_PREFIX))
            .expect("hostile locality case emitted no owner evidence");
        let rows: Vec<serde_json::Value> =
            serde_json::from_str(payload).expect("hostile locality evidence is valid JSON");
        let [row] = rows.as_slice() else {
            panic!("hostile locality case must emit one row");
        };
        assert_eq!(row["terminal_zero"], true);
        let axis = row["axis"].as_str().expect("hostile row axis");
        let row_convictions = row["hostile_cost_convictions"]
            .as_array()
            .expect("owner evidence omits hostile convictions");
        let row_names = row_convictions
            .iter()
            .map(|conviction| {
                assert_ne!(conviction["performed_work"], conviction["mutant_work"]);
                assert_ne!(
                    conviction["performed_trace_digest"],
                    conviction["mutant_trace_digest"]
                );
                assert_eq!(
                    conviction["performed_trace_digest"]
                        .as_str()
                        .expect("performed trace digest")
                        .len(),
                    64
                );
                assert_eq!(
                    conviction["mutant_trace_digest"]
                        .as_str()
                        .expect("mutant trace digest")
                        .len(),
                    64
                );
                assert!(conviction["denial"].as_str().is_some());
                conviction["mutant"].as_str().expect("mutant name")
            })
            .collect::<Vec<_>>();
        assert_eq!(
            row_names,
            cost_hostile_cases_for_axis(
                axis,
                row["retained"].as_u64().expect("hostile retained size") as usize,
            )
            .expect("known hostile axis")
        );
        convictions.extend(row_names.into_iter().map(str::to_owned));
    }
    for expected in COST_HOSTILE_CASES {
        assert!(
            convictions.iter().any(|observed| observed == expected),
            "hostile mutant {expected} was not causally exercised"
        );
    }
    println!(
        "WORTH_UI_LEDGER_MUTATION_CONTROLS={{\"P5-TEXT-COST-01\":\"complete-document-rescan\"}}"
    );
    println!(
        "WORTH_UI_LEDGER_MUTATION_CASES={{\"P5-TEXT-COST-01\":{}}}",
        serde_json::to_string(&COST_HOSTILE_CASES).unwrap()
    );
}
