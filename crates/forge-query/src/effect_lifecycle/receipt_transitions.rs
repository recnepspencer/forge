use crate::identity::hash_parts;

use super::inventory::EffectReceiptArtifactKind;
use super::support_contract::EffectDeferredNeighborFamily;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectReceiptTransitionKind {
    InspectReceipt,
    DeriveEnvelope,
    MaterializeDiagnostics,
    ProjectMaterializedFacts,
    ReplayExecution,
    ExportPortableReceipt,
}

impl EffectReceiptTransitionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectReceipt => "inspect_receipt",
            Self::DeriveEnvelope => "derive_envelope",
            Self::MaterializeDiagnostics => "materialize_diagnostics",
            Self::ProjectMaterializedFacts => "project_materialized_facts",
            Self::ReplayExecution => "replay_execution",
            Self::ExportPortableReceipt => "export_portable_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum EffectReceiptTransitionPosture {
    Implemented,
    Deferred,
}

impl EffectReceiptTransitionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptTransitionRule {
    kind: EffectReceiptTransitionKind,
    posture: EffectReceiptTransitionPosture,
    detail: &'static str,
    deferred_neighbor: Option<EffectDeferredNeighborFamily>,
    rule_digest: String,
}

impl EffectReceiptTransitionRule {
    pub(super) fn new(
        kind: EffectReceiptTransitionKind,
        posture: EffectReceiptTransitionPosture,
        detail: &'static str,
        deferred_neighbor: Option<EffectDeferredNeighborFamily>,
    ) -> Self {
        let rule_digest = hash_parts(&[
            "effect_receipt_transition_rule_v1".to_string(),
            format!("kind:{}", kind.as_str()),
            format!("posture:{}", posture.as_str()),
            format!("detail:{detail}"),
            format!(
                "neighbor:{}",
                deferred_neighbor
                    .map(|neighbor| neighbor.as_str())
                    .unwrap_or("none")
            ),
        ]);
        Self {
            kind,
            posture,
            detail,
            deferred_neighbor,
            rule_digest,
        }
    }

    pub fn kind(&self) -> EffectReceiptTransitionKind {
        self.kind
    }

    pub fn posture(&self) -> EffectReceiptTransitionPosture {
        self.posture
    }

    pub fn detail(&self) -> &str {
        self.detail
    }

    pub fn deferred_neighbor(&self) -> Option<EffectDeferredNeighborFamily> {
        self.deferred_neighbor
    }

    pub fn rule_digest(&self) -> &str {
        &self.rule_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptTransitionRules {
    receipt_family: EffectReceiptArtifactKind,
    rules: Vec<EffectReceiptTransitionRule>,
    rules_digest: String,
}

impl EffectReceiptTransitionRules {
    pub(super) fn for_receipt_family(receipt_family: EffectReceiptArtifactKind) -> Self {
        let rules = vec![
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::InspectReceipt,
                EffectReceiptTransitionPosture::Implemented,
                "the receipt is the canonical operational artifact for post-execution inspection",
                None,
            ),
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::DeriveEnvelope,
                EffectReceiptTransitionPosture::Implemented,
                "the self-describing envelope derives directly from the receipt",
                None,
            ),
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::MaterializeDiagnostics,
                EffectReceiptTransitionPosture::Implemented,
                "diagnostics materialization derives from receipt and envelope evidence",
                None,
            ),
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::ProjectMaterializedFacts,
                EffectReceiptTransitionPosture::Deferred,
                "projection fact receipts remain owned by milestone 9.3.4",
                None,
            ),
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::ReplayExecution,
                EffectReceiptTransitionPosture::Deferred,
                "durable effect replay remains deferred to later store-backed milestones",
                Some(EffectDeferredNeighborFamily::DurableReplayAndRestartStableEnvelope),
            ),
            EffectReceiptTransitionRule::new(
                EffectReceiptTransitionKind::ExportPortableReceipt,
                EffectReceiptTransitionPosture::Deferred,
                "portable receipt import/export remains deferred to later store-backed milestones",
                Some(EffectDeferredNeighborFamily::StoreBackedExecutionParity),
            ),
        ];
        let rules_digest = hash_parts(
            &std::iter::once("effect_receipt_transition_rules_v1".to_string())
                .chain(std::iter::once(format!(
                    "receipt_family:{}",
                    receipt_family.as_str()
                )))
                .chain(
                    rules
                        .iter()
                        .map(|rule| format!("rule:{}", rule.rule_digest())),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            receipt_family,
            rules,
            rules_digest,
        }
    }

    pub fn receipt_family(&self) -> EffectReceiptArtifactKind {
        self.receipt_family
    }

    pub fn rules(&self) -> &[EffectReceiptTransitionRule] {
        &self.rules
    }

    pub fn rules_digest(&self) -> &str {
        &self.rules_digest
    }
}
