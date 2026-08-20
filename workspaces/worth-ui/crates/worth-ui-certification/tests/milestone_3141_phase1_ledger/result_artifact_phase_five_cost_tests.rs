use serde_json::{json, Value};

use crate::milestone_3141_phase1_ledger::result_artifact_cost;

#[test]
fn phase_five_product_cost_is_derived_from_numeric_world_observations() {
    for (requirement, worlds, presentations) in [
        ("P5-TEXT-PIXELS-01", 1, 3),
        ("P5-TEXT-RECONSTRUCTION-01", 7, 21),
        ("P5-TEXT-COST-01", 32, 64),
        ("P5-TEXT-ASYNC-PRESENTATION-01", 1, 3),
    ] {
        let artifact = product_world_artifact(requirement, worlds, presentations);
        result_artifact_cost::validate(requirement, &artifact).unwrap();

        let mut mutant = artifact;
        mutant["construction_cost"] = Value::from(format!(
            "main-tests=1;hostile-controls=1;product-processes={};compile-sessions=0;courtroom-worlds={worlds}",
            worlds.saturating_sub(1)
        ));
        assert!(result_artifact_cost::validate(requirement, &mutant).is_err());
    }
}

#[test]
fn locality_cost_rejects_missing_duplicate_or_unmeasured_world_timings() {
    let artifact = product_world_artifact("P5-TEXT-COST-01", 32, 64);
    let mut missing = artifact.clone();
    missing["test_stdout"] =
        Value::from("WORTH_UI_LEDGER_WORLD=32\nWORTH_UI_LEDGER_PRESENTATIONS=64\n");
    assert!(result_artifact_cost::validate("P5-TEXT-COST-01", &missing).is_err());

    let mut duplicate = artifact;
    let stdout = duplicate["test_stdout"].as_str().unwrap();
    let payload = stdout
        .lines()
        .find_map(|line| line.strip_prefix("WORTH_UI_PHASE5_PRODUCTION_LOCALITY="))
        .unwrap();
    let mut rows: Vec<Value> = serde_json::from_str(payload).unwrap();
    rows[1] = rows[0].clone();
    duplicate["test_stdout"] = Value::from(format!(
        "WORTH_UI_PHASE5_PRODUCTION_LOCALITY={}\nWORTH_UI_LEDGER_WORLD=32\nWORTH_UI_LEDGER_PRESENTATIONS=64\n",
        Value::from(rows)
    ));
    assert!(result_artifact_cost::validate("P5-TEXT-COST-01", &duplicate).is_err());
}

fn product_world_artifact(requirement: &str, worlds: u64, presentations: u64) -> Value {
    let async_compile = requirement == "P5-TEXT-ASYNC-PRESENTATION-01";
    let compile_sessions = if async_compile { 2 } else { 0 };
    let source_identity = if async_compile {
        vec![Value::from(
            "_docs/worth-ui/milestone-3.14.1-evidence/compile-contracts.json",
        )]
    } else {
        Vec::new()
    };
    let locality = (requirement == "P5-TEXT-COST-01")
        .then(|| format!("WORTH_UI_PHASE5_PRODUCTION_LOCALITY={}\n", locality_rows()))
        .unwrap_or_default();
    json!({
        "hostile_control": {"executed_test_count": 1},
        "test_stdout": format!(
            "{locality}WORTH_UI_LEDGER_WORLD={worlds}\nWORTH_UI_LEDGER_PRESENTATIONS={presentations}\n"
        ),
        "source_identity": source_identity,
        "execution_receipts": [],
        "construction_cost": format!(
            "main-tests=1;hostile-controls=1;product-processes={worlds};compile-sessions={compile_sessions};courtroom-worlds={worlds}"
        ),
        "execution_cost": format!("executed-tests=2;presentations={presentations}")
    })
}

fn locality_rows() -> Value {
    let axes = [
        "content",
        "width",
        "paint-value",
        "paint-boundary",
        "dpi",
        "atlas-miss",
        "upload-completion",
        "pin-release",
    ];
    Value::from(
        [1_u64, 32, 2_048, 4_096]
            .into_iter()
            .flat_map(|retained| {
                axes.into_iter().map(move |axis| {
                    json!({
                        "retained": retained,
                        "axis": axis,
                        "world_elapsed_ms": 1,
                        "timing_us": {
                            "profile": 1,
                            "platform_prepare": 1,
                            "query_install": 1,
                            "fixture_materialization": 1,
                            "owner_installation": 1,
                            "builder_registration": 1,
                            "application_completion": 1,
                            "native_run": 1,
                        },
                    })
                })
            })
            .collect::<Vec<_>>(),
    )
}
