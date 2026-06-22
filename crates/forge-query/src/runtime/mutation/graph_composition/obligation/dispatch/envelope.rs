use crate::evidence_identity::{
    forge_query_evidence_identity, ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope,
    ForgeQueryEvidenceTag,
};

use super::super::error::ForgeQueryGraphObligationDispatchError;
use super::super::kind::ForgeQueryGraphObligationKind;
use super::context::ForgeQueryGraphObligationDispatchContext;
use super::plan::ForgeQueryGraphObligationDispatchPlan;

pub const FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME: &str =
    "forge-query.graph-obligation.dispatch-envelope.v1";

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationDispatchEnvelope {
    context: ForgeQueryGraphObligationDispatchContext,
    rows: Vec<ForgeQueryGraphObligationDispatchPlan>,
    envelope_digest: ForgeQueryEvidenceIdentity,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphObligationDispatchEnvelopeBuilder {
    context: ForgeQueryGraphObligationDispatchContext,
    rows: Vec<ForgeQueryGraphObligationDispatchPlan>,
}

impl ForgeQueryGraphObligationDispatchEnvelope {
    pub fn builder(
        context: ForgeQueryGraphObligationDispatchContext,
    ) -> ForgeQueryGraphObligationDispatchEnvelopeBuilder {
        ForgeQueryGraphObligationDispatchEnvelopeBuilder {
            context,
            rows: Vec::new(),
        }
    }

    pub fn context(&self) -> &ForgeQueryGraphObligationDispatchContext {
        &self.context
    }

    pub fn rows(&self) -> &[ForgeQueryGraphObligationDispatchPlan] {
        &self.rows
    }

    pub fn envelope_digest(&self) -> &str {
        self.envelope_digest.as_str()
    }

    pub fn scheme(&self) -> &'static str {
        FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME
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

    pub fn kind_count(&self, kind: ForgeQueryGraphObligationKind) -> usize {
        self.rows.iter().filter(|row| row.kind() == kind).count()
    }
}

impl ForgeQueryGraphObligationDispatchEnvelopeBuilder {
    pub fn record(mut self, row: ForgeQueryGraphObligationDispatchPlan) -> Self {
        self.rows.push(row);
        self
    }

    pub fn seal(
        mut self,
    ) -> Result<ForgeQueryGraphObligationDispatchEnvelope, ForgeQueryGraphObligationDispatchError>
    {
        if self.rows.is_empty() {
            return Err(ForgeQueryGraphObligationDispatchError::EmptyEnvelope);
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
            .map(ForgeQueryGraphObligationDispatchPlan::plan_evidence_digest)
            .collect::<Vec<_>>();
        let envelope_digest =
            forge_query_evidence_identity(ForgeQueryEvidenceScope::GraphObligationDispatchEnvelope)
                .field_shape(
                    ForgeQueryEvidenceTag::new("scheme"),
                    FORGE_QUERY_GRAPH_OBLIGATION_DISPATCH_ENVELOPE_SCHEME,
                )
                .field_evidence_identity(
                    ForgeQueryEvidenceTag::new("context"),
                    self.context.context_evidence_digest(),
                )
                .field_usize(ForgeQueryEvidenceTag::new("rows"), self.rows.len())
                .field_usize(
                    ForgeQueryEvidenceTag::new("allow"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_allow())
                        .count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("advise"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_advisory())
                        .count(),
                )
                .field_usize(
                    ForgeQueryEvidenceTag::new("block"),
                    self.rows
                        .iter()
                        .filter(|row| row.verdict().is_blocking())
                        .count(),
                )
                .field_evidence_identity_sequence(
                    ForgeQueryEvidenceTag::new("dispatch_row"),
                    row_digests,
                )
                .seal();
        Ok(ForgeQueryGraphObligationDispatchEnvelope {
            context: self.context,
            rows: self.rows,
            envelope_digest,
        })
    }
}
