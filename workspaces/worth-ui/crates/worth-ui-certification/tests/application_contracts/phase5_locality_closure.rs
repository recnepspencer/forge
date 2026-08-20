use std::collections::BTreeSet;
use std::process::{Command, Stdio};

use worth_ui_certification::scenario::phase5_locality_matrix::cost_hostile_cases_for_axis;

const EVIDENCE_PREFIX: &str = "WORTH_UI_PHASE5_PRODUCTION_LOCALITY=";

#[test]
#[ignore = "closure portfolio: 32 fresh native worlds use bounded parallel shards"]
fn all_32_fresh_native_locality_worlds_retain_owner_issued_evidence() {
    let output = Command::new(env!("CARGO_BIN_EXE_worth-ui-phase5-locality-matrix"))
        .stderr(Stdio::inherit())
        .output()
        .expect("the prebuilt Phase 5 locality runner launches");
    assert!(output.status.success(), "Phase 5 locality runner failed");
    let stdout = String::from_utf8(output.stdout).expect("locality evidence is UTF-8");
    let payload = stdout
        .lines()
        .find_map(|line| line.strip_prefix(EVIDENCE_PREFIX))
        .expect("locality runner emitted no retained evidence");
    println!("{EVIDENCE_PREFIX}{payload}");
    let rows: Vec<serde_json::Value> =
        serde_json::from_str(payload).expect("locality evidence is valid JSON");
    assert_eq!(rows.len(), 32);

    let cases = rows
        .iter()
        .map(|row| {
            assert_eq!(row["terminal_zero"], true);
            let retained = row["retained"].as_u64().expect("retained size");
            let paragraphs = row["retained_paragraphs"]
                .as_u64()
                .expect("retained paragraph count");
            let mechanics = row["retained_mechanics"]
                .as_u64()
                .expect("retained mechanic count");
            let initial_mechanics = row["text_work"][0]["layouts"]
                .as_u64()
                .expect("initial retained text mechanics");
            assert_eq!(mechanics, if retained == 1 { 2 } else { retained });
            assert_eq!(paragraphs, if retained == 1 { 1 } else { retained / 2 });
            assert_eq!(
                initial_mechanics, mechanics,
                "matrix scale must reach the qualified retained-mechanic ceiling",
            );
            assert!(row["query_completed"]
                .as_u64()
                .is_some_and(|count| count >= 2));
            assert!(row["semantic_frontiers"]
                .as_array()
                .is_some_and(|frontiers| !frontiers.is_empty()));
            assert!(row["physical_signal"]["performed_nodes"]
                .as_u64()
                .is_some_and(|performed| performed > 0));
            assert!(row["world_elapsed_ms"].as_u64().is_some());
            for phase in [
                "profile",
                "platform_prepare",
                "query_install",
                "fixture_materialization",
                "owner_installation",
                "builder_registration",
                "application_completion",
                "native_run",
            ] {
                assert!(
                    row["timing_us"][phase].as_u64().is_some(),
                    "matrix row omits timing phase {phase}"
                );
            }
            let axis = row["axis"].as_str().expect("locality axis");
            let convictions = row["hostile_cost_convictions"]
                .as_array()
                .expect("matrix row omits hostile cost convictions");
            let observed = convictions
                .iter()
                .map(|conviction| {
                    assert!(conviction["performed_work"].as_u64().is_some());
                    assert!(conviction["mutant_work"].as_u64().is_some());
                    assert!(conviction["denial"].as_str().is_some());
                    conviction["mutant"].as_str().expect("mutant name")
                })
                .collect::<Vec<_>>();
            assert_eq!(
                observed,
                cost_hostile_cases_for_axis(axis, retained as usize).expect("known locality axis")
            );
            (retained, axis.to_owned())
        })
        .collect::<BTreeSet<_>>();
    let expected = [1_u64, 32, 2_048, 4_096]
        .into_iter()
        .flat_map(|retained| {
            [
                "content",
                "width",
                "paint-value",
                "paint-boundary",
                "dpi",
                "atlas-miss",
                "upload-completion",
                "pin-release",
            ]
            .into_iter()
            .map(move |axis| (retained, axis.to_owned()))
        })
        .collect::<BTreeSet<_>>();
    assert_eq!(cases, expected);
    let worlds = rows.len();
    let presentations = rows
        .iter()
        .map(|row| row["query_completed"].as_u64().unwrap())
        .sum::<u64>();
    println!("WORTH_UI_LEDGER_COUNTERS={{\"P5-TEXT-COST-01\":{worlds}}}");
    println!("WORTH_UI_LEDGER_PRESENTATIONS={presentations}");
    println!("WORTH_UI_LEDGER_WORLD={worlds}");
}
