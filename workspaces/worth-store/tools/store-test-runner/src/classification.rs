use std::fmt;
use std::str::FromStr;

use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, Eq, PartialEq, Ord, PartialOrd, Serialize, Deserialize)]
#[serde(rename_all = "kebab-case")]
pub(crate) enum CiTestLane {
    OwnerUnit,
    Scenario,
    Ui,
    Formal,
    Structural,
}

impl CiTestLane {
    pub(crate) fn as_str(self) -> &'static str {
        match self {
            Self::OwnerUnit => "owner-unit",
            Self::Scenario => "scenario",
            Self::Ui => "ui",
            Self::Formal => "formal",
            Self::Structural => "structural",
        }
    }
}

impl fmt::Display for CiTestLane {
    fn fmt(&self, formatter: &mut fmt::Formatter<'_>) -> fmt::Result {
        formatter.write_str(self.as_str())
    }
}

impl FromStr for CiTestLane {
    type Err = String;

    fn from_str(value: &str) -> Result<Self, Self::Err> {
        match value {
            "owner-unit" => Ok(Self::OwnerUnit),
            "scenario" => Ok(Self::Scenario),
            "ui" => Ok(Self::Ui),
            "formal" => Ok(Self::Formal),
            "structural" => Ok(Self::Structural),
            _ => Err(format!(
                "unknown CI lane `{value}`; expected owner-unit, scenario, ui, formal, or structural"
            )),
        }
    }
}

pub(crate) fn classify(
    package: &str,
    target: &str,
    source: &str,
    is_integration: bool,
) -> Result<CiTestLane, String> {
    if !is_integration {
        return Ok(CiTestLane::OwnerUnit);
    }

    let normalized = source.replace('\\', "/").to_ascii_lowercase();
    let name = target.to_ascii_lowercase();
    let ui = normalized.contains("/tests/compile_fail/")
        || name.contains("compile_fail")
        || name.ends_with("_ui");
    let formal = package == "worth-store-formal-models";

    match (ui, formal) {
        (true, true) => Err(format!(
            "ambiguous CI lane for {package}::{target} at {source}: target is both UI and formal"
        )),
        (true, false) => Ok(CiTestLane::Ui),
        (false, true) => Ok(CiTestLane::Formal),
        (false, false) => Ok(CiTestLane::Scenario),
    }
}

#[cfg(test)]
mod tests {
    use super::classify;

    #[test]
    fn ambiguous_target_is_denied_with_identity() {
        let error = classify(
            "worth-store-formal-models",
            "conflict_ui",
            "/repo/tests/compile_fail/conflict_ui.rs",
            true,
        )
        .unwrap_err();
        assert!(error.contains("worth-store-formal-models::conflict_ui"));
        assert!(error.contains("compile_fail"));
    }
}
