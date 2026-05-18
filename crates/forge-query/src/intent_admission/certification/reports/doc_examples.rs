use crate::identity::hash_parts;

const DOC_PATH: &str = "crates/forge-query/docs/intent-admission.md";
const DOC_CONTENTS: &str = include_str!("../../../../docs/intent-admission.md");
const COMMON_PATH_SNIPPET: &str =
    "let receipt = runtime\n    .intent(declaration)\n    .execute()?;";
const ADVANCED_REVIEW_SNIPPET: &str =
    "let review = runtime.intent(declaration).review()?;\n\nlet request = review.request();";
const ADVANCED_EXECUTE_SNIPPET: &str =
    "let admitted = runtime.intent(declaration).review()?.admit()?;";
const CONSUMER_SNIPPET: &str =
    "let consumer = receipt.consumer_inspection();\n\nlet outcome = consumer.outcome_class();";
const BASIS_COMMON_PATH_SNIPPET: &str = "let scoped_basis = forge_query_basis_observation_intent(\n    RawBasisIntent::CurrentHead,\n)?\n.admit()?\n.scope();";
const PROJECTION_COMMON_PATH_SNIPPET: &str =
    "let contract = forge_query_projection_consumption_intent(declaration)?\n    .admit()?\n    .bind_contract();";
const READ_COMMON_PATH_SNIPPET: &str =
    "let read_result = workspace.read_family_intent(&family).execute()?;";
const READ_BASIS_COMMON_PATH_SNIPPET: &str =
    "let basis_result = workspace\n    .read_family_in_basis_context_intent(&family, &context)\n    .execute()?;";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionDocExampleRow {
    label: &'static str,
    path: &'static str,
    snippet_digest: String,
    row_digest: String,
}

impl ForgeQueryIntentAdmissionDocExampleRow {
    pub fn label(&self) -> &'static str {
        self.label
    }

    pub fn path(&self) -> &'static str {
        self.path
    }

    pub fn row_digest(&self) -> &str {
        &self.row_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryIntentAdmissionDocExampleReport {
    rows: Vec<ForgeQueryIntentAdmissionDocExampleRow>,
    crate_doc_example_digest: String,
}

impl ForgeQueryIntentAdmissionDocExampleReport {
    pub fn rows(&self) -> &[ForgeQueryIntentAdmissionDocExampleRow] {
        &self.rows
    }

    pub fn crate_doc_example_digest(&self) -> &str {
        &self.crate_doc_example_digest
    }
}

pub fn forge_query_intent_admission_doc_example_report() -> ForgeQueryIntentAdmissionDocExampleReport
{
    let rows = vec![
        row("common_path", COMMON_PATH_SNIPPET),
        row("advanced_review", ADVANCED_REVIEW_SNIPPET),
        row("advanced_execute", ADVANCED_EXECUTE_SNIPPET),
        row("consumer_lane", CONSUMER_SNIPPET),
        row("basis_common_path", BASIS_COMMON_PATH_SNIPPET),
        row("projection_common_path", PROJECTION_COMMON_PATH_SNIPPET),
        row("read_common_path", READ_COMMON_PATH_SNIPPET),
        row("read_basis_common_path", READ_BASIS_COMMON_PATH_SNIPPET),
    ];
    let crate_doc_example_digest = hash_parts(
        &rows
            .iter()
            .map(|row| row.row_digest().to_string())
            .chain(std::iter::once(hash_parts(&[DOC_CONTENTS.to_string()])))
            .collect::<Vec<_>>(),
    );
    ForgeQueryIntentAdmissionDocExampleReport {
        rows,
        crate_doc_example_digest,
    }
}

fn row(label: &'static str, snippet: &'static str) -> ForgeQueryIntentAdmissionDocExampleRow {
    assert!(
        DOC_CONTENTS.contains(snippet),
        "intent admission doc must preserve the `{label}` example snippet"
    );
    let snippet_digest = hash_parts(&[snippet.to_string()]);
    let row_digest = hash_parts(&[
        "forge_query_intent_admission_doc_example_row_v1".to_string(),
        format!("label:{label}"),
        format!("path:{DOC_PATH}"),
        format!("snippet:{snippet_digest}"),
    ]);
    ForgeQueryIntentAdmissionDocExampleRow {
        label,
        path: DOC_PATH,
        snippet_digest,
        row_digest,
    }
}
