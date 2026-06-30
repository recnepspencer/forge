use worth_primitives::{truth_digest_parts, TruthDigestScope};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EvidenceLookupProductOutput {
    output_digest: String,
    execution_receipt_digest: String,
    evidence_receipt_digests: Vec<String>,
}

impl EvidenceLookupProductOutput {
    pub(crate) fn new(
        execution_receipt_digest: String,
        mut evidence_receipt_digests: Vec<String>,
    ) -> Self {
        evidence_receipt_digests.sort();
        evidence_receipt_digests.dedup();
        let output_digest = truth_digest_parts(
            TruthDigestScope::ArtifactIdentity,
            &std::iter::once("worth-spatial:evidence-lookup-product-output:v1".to_string())
                .chain(std::iter::once(format!(
                    "receipt:{execution_receipt_digest}"
                )))
                .chain(
                    evidence_receipt_digests
                        .iter()
                        .map(|digest| format!("evidence:{digest}")),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            output_digest,
            execution_receipt_digest,
            evidence_receipt_digests,
        }
    }

    pub fn output_digest(&self) -> &str {
        &self.output_digest
    }

    pub fn execution_receipt_digest(&self) -> &str {
        &self.execution_receipt_digest
    }

    pub fn evidence_receipt_digests(&self) -> &[String] {
        &self.evidence_receipt_digests
    }
}
