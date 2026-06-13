use std::collections::BTreeSet;

use super::denial::{OpenClassTriadParityDenial, OpenClassTriadParityDenialKind};
use super::lane_set::OpenClassParityLaneSet;
use super::open_class::OpenTopologyClass;
use super::receipt::OpenClassTriadParityReceipt;

pub struct OpenClassTriadParityWorkload {
    declaration: String,
    lane_sets: Vec<OpenClassParityLaneSet>,
}

impl OpenClassTriadParityWorkload {
    pub fn new() -> Self {
        Self {
            declaration: "open-class triad parity".to_string(),
            lane_sets: Vec::new(),
        }
    }

    pub fn declared(mut self, declaration: impl Into<String>) -> Self {
        self.declaration = declaration.into();
        self
    }

    pub fn with_class_lane_set(mut self, lane_set: OpenClassParityLaneSet) -> Self {
        self.lane_sets.push(lane_set);
        self
    }

    pub fn compare_required_lanes(self) -> OpenClassTriadParityComparison {
        OpenClassTriadParityComparison {
            declaration: self.declaration,
            lane_sets: self.lane_sets,
        }
    }
}

impl Default for OpenClassTriadParityWorkload {
    fn default() -> Self {
        Self::new()
    }
}

pub struct OpenClassTriadParityComparison {
    declaration: String,
    lane_sets: Vec<OpenClassParityLaneSet>,
}

impl OpenClassTriadParityComparison {
    pub fn certify(self) -> Result<OpenClassTriadParityReceipt, OpenClassTriadParityDenial> {
        if self.declaration.trim().is_empty() {
            return Err(OpenClassTriadParityDenial::new(
                OpenClassTriadParityDenialKind::MissingDeclaration,
                None,
                "Open-class triad parity requires a human-readable declaration.",
            ));
        }
        self.require_each_open_class_once()?;
        Ok(OpenClassTriadParityReceipt::new(
            self.declaration,
            self.lane_sets,
        ))
    }

    fn require_each_open_class_once(&self) -> Result<(), OpenClassTriadParityDenial> {
        let mut seen = BTreeSet::new();
        for lane_set in &self.lane_sets {
            if !seen.insert(lane_set.topology_class()) {
                return Err(OpenClassTriadParityDenial::new(
                    OpenClassTriadParityDenialKind::DuplicateOpenClass,
                    Some(lane_set.topology_class()),
                    format!(
                        "Open-class triad parity has duplicate {} evidence.",
                        lane_set.topology_class().human_name()
                    ),
                ));
            }
        }
        for required in OpenTopologyClass::REQUIRED {
            if !seen.contains(&required) {
                return Err(OpenClassTriadParityDenial::new(
                    OpenClassTriadParityDenialKind::MissingOpenClass,
                    Some(required),
                    format!(
                        "Open-class triad parity is missing {} evidence.",
                        required.human_name()
                    ),
                ));
            }
        }
        Ok(())
    }
}
