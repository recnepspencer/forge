use std::collections::BTreeSet;

use crate::runtime::{
    WorthUiAdmittedCompositionGraphReceipt, WorthUiAuthoredCompositionDeclaration,
    WorthUiAuthoredCompositionEdgeDeclaration, WorthUiAuthoredCompositionNodeDeclaration,
    WorthUiAuthoredCompositionPolicyDeclaration, WorthUiCompositionNodeKind,
    WorthUiPrimitiveSourceSpan,
};

use super::super::composition_source_admission::{
    WorthUiCompositionSourceAdmissionCounters, WorthUiCompositionSourceAdmissionDenial,
    WorthUiCompositionSourceAdmissionReport, WorthUiCompositionSourceDenialCode,
};

impl WorthUiAuthoredCompositionDeclaration {
    pub(crate) fn admit_source(
        &self,
        control_ids: &BTreeSet<String>,
        interaction_ids: &BTreeSet<String>,
    ) -> Result<WorthUiAdmittedCompositionGraphReceipt, WorthUiCompositionSourceAdmissionReport>
    {
        let mut denials = self.authority_denials(control_ids, interaction_ids);
        if let Err(graph_denials) = self.admit() {
            denials.extend(graph_denials.into_iter().map(|denial| {
                let source_span = self.source_span_for_subject(denial.subject());
                WorthUiCompositionSourceAdmissionDenial::graph(denial, source_span)
            }));
        }
        if denials.is_empty() {
            return Ok(self
                .admit()
                .expect("composition source was just admitted without denials"));
        }
        denials.sort_by(|left, right| {
            left.source_span_key()
                .cmp(&right.source_span_key())
                .then_with(|| left.subject().cmp(right.subject()))
                .then_with(|| left.code().token().cmp(right.code().token()))
        });
        Err(WorthUiCompositionSourceAdmissionReport::denied(
            denials,
            WorthUiCompositionSourceAdmissionCounters::new(
                self.nodes().len(),
                self.edges().len(),
                self.policies().len(),
            ),
        ))
    }

    fn authority_denials(
        &self,
        control_ids: &BTreeSet<String>,
        interaction_ids: &BTreeSet<String>,
    ) -> Vec<WorthUiCompositionSourceAdmissionDenial> {
        self.nodes()
            .iter()
            .filter_map(|node| match node.kind() {
                WorthUiCompositionNodeKind::Control => {
                    let control_id = node.node_id().strip_prefix("live_view.control.")?;
                    (!control_ids.contains(control_id)).then(|| {
                        WorthUiCompositionSourceAdmissionDenial::new(
                            WorthUiCompositionSourceDenialCode::StaleControlReference,
                            node.node_id(),
                            "composition control child must reference an authored control",
                            "child control <authored-control-id>",
                            node.source_span(),
                        )
                    })
                }
                WorthUiCompositionNodeKind::Interaction => {
                    let interaction_id = node.node_id().strip_prefix("live_view.interaction.")?;
                    (!interaction_ids.contains(interaction_id)).then(|| {
                        WorthUiCompositionSourceAdmissionDenial::new(
                            WorthUiCompositionSourceDenialCode::StaleInteractionReference,
                            node.node_id(),
                            "composition interaction child must reference an authored interaction",
                            "child interaction <authored-interaction-id>",
                            node.source_span(),
                        )
                    })
                }
                _ => None,
            })
            .collect()
    }

    fn source_span_for_subject(&self, subject: &str) -> Option<WorthUiPrimitiveSourceSpan> {
        self.nodes()
            .iter()
            .find(|node| node.node_id() == subject)
            .and_then(WorthUiAuthoredCompositionNodeDeclaration::source_span)
            .or_else(|| self.edge_source_span_for_subject(subject))
            .or_else(|| self.policy_source_span_for_subject(subject))
    }

    fn edge_source_span_for_subject(&self, subject: &str) -> Option<WorthUiPrimitiveSourceSpan> {
        self.edges()
            .iter()
            .find(|edge| edge.child_id() == subject || edge.parent_id() == Some(subject))
            .and_then(WorthUiAuthoredCompositionEdgeDeclaration::source_span)
    }

    fn policy_source_span_for_subject(&self, subject: &str) -> Option<WorthUiPrimitiveSourceSpan> {
        self.policies()
            .iter()
            .find(|policy| policy.node_id() == subject)
            .and_then(WorthUiAuthoredCompositionPolicyDeclaration::source_span)
    }
}
