use std::collections::BTreeSet;

use serde::{Deserialize, Serialize};

use crate::capture::{DiagnosticsLevel, ExecutionMode};
use crate::comparison::ComparisonMode;
use crate::timeline::{ClockDomain, ExecutionPhase};

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum CaptureDepth {
    Minimal,
    Standard,
    Rich,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum DeterminismMode {
    RuntimeDefault,
    Strict,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize)]
pub enum AdapterSupport {
    Unsupported,
    Supported,
}

#[derive(Debug, Clone, PartialEq, Eq, Serialize, Deserialize)]
pub struct HarnessCapabilities {
    pub execution_modes: BTreeSet<ExecutionMode>,
    pub diagnostics_levels: BTreeSet<DiagnosticsLevel>,
    pub capture_depths: BTreeSet<CaptureDepth>,
    pub comparison_modes: BTreeSet<ComparisonMode>,
    pub clock_domains: BTreeSet<ClockDomain>,
    pub execution_phases: BTreeSet<ExecutionPhase>,
    pub replay_support: AdapterSupport,
    pub lineage_support: AdapterSupport,
    pub provenance_support: AdapterSupport,
    pub event_stream_support: AdapterSupport,
    pub performance_counter_support: AdapterSupport,
    pub workload_budget_support: AdapterSupport,
    pub attachment_support: AdapterSupport,
    pub rich_record_kinds: BTreeSet<String>,
}

impl Default for HarnessCapabilities {
    fn default() -> Self {
        let mut execution_modes = BTreeSet::new();
        execution_modes.insert(ExecutionMode::RuntimeDefault);
        execution_modes.insert(ExecutionMode::Serial);

        let mut diagnostics_levels = BTreeSet::new();
        diagnostics_levels.insert(DiagnosticsLevel::Off);

        let mut capture_depths = BTreeSet::new();
        capture_depths.insert(CaptureDepth::Standard);

        let mut comparison_modes = BTreeSet::new();
        comparison_modes.insert(ComparisonMode::Exact);
        let mut clock_domains = BTreeSet::new();
        clock_domains.insert(ClockDomain::Logical);
        let execution_phases = BTreeSet::new();

        Self {
            execution_modes,
            diagnostics_levels,
            capture_depths,
            comparison_modes,
            clock_domains,
            execution_phases,
            replay_support: AdapterSupport::Unsupported,
            lineage_support: AdapterSupport::Unsupported,
            provenance_support: AdapterSupport::Unsupported,
            event_stream_support: AdapterSupport::Unsupported,
            performance_counter_support: AdapterSupport::Unsupported,
            workload_budget_support: AdapterSupport::Unsupported,
            attachment_support: AdapterSupport::Unsupported,
            rich_record_kinds: BTreeSet::new(),
        }
    }
}

impl HarnessCapabilities {
    pub fn supports_execution_mode(&self, mode: ExecutionMode) -> bool {
        self.execution_modes.contains(&mode)
    }

    pub fn supports_diagnostics_level(&self, level: DiagnosticsLevel) -> bool {
        self.diagnostics_levels.contains(&level)
    }

    pub fn supports_capture_depth(&self, depth: CaptureDepth) -> bool {
        self.capture_depths.contains(&depth)
    }

    pub fn supports_comparison_mode(&self, mode: ComparisonMode) -> bool {
        self.comparison_modes.contains(&mode)
    }

    pub fn supports_clock_domain(&self, domain: ClockDomain) -> bool {
        self.clock_domains.contains(&domain)
    }

    pub fn supports_execution_phase(&self, phase: ExecutionPhase) -> bool {
        self.execution_phases.contains(&phase)
    }
}
