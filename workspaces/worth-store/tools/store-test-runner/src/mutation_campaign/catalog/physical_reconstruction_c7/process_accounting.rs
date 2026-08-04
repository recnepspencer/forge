use super::super::{ControlledMutation, MutationTarget};

pub(super) const MUTATIONS: &[ControlledMutation] = &[
    ControlledMutation {
        id: 115,
        predicate: "c7-termination-process-accounting-omitted",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/c7_crash_campaign/process_accounting.rs",
        needle: "            if process_ids.len() != PROCESSES_PER_CASE {",
        replacement: "            if false {",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::c7_crash_campaign::process_accounting::tests::repeated_process_identity_within_one_case_is_rejected",
    },
    ControlledMutation {
        id: 116,
        predicate: "c7-termination-process-report-omitted",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/evidence_projection/c7_crash_campaign.rs",
        needle: "        \"processes\": process_values(campaign.processes()),\n",
        replacement: "",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::evidence_projection::tests::c7_report_schema_requires_termination_process_projection",
    },
    ControlledMutation {
        id: 117,
        predicate: "c7-ordinary-process-scope-extended",
        source: "tools/store-test-runner/src/courtroom_campaign/bounded_residency_siege/oracle.rs",
        needle: "    if processes.len() != expected.len() {",
        replacement: "    if false {",
        package: "store-test-runner",
        target: MutationTarget::LibraryWithFeatures {
            features: "physical-work-evidence",
        },
        selector: "courtroom_campaign::bounded_residency_siege::oracle::tests::ordinary_process_accounting_rejects_campaign_extension",
    },
];
