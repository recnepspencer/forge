#[test]
fn gate_d_open_rows_carry_no_stale_execution_truth() {
    let rows = super::parse(&super::ledger_document()).expect("the milestone ledger should parse");
    for requirement in ["P5-ATLAS-01", "P5-ATLAS-PINNING-01"] {
        let row = &rows[requirement];
        if row["result"] == "OPEN" {
            for (field, expected) in [
                ("final_source", "false"),
                ("matched_test_count", "0"),
                ("command_result", "not-run"),
                ("source_revision", "not-bound"),
                ("source_digest", "not-bound"),
                ("source_state_digest", "not-bound"),
                ("run_nonce", "not-bound"),
                ("result_artifact_digest", "not-bound"),
            ] {
                assert_eq!(row[field], expected, "{requirement} carries stale {field}");
            }
        }
        let mut premature = row.clone();
        for (field, value) in [
            ("matched_test_count", "0"),
            ("command_result", "not-run"),
            ("source_revision", "not-bound"),
            ("source_digest", "not-bound"),
            ("source_state_digest", "not-bound"),
            ("run_nonce", "not-bound"),
            ("result_artifact_digest", "not-bound"),
        ] {
            premature.insert(field.to_owned(), value.to_owned());
        }
        premature.insert("result".to_owned(), "PROVED".to_owned());
        premature.insert("final_source".to_owned(), "true".to_owned());
        assert!(
            super::validate_row(&premature).is_err(),
            "{requirement} accepted proof without a governed execution"
        );
    }
}
