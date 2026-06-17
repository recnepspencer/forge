use super::{
    ForgeQueryPublicBridgeForbiddenAccessFinding, ForgeQueryPublicBridgeReaderLaneInventory,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ForgeQueryPublicBridgeReaderLaneSabotageKind {
    DirectMaterializationRead,
}

impl ForgeQueryPublicBridgeReaderLaneSabotageKind {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::DirectMaterializationRead => "direct_materialization_read",
        }
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryPublicBridgeReaderLaneSabotage {
    kind: ForgeQueryPublicBridgeReaderLaneSabotageKind,
    outcome: ForgeQueryPublicBridgeReaderLaneSabotageOutcome,
}

impl ForgeQueryPublicBridgeReaderLaneSabotage {
    pub fn evaluate_direct_materialization_read(
        inventory: &ForgeQueryPublicBridgeReaderLaneInventory,
    ) -> Self {
        let outcome = inventory
            .forbidden_findings()
            .first()
            .cloned()
            .map(ForgeQueryPublicBridgeReaderLaneSabotageOutcome::Rejected)
            .unwrap_or(ForgeQueryPublicBridgeReaderLaneSabotageOutcome::Missed);
        Self {
            kind: ForgeQueryPublicBridgeReaderLaneSabotageKind::DirectMaterializationRead,
            outcome,
        }
    }

    pub fn kind(&self) -> ForgeQueryPublicBridgeReaderLaneSabotageKind {
        self.kind
    }

    pub fn localized_pattern(&self) -> &str {
        match &self.outcome {
            ForgeQueryPublicBridgeReaderLaneSabotageOutcome::Rejected(finding) => {
                finding.matched_text()
            }
            ForgeQueryPublicBridgeReaderLaneSabotageOutcome::Missed => "none",
        }
    }

    pub fn rejected(&self) -> bool {
        matches!(
            self.outcome,
            ForgeQueryPublicBridgeReaderLaneSabotageOutcome::Rejected(_)
        )
    }

    pub fn outcome(&self) -> &ForgeQueryPublicBridgeReaderLaneSabotageOutcome {
        &self.outcome
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ForgeQueryPublicBridgeReaderLaneSabotageOutcome {
    Rejected(ForgeQueryPublicBridgeForbiddenAccessFinding),
    Missed,
}
