use crate::identity::hash_parts;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionDeferredNeighborFamily {
    DurableReceiptReloadAndStoreParity,
    PortableReceiptExport,
}

impl ProjectionConsumptionDeferredNeighborFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DurableReceiptReloadAndStoreParity => "durable_receipt_reload_and_store_parity",
            Self::PortableReceiptExport => "portable_receipt_export",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionTransitionKind {
    InspectReceipt,
    DeriveEnvelope,
    DiscoverSupport,
    ReloadPersistedReceipt,
    ExportPortableReceipt,
}

impl ProjectionConsumptionTransitionKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::InspectReceipt => "inspect_receipt",
            Self::DeriveEnvelope => "derive_envelope",
            Self::DiscoverSupport => "discover_support",
            Self::ReloadPersistedReceipt => "reload_persisted_receipt",
            Self::ExportPortableReceipt => "export_portable_receipt",
        }
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ProjectionConsumptionTransitionPosture {
    Implemented,
    Deferred,
}

impl ProjectionConsumptionTransitionPosture {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Implemented => "implemented",
            Self::Deferred => "deferred",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionTransitionRule {
    kind: ProjectionConsumptionTransitionKind,
    posture: ProjectionConsumptionTransitionPosture,
    detail: &'static str,
    deferred_neighbor: Option<ProjectionConsumptionDeferredNeighborFamily>,
    rule_digest: String,
}

impl ProjectionConsumptionTransitionRule {
    fn new(
        kind: ProjectionConsumptionTransitionKind,
        posture: ProjectionConsumptionTransitionPosture,
        detail: &'static str,
        deferred_neighbor: Option<ProjectionConsumptionDeferredNeighborFamily>,
    ) -> Self {
        let rule_digest = hash_parts(&[
            "projection_consumption_transition_rule_v1".to_string(),
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

    pub fn kind(&self) -> ProjectionConsumptionTransitionKind {
        self.kind
    }

    pub fn posture(&self) -> ProjectionConsumptionTransitionPosture {
        self.posture
    }

    pub fn detail(&self) -> &str {
        self.detail
    }

    pub fn deferred_neighbor(&self) -> Option<ProjectionConsumptionDeferredNeighborFamily> {
        self.deferred_neighbor
    }

    pub fn rule_digest(&self) -> &str {
        &self.rule_digest
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ProjectionConsumptionTransitionRules {
    rules: Vec<ProjectionConsumptionTransitionRule>,
    rules_digest: String,
}

impl ProjectionConsumptionTransitionRules {
    pub(crate) fn current_phase_five_surface() -> Self {
        let rules = vec![
            ProjectionConsumptionTransitionRule::new(
                ProjectionConsumptionTransitionKind::InspectReceipt,
                ProjectionConsumptionTransitionPosture::Implemented,
                "the receipt is the canonical operational artifact for consumed-fact inspection",
                None,
            ),
            ProjectionConsumptionTransitionRule::new(
                ProjectionConsumptionTransitionKind::DeriveEnvelope,
                ProjectionConsumptionTransitionPosture::Implemented,
                "the self-describing projection-consumption envelope derives directly from the receipt",
                None,
            ),
            ProjectionConsumptionTransitionRule::new(
                ProjectionConsumptionTransitionKind::DiscoverSupport,
                ProjectionConsumptionTransitionPosture::Implemented,
                "the receipt exposes support posture and deferred-neighbor summary for downstream inspection",
                None,
            ),
            ProjectionConsumptionTransitionRule::new(
                ProjectionConsumptionTransitionKind::ReloadPersistedReceipt,
                ProjectionConsumptionTransitionPosture::Deferred,
                "durable receipt reload remains deferred to later store-backed milestones",
                Some(ProjectionConsumptionDeferredNeighborFamily::DurableReceiptReloadAndStoreParity),
            ),
            ProjectionConsumptionTransitionRule::new(
                ProjectionConsumptionTransitionKind::ExportPortableReceipt,
                ProjectionConsumptionTransitionPosture::Deferred,
                "portable receipt export remains deferred to later store-backed milestones",
                Some(ProjectionConsumptionDeferredNeighborFamily::PortableReceiptExport),
            ),
        ];
        let rules_digest = hash_parts(
            &std::iter::once("projection_consumption_transition_rules_v1".to_string())
                .chain(
                    rules
                        .iter()
                        .map(|rule| format!("rule:{}", rule.rule_digest())),
                )
                .collect::<Vec<_>>(),
        );
        Self {
            rules,
            rules_digest,
        }
    }

    pub fn rules(&self) -> &[ProjectionConsumptionTransitionRule] {
        &self.rules
    }

    pub fn rules_digest(&self) -> &str {
        &self.rules_digest
    }
}
