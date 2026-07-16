use crate::identity::hash_parts;
use crate::intent_admission::certification::manifests::{
    worth_query_intent_admission_crate_doc_example_targets,
    WorthQueryIntentAdmissionCrateDocExampleTarget,
};

const DOC_PATH: &str = "crates/worth-query/docs/execution/intent-admission.md";
const DOC_CONTENTS: &str = include_str!("../../../../docs/execution/intent-admission.md");
const COMMON_PATH_SNIPPET: &str =
    "let receipt = runtime\n    .intent(declaration)\n    .execute()?;";
const ADVANCED_REVIEW_SNIPPET: &str =
    "let review = runtime.intent(declaration).review()?;\n\nlet request = review.request();";
const ADVANCED_EXECUTE_SNIPPET: &str =
    "let admitted = runtime.intent(declaration).review()?.admit()?;";
const CONSUMER_SNIPPET: &str =
    "let consumer = receipt.consumer_inspection();\n\nlet outcome = consumer.map(|inspection| inspection.outcome_class());";
const BASIS_COMMON_PATH_SNIPPET: &str =
    "let scoped_basis = basis_lifecycle()\n    .current_head()\n    .observe()?;";
const PROJECTION_COMMON_PATH_SNIPPET: &str =
    "let read_completion = read_declaration\n    .using(read::current())\n    .run(&mut workspace)\n    .into_result()?;\nlet projection = read_completion\n    .consume_projection(read::project_facts().entity_identities());";
const READ_COMMON_PATH_SNIPPET: &str =
    "let read_result = workspace.read_family_intent(&family).execute()?;";
const READ_BASIS_COMMON_PATH_SNIPPET: &str =
    "let basis_result = workspace\n    .read_family_in_basis_context_intent(&family, &context)\n    .execute()?;";
const MUTATION_COMMON_PATH_SNIPPET: &str =
    "let write_receipt = runtime.write_intent(command).execute()?;";
const MUTATION_BATCH_COMMON_PATH_SNIPPET: &str =
    "let batch_receipt = runtime.write_batch_intent(commands).execute()?;";
const MUTATION_BATCH_ADVANCED_PATH_SNIPPET: &str =
    "let batch_review = runtime.write_batch_intent(commands).review()?;";
const LIVE_READ_COMMON_PATH_SNIPPET: &str =
    "let live_result = workspace.read_live_intent(&view).execute()?;";
const LIVE_READ_CONVENIENCE_SNIPPET: &str = "let live_rows = workspace.read(&view);";
const INSPECTION_COMMON_PATH_SNIPPET: &str =
    "let inspection_result = workspace.inspections()?.inspect_intent(&view).execute()?;";
const MATERIALIZATION_CONVENIENCE_SNIPPET: &str =
    "let materialization = workspace.materialize_result(&view)?;";
const INSPECTION_CONVENIENCE_SNIPPET: &str =
    "let inspection = workspace.inspections()?.inspect(&view)?;";
const ROUTING_COMMON_PATH_SNIPPET: &str =
    "let probe_result = runtime.probe_existing_intent(request).execute()?;";
const ROUTING_CONVENIENCE_SNIPPET: &str =
    "let workspace_probe = workspace.probe_existing_intent(request).execute()?;";
const BASIS_ADVANCED_PATH_SNIPPET: &str =
    "let scoped_inspection_basis = basis_lifecycle()\n    .current_head()\n    .inspect()?;";
const PROJECTION_ADVANCED_PATH_SNIPPET: &str =
    "let (authority, projection_warnings) = projection\n    .into_admitted()\n    .map_err(handle_projection_stop)?;";
const MUTATION_ADVANCED_PATH_SNIPPET: &str =
    "let write_review = runtime.write_intent(command).review()?;";
const LIVE_READ_ADVANCED_PATH_SNIPPET: &str =
    "let live_review = workspace.read_live_intent(&view).review()?;";
const INSPECTION_ADVANCED_PATH_SNIPPET: &str =
    "let inspection_review = workspace.inspections()?.inspect_intent(&view).review()?;";
const ROUTING_ADVANCED_PATH_SNIPPET: &str =
    "let probe_review = runtime.probe_existing_intent(request).review()?;";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionDocExampleRow {
    label: &'static str,
    path: &'static str,
    example_target_label: &'static str,
    example_target_path: &'static str,
    snippet_digest: String,
    row_digest: String,
}

