use crate::{ForgeQueryEvidenceIdentity, ForgeQueryEvidenceScope, ForgeQueryEvidenceTag};

use super::inventory::EffectReceiptArtifactKind;
use super::support_contract::EffectDeferredNeighborFamily;

const EFFECT_LIFECYCLE_IDENTITY_SCOPE: ForgeQueryEvidenceScope =
    ForgeQueryEvidenceScope::WorkflowMutationLowering;

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
    rule_identity: ForgeQueryEvidenceIdentity,
}

impl EffectReceiptTransitionRule {
    pub(super) fn new(
        kind: EffectReceiptTransitionKind,
        posture: EffectReceiptTransitionPosture,
        detail: &'static str,
        deferred_neighbor: Option<EffectDeferredNeighborFamily>,
    ) -> Self {
        let mut rule = ForgeQueryEvidenceIdentity::compose(EFFECT_LIFECYCLE_IDENTITY_SCOPE)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "effect_receipt_transition_rule_v1",
            )
            .field_shape(ForgeQueryEvidenceTag::new("kind"), kind.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("posture"), posture.as_str())
            .field_shape(ForgeQueryEvidenceTag::new("detail"), detail);
        rule = match deferred_neighbor {
            Some(neighbor) => {
                rule.field_shape(ForgeQueryEvidenceTag::new("neighbor"), neighbor.as_str())
            }
            None => rule.field_shape(ForgeQueryEvidenceTag::new("neighbor"), "none"),
        };
        let rule_identity = rule.seal();
        Self {
            kind,
            posture,
            detail,
            deferred_neighbor,
            rule_identity,
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

    pub fn rule_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.rule_identity
    }

    pub fn rule_for_reporting(&self) -> &str {
        self.rule_identity.as_str()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct EffectReceiptTransitionRules {
    receipt_family: EffectReceiptArtifactKind,
    rules: Vec<EffectReceiptTransitionRule>,
    rules_identity: ForgeQueryEvidenceIdentity,
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
        let rule_identities = rules
            .iter()
            .map(|rule| rule.rule_identity().clone())
            .collect::<Vec<_>>();
        let rules_identity = ForgeQueryEvidenceIdentity::compose(EFFECT_LIFECYCLE_IDENTITY_SCOPE)
            .field_shape(
                ForgeQueryEvidenceTag::new("identity_family"),
                "effect_receipt_transition_rules_v1",
            )
            .field_shape(
                ForgeQueryEvidenceTag::new("receipt_family"),
                receipt_family.as_str(),
            )
            .field_evidence_identity_sequence(ForgeQueryEvidenceTag::new("rules"), &rule_identities)
            .seal();
        Self {
            receipt_family,
            rules,
            rules_identity,
        }
    }

    pub fn receipt_family(&self) -> EffectReceiptArtifactKind {
        self.receipt_family
    }

    pub fn rules(&self) -> &[EffectReceiptTransitionRule] {
        &self.rules
    }

    pub fn rules_identity(&self) -> &ForgeQueryEvidenceIdentity {
        &self.rules_identity
    }

    pub fn rules_for_reporting(&self) -> &str {
        self.rules_identity.as_str()
    }
}
