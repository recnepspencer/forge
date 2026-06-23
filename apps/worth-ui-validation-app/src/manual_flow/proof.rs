use worth_ui::facade::WorthUiDropdownSelectionStateStatus;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ValidationManualFlowProof {
    status: String,
    visible_result: ValidationManualFlowVisibleResult,
    counter_posture: ValidationManualFlowCounterPosture,
    counter_details: String,
    replay_posture: ValidationManualFlowReplayPosture,
    projection_digest: String,
    changed_facts: Vec<String>,
    rebuilt_projections: Vec<String>,
    preserved_projections: Vec<String>,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub enum ValidationManualFlowVisibleResult {
    NotRunYet,
    SaveLabel(String),
    HeaderPanelFill(String),
    HeaderFontSizePx(u32),
    HeaderRowPaddingPx {
        vertical: u32,
        horizontal: u32,
    },
    HeaderContainerPaddingPx {
        top: i8,
        right: i8,
        bottom: i8,
        left: i8,
    },
    HeaderShadow(String),
    FileMenuSelectionMode(String),
    FileMenuReconciliation(WorthUiDropdownSelectionStateStatus),
    ComponentFactChanged(String),
    ProofComponent(String),
    ChangedFact(String),
    PreservedHeaderFontSizePx(u32),
    HeaderMenuMinWidthPx(u32),
    MixedStormPosture {
        activated: usize,
        equivalent: usize,
        denied: usize,
    },
    Unavailable(String),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationManualFlowCounterPosture {
    NotRunYet,
    NoVisibleRebindReceipts,
    HeaderPreservedPageHostPreserved,
    HeaderPreservedPageHostRebuilt,
    HeaderRebuiltPageHostPreserved,
    HeaderRebuiltPageHostRebuilt,
    HeaderPreservedDeniedPageHostPreservedDenied,
    HeaderPreservedEquivalentNoRebuild,
    VisibleRebindPostureMixed,
    MixedStormReplayStable,
    MixedStormUnavailable,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationManualFlowReplayPosture {
    NotRunYet,
    NotApplicable,
    ReplayAvailable,
    NotAvailable,
}

impl ValidationManualFlowProof {
    #[allow(clippy::too_many_arguments)]
    pub fn new(
        status: impl Into<String>,
        visible_result: ValidationManualFlowVisibleResult,
        counter_posture: ValidationManualFlowCounterPosture,
        counter_details: impl Into<String>,
        replay_posture: ValidationManualFlowReplayPosture,
        projection_digest: impl Into<String>,
        changed_facts: Vec<String>,
        rebuilt_projections: Vec<String>,
        preserved_projections: Vec<String>,
    ) -> Self {
        Self {
            status: status.into(),
            visible_result,
            counter_posture,
            counter_details: counter_details.into(),
            replay_posture,
            projection_digest: projection_digest.into(),
            changed_facts,
            rebuilt_projections,
            preserved_projections,
        }
    }

    pub fn not_run_yet() -> Self {
        Self::new(
            "Not run yet",
            ValidationManualFlowVisibleResult::NotRunYet,
            ValidationManualFlowCounterPosture::NotRunYet,
            "Not run yet",
            ValidationManualFlowReplayPosture::NotRunYet,
            "Not run yet",
            Vec::new(),
            Vec::new(),
            Vec::new(),
        )
    }

    pub fn status(&self) -> &str {
        &self.status
    }

    pub fn visible_result(&self) -> &ValidationManualFlowVisibleResult {
        &self.visible_result
    }

    pub fn counter_posture(&self) -> ValidationManualFlowCounterPosture {
        self.counter_posture
    }

    pub fn counter_details(&self) -> &str {
        &self.counter_details
    }

    pub fn replay_posture(&self) -> ValidationManualFlowReplayPosture {
        self.replay_posture
    }

    pub fn projection_digest(&self) -> &str {
        &self.projection_digest
    }

    pub fn changed_facts(&self) -> &[String] {
        &self.changed_facts
    }

    pub fn rebuilt_projections(&self) -> &[String] {
        &self.rebuilt_projections
    }

    pub fn preserved_projections(&self) -> &[String] {
        &self.preserved_projections
    }

    pub fn visible_result_label(&self) -> String {
        self.visible_result.label()
    }

    pub fn counter_posture_label(&self) -> &'static str {
        self.counter_posture.label()
    }

    pub fn replay_posture_label(&self) -> &'static str {
        self.replay_posture.label()
    }
}

impl ValidationManualFlowVisibleResult {
    pub fn label(&self) -> String {
        match self {
            Self::NotRunYet => "Not run yet".to_owned(),
            Self::SaveLabel(label) => format!("Save label = {label}"),
            Self::HeaderPanelFill(fill) => format!("Header panel fill = {fill}"),
            Self::HeaderFontSizePx(px) => format!("Header font size = {px}px"),
            Self::HeaderRowPaddingPx {
                vertical,
                horizontal,
            } => format!("Header row padding = {vertical}px/{horizontal}px"),
            Self::HeaderContainerPaddingPx {
                top,
                right,
                bottom,
                left,
            } => {
                format!("Header container padding = top {top} right {right} bottom {bottom} left {left}")
            }
            Self::HeaderShadow(summary) => format!("Header shadow = {summary}"),
            Self::FileMenuSelectionMode(mode) => {
                format!("File menu selection mode = {mode}")
            }
            Self::FileMenuReconciliation(status) => {
                format!("File menu reconciliation = {status:?}")
            }
            Self::ComponentFactChanged(component) => {
                format!("Component fact changed = {component}")
            }
            Self::ProofComponent(component) => format!("Proof component = {component}"),
            Self::ChangedFact(fact) => format!("Changed fact = {fact}"),
            Self::PreservedHeaderFontSizePx(px) => {
                format!("Header font size preserved at {px}px")
            }
            Self::HeaderMenuMinWidthPx(px) => format!("Header menu min width = {px}px"),
            Self::MixedStormPosture {
                activated,
                equivalent,
                denied,
            } => {
                format!("Storm posture = activated {activated} / equivalent {equivalent} / denied {denied}")
            }
            Self::Unavailable(reason) => reason.clone(),
        }
    }
}

impl ValidationManualFlowCounterPosture {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotRunYet => "Not run yet",
            Self::NoVisibleRebindReceipts => "No visible rebind receipts",
            Self::HeaderPreservedPageHostPreserved => "Header preserved; page-host preserved",
            Self::HeaderPreservedPageHostRebuilt => "Header preserved; page-host rebuilt",
            Self::HeaderRebuiltPageHostPreserved => "Header rebuilt; page-host preserved",
            Self::HeaderRebuiltPageHostRebuilt => "Header rebuilt; page-host rebuilt",
            Self::HeaderPreservedDeniedPageHostPreservedDenied => {
                "Header preserved denied; page-host preserved denied"
            }
            Self::HeaderPreservedEquivalentNoRebuild => {
                "Header preserved equivalent; no rebuild work"
            }
            Self::VisibleRebindPostureMixed => "Visible rebind posture mixed",
            Self::MixedStormReplayStable => "Mixed storm counters replay-stable",
            Self::MixedStormUnavailable => "Mixed storm proof unavailable",
        }
    }
}

impl ValidationManualFlowReplayPosture {
    pub fn label(self) -> &'static str {
        match self {
            Self::NotRunYet => "Not run yet",
            Self::NotApplicable => "not_applicable",
            Self::ReplayAvailable => "replay_available",
            Self::NotAvailable => "not_available",
        }
    }
}
