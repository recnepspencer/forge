use crate::evidence_identity::{
    worth_query_evidence_identity, WorthQueryEvidenceIdentity, WorthQueryEvidenceScope,
    WorthQueryEvidenceTag,
};

use super::super::error::WorthQueryGraphObligationDispatchError;
use super::super::kind::WorthQueryGraphObligationKind;
use super::context::WorthQueryGraphObligationDispatchContext;
use super::plan::WorthQueryGraphObligationDispatchPlan;

pub const WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME: &str =
    "worth-query.graph-obligation.dispatch-envelope.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDispatchEnvelope {
    context: WorthQueryGraphObligationDispatchContext,
    rows: Vec<WorthQueryGraphObligationDispatchPlan>,
    envelope_digest: WorthQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryGraphObligationDispatchEnvelopeBuilder {
    context: WorthQueryGraphObligationDispatchContext,
    rows: Vec<WorthQueryGraphObligationDispatchPlan>,
}

impl WorthQueryGraphObligationDispatchEnvelope {
    pub fn builder(
        context: WorthQueryGraphObligationDispatchContext,
    ) -> WorthQueryGraphObligationDispatchEnvelopeBuilder {
        WorthQueryGraphObligationDispatchEnvelopeBuilder {
            context,
            rows: Vec::new(),
        }
    }

    pub fn context(&self) -> &WorthQueryGraphObligationDispatchContext {
        &self.context
    }

    pub fn rows(&self) -> &[WorthQueryGraphObligationDispatchPlan] {
        &self.rows
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_str()
    }

    pub fn scheme(&self) -> &'static str {
        WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME
    }

    pub fn allow_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_allow())
            .count()
    }

    pub fn advisory_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_advisory())
            .count()
    }

    pub fn blocking_count(&self) -> usize {
        self.rows
            .iter()
            .filter(|row| row.verdict().is_blocking())
            .count()
    }

    pub fn kind_count(&self, kind: WorthQueryGraphObligationKind) -> usize {
        self.rows.iter().filter(|row| row.kind() == kind).count()
    }
}

impl WorthQueryGraphObligationDispatchEnvelopeBuilder {
    pub fn record(mut self, row: WorthQueryGraphObligationDispatchPlan) -> Self {
        self.rows.push(row);
        self
    }

    pub fn seal(
        mut self,
    ) -> Result<WorthQueryGraphObligationDispatchEnvelope, WorthQueryGraphObligationDispatchError>
    {
        if self.rows.is_empty() {
            return Err(WorthQueryGraphObligationDispatchError::EmptyEnvelope);
        }
        self.rows.sort_by(|left, right| {
            left.plan_digest().cmp(right.plan_digest()).then_with(|| {
                left.rule_identity()
                    .identity_digest()
                    .cmp(right.rule_identity().identity_digest())
            })
        });
        let row_digests = self
            .rows
            .iter()
            .map(WorthQueryGraphObligationDispatchPlan::plan_evidence_digest)
            .collect::<Vec<_>>();
        let envelope_digest =
            worth_query_evidence_identity(WorthQueryEvidenceScope::GraphObligationDispatchEnvelope)
                .field_shape(
                    WorthQueryEvidenceTag::new("scheme"),
                    WORTH_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
                )
                .field_evidence_identity(
                    WorthQueryEvidenceTag::new("context"),
                    self.context.context_evidence_digest(),
                )
                .field_usize(WorthQueryEvidenceTag::new("rows"), self.rows.len())
                .field_usize(
                    WorthQueryEvidenceTag::new("allow"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_allow())
                        .count(),
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("advise"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_advisory())
                        .count(),
                )
                .field_usize(
                    WorthQueryEvidenceTag::new("block"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_blocking())
                        .count(),
                )
                .field_evidence_identity_sequence(
                    WorthQueryEvidenceTag::new("dispatch_row"),
                    row_digests,
                )
                .seal();
        Ok(WorthQueryGraphObligationDispatchEnvelope {
            context: self.context,
            rows: self.rows,
            envelope_digest,
        })
    }
}
