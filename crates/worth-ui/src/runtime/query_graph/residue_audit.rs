use forge_query::facade::consumer_kit::{
    query_consumer_residue_audit, ForgeQueryBoundaryAuditError, ForgeQueryConsumerResidueReport,
};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiQueryGraphAdoptionResidueFinding {
    source: String,
    reason: String,
}

pub struct WorthUiQueryGraphAdoptionResidueAudit;

impl WorthUiQueryGraphAdoptionResidueAudit {
    pub fn evaluate_query_owned_report(
    ) -> Result<ForgeQueryConsumerResidueReport, ForgeQueryBoundaryAuditError> {
        query_consumer_residue_audit("worth-ui")
            .required_root(
                std::path::Path::new(env!("CARGO_MANIFEST_DIR")).join("src/runtime/query_graph"),
            )
            .evaluate()
    }

    pub fn scan_source(source: &str) -> Vec<WorthUiQueryGraphAdoptionResidueFinding> {
        let residue_patterns = [
            (
                "WorthUiGraphObligationRegistry::select(",
                "local graph obligation selection cannot be final Query graph proof",
            ),
            (
                "select_graph_obligations_for_touch(",
                "local graph obligation selection cannot be final Query graph proof",
            ),
            (
                "WorthUiGraphTouchAdmission::admit(",
                "local graph touch admission cannot replace Query execution proof",
            ),
            (
                "WorthUiGraphTouchAdmission",
                "local graph touch admission types cannot be production proof surfaces",
            ),
            (
                "WorthUiGraphTouchDeclaration",
                "local graph touch declarations cannot create a second admission path",
            ),
            (
                "WorthUiGraphObligationPosture",
                "local graph obligation posture duplicates Query support and execution status",
            ),
            (
                "WorthUiGraphQueryPosture",
                "local graph query posture duplicates Query-owned support posture",
            ),
            (
                "admission_digest",
                "local graph admission digests cannot stand in for Query proof digests",
            ),
            (
                "graph_admission",
                "receipts must consume Query execution receipts rather than local graph admission",
            ),
        ];
        residue_patterns
            .iter()
            .filter(|(pattern, _)| source.contains(*pattern))
            .map(
                |(pattern, reason)| WorthUiQueryGraphAdoptionResidueFinding {
                    source: (*pattern).to_owned(),
                    reason: (*reason).to_owned(),
                },
            )
            .collect()
    }
}

impl WorthUiQueryGraphAdoptionResidueFinding {
    pub fn source(&self) -> &str {
        &self.source
    }

    pub fn reason(&self) -> &str {
        &self.reason
    }
}
