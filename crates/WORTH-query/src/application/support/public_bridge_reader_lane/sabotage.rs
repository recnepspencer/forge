use super::{
    WorthQueryPublicBridgeForbiddenAccessFinding, WorthQueryPublicBridgeReaderLaneInventory,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicBridgeReaderLaneSabotageKind {
    DirectMaterializationRead,
}

impl WorthQueryPublicBridgeReaderLaneSabotageKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectMaterializationRead => "direct_materialization_read",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryPublicBridgeReaderLaneSabotage {
    kind: WorthQueryPublicBridgeReaderLaneSabotageKind,
    outcome: WorthQueryPublicBridgeReaderLaneSabotageOutcome,
}

impl WorthQueryPublicBridgeReaderLaneSabotage {
    pub fn evaluate_direct_materialization_read(
        inventory: &WorthQueryPublicBridgeReaderLaneInventory,
    ) -> Self {
        let outcome = inventory
            .forbidden_findings()
            .first()
            .cloned()
            .map(WorthQueryPublicBridgeReaderLaneSabotageOutcome::Rejected)
            .unwrap_or(WorthQueryPublicBridgeReaderLaneSabotageOutcome::Missed);
        Self {
            kind: WorthQueryPublicBridgeReaderLaneSabotageKind::DirectMaterializationRead,
            outcome,
        }
    }

    pub fn kind(&self) -> WorthQueryPublicBridgeReaderLaneSabotageKind {
        self.kind
    }

    pub fn localized_pattern(&self) -> &str {
        match &self.outcome {
            WorthQueryPublicBridgeReaderLaneSabotageOutcome::Rejected(finding) => {
                finding.matched_text()
            }
            WorthQueryPublicBridgeReaderLaneSabotageOutcome::Missed => "none",
        }
    }

    pub fn rejected(&self) -> bool {
        matches!(
            self.outcome,
            WorthQueryPublicBridgeReaderLaneSabotageOutcome::Rejected(_)
        )
    }

    pub fn outcome(&self) -> &WorthQueryPublicBridgeReaderLaneSabotageOutcome {
        &self.outcome
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum WorthQueryPublicBridgeReaderLaneSabotageOutcome {
    Rejected(WorthQueryPublicBridgeForbiddenAccessFinding),
    Missed,
}