impl WorthQueryIntentAdmissionDocExampleRow {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn example_target_label(&self) -> &'static str {
        self.example_target_label
    }

    pub fn example_target_path(&self) -> &'static str {
        self.example_target_path
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryIntentAdmissionDocExampleReport {
    rows: Vec<WorthQueryIntentAdmissionDocExampleRow>,
    crate_doc_example_digest: String,
}

impl WorthQueryIntentAdmissionDocExampleReport {
    pub fn rows(&self) -> &[WorthQueryIntentAdmissionDocExampleRow] {
        &self.rows
    }

    pub fn crate_doc_example_digest(&self) -> &str {
        &self.crate_doc_example_digest
    }
}

pub fn worth_query_intent_admission_doc_example_report() -> WorthQueryIntentAdmissionDocExampleReport
{
    let normalized_doc_contents = DOC_CONTENTS.replace("\r\n", "\n");
    let crate_doc_example_targets = worth_query_intent_admission_crate_doc_example_targets();
    let rows = vec![
        row(
            "common_path",
            COMMON_PATH_SNIPPET,
            crate_doc_example_targets[0],
            &normalized_doc_contents,
        ),
        row(
            "advanced_review",
            ADVANCED_REVIEW_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "advanced_execute",
            ADVANCED_EXECUTE_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "consumer_lane",
            CONSUMER_SNIPPET,
            crate_doc_example_targets[2],
            &normalized_doc_contents,
        ),
        row(
            "basis_common_path",
            BASIS_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[3],
            &normalized_doc_contents,
        ),
        row(
            "projection_common_path",
            PROJECTION_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[3],
            &normalized_doc_contents,
        ),
        row(
            "read_common_path",
            READ_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "read_basis_common_path",
            READ_BASIS_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "mutation_common_path",
            MUTATION_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "mutation_batch_common_path",
            MUTATION_BATCH_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "mutation_batch_advanced_path",
            MUTATION_BATCH_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "live_read_common_path",
            LIVE_READ_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "live_read_convenience",
            LIVE_READ_CONVENIENCE_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "inspection_common_path",
            INSPECTION_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "materialization_convenience",
            MATERIALIZATION_CONVENIENCE_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "inspection_convenience",
            INSPECTION_CONVENIENCE_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "routing_common_path",
            ROUTING_COMMON_PATH_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "routing_convenience",
            ROUTING_CONVENIENCE_SNIPPET,
            crate_doc_example_targets[4],
            &normalized_doc_contents,
        ),
        row(
            "basis_advanced_path",
            BASIS_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[3],
            &normalized_doc_contents,
        ),
        row(
            "projection_advanced_path",
            PROJECTION_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[3],
            &normalized_doc_contents,
        ),
        row(
            "mutation_advanced_path",
            MUTATION_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "live_read_advanced_path",
            LIVE_READ_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "inspection_advanced_path",
            INSPECTION_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
        row(
            "routing_advanced_path",
            ROUTING_ADVANCED_PATH_SNIPPET,
            crate_doc_example_targets[1],
            &normalized_doc_contents,
        ),
    ];
    let crate_doc_example_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(hash_parts(
                &crate_doc_example_targets
                    .iter()
                    .map(|target| format!("{}:{}", target.label(), target.path()))
                    .collect::<Vec<_>>(),
            )))
            .chain(std::iter::once(hash_parts(std::slice::from_ref(
                &normalized_doc_contents,
            ))))
            .collect::<Vec<_>>(),
    );
    WorthQueryIntentAdmissionDocExampleReport {
        rows,
        crate_doc_example_digest,
    }
}

fn row(
    label: &'static str,
    snippet: &'static str,
    example_target: WorthQueryIntentAdmissionCrateDocExampleTarget,
    normalized_doc_contents: &str,
) -> WorthQueryIntentAdmissionDocExampleRow {
    assert!(
        normalized_doc_contents.contains(snippet),
        "intent admission doc must preserve the `{label}` example snippet"
    );
    let snippet_digest = hash_parts(&[snippet.to_string()]);
    let row_digest = hash_parts(&[
        "worth_query_intent_admission_doc_example_row_v1".to_string(),
        format!("label:{label}"),
        format!("path:{DOC_PATH}"),
        format!("example_target_label:{}", example_target.label()),
        format!("example_target_path:{}", example_target.path()),
        format!("snippet:{snippet_digest}"),
    ]);
    WorthQueryIntentAdmissionDocExampleRow {
        label,
        path: DOC_PATH,
        example_target_label: example_target.label(),
        example_target_path: example_target.path(),
        snippet_digest,
        row_digest,
    }
}
