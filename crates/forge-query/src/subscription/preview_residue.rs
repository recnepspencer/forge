use crate::identity::hash_parts;

use super::delivery_dimensions::PreviewResidueWidth;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum PreviewSubscriptionResidueClass {
    AuthoritativeRouting,
    AuthoritativeCheckpoint,
    AuthoritativeReplay,
    AuthoritativeDiagnostics,
    AuthoritativeWriteback,
    TemporaryPreviewExecution,
    TemporaryPreviewDiagnostics,
}

impl PreviewSubscriptionResidueClass {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::AuthoritativeRouting => "authoritative_routing",
            Self::AuthoritativeCheckpoint => "authoritative_checkpoint",
            Self::AuthoritativeReplay => "authoritative_replay",
            Self::AuthoritativeDiagnostics => "authoritative_diagnostics",
            Self::AuthoritativeWriteback => "authoritative_writeback",
            Self::TemporaryPreviewExecution => "temporary_preview_execution",
            Self::TemporaryPreviewDiagnostics => "temporary_preview_diagnostics",
        }
    }

    pub fn is_authoritative(&self) -> bool {
        matches!(
            self,
            Self::AuthoritativeRouting
                | Self::AuthoritativeCheckpoint
                | Self::AuthoritativeReplay
                | Self::AuthoritativeDiagnostics
                | Self::AuthoritativeWriteback
        )
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct PreviewSubscriptionResidueReport {
    authoritative_routing_width: PreviewResidueWidth,
    authoritative_checkpoint_width: PreviewResidueWidth,
    authoritative_replay_width: PreviewResidueWidth,
    authoritative_diagnostics_width: PreviewResidueWidth,
    authoritative_writeback_width: PreviewResidueWidth,
    temporary_execution_width: PreviewResidueWidth,
    temporary_diagnostics_width: PreviewResidueWidth,
    report_digest: String,
}

impl PreviewSubscriptionResidueReport {
    #[allow(clippy::too_many_arguments)]
    pub(super) fn new(
        authoritative_routing_width: PreviewResidueWidth,
        authoritative_checkpoint_width: PreviewResidueWidth,
        authoritative_replay_width: PreviewResidueWidth,
        authoritative_diagnostics_width: PreviewResidueWidth,
        authoritative_writeback_width: PreviewResidueWidth,
        temporary_execution_width: PreviewResidueWidth,
        temporary_diagnostics_width: PreviewResidueWidth,
    ) -> Self {
        let report_digest = hash_parts(&[
            "preview_subscription_residue_report_v1".to_string(),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeRouting.as_str(),
                authoritative_routing_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeCheckpoint.as_str(),
                authoritative_checkpoint_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeReplay.as_str(),
                authoritative_replay_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeDiagnostics.as_str(),
                authoritative_diagnostics_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::AuthoritativeWriteback.as_str(),
                authoritative_writeback_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::TemporaryPreviewExecution.as_str(),
                temporary_execution_width.get()
            ),
            format!(
                "{}:{}",
                PreviewSubscriptionResidueClass::TemporaryPreviewDiagnostics.as_str(),
                temporary_diagnostics_width.get()
            ),
        ]);
        Self {
            authoritative_routing_width,
            authoritative_checkpoint_width,
            authoritative_replay_width,
            authoritative_diagnostics_width,
            authoritative_writeback_width,
            temporary_execution_width,
            temporary_diagnostics_width,
            report_digest,
        }
    }

    pub fn authoritative_residue_width(&self) -> u64 {
        self.authoritative_routing_width.get()
            + self.authoritative_checkpoint_width.get()
            + self.authoritative_replay_width.get()
            + self.authoritative_diagnostics_width.get()
            + self.authoritative_writeback_width.get()
    }

    pub fn temporary_residue_width(&self) -> u64 {
        self.temporary_execution_width.get() + self.temporary_diagnostics_width.get()
    }

    pub fn preview_residue_width(&self) -> u64 {
        self.authoritative_residue_width() + self.temporary_residue_width()
    }

    pub fn class_width(&self, residue_class: PreviewSubscriptionResidueClass) -> u64 {
        match residue_class {
            PreviewSubscriptionResidueClass::AuthoritativeRouting => {
                self.authoritative_routing_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeCheckpoint => {
                self.authoritative_checkpoint_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeReplay => {
                self.authoritative_replay_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeDiagnostics => {
                self.authoritative_diagnostics_width.get()
            }
            PreviewSubscriptionResidueClass::AuthoritativeWriteback => {
                self.authoritative_writeback_width.get()
            }
            PreviewSubscriptionResidueClass::TemporaryPreviewExecution => {
                self.temporary_execution_width.get()
            }
            PreviewSubscriptionResidueClass::TemporaryPreviewDiagnostics => {
                self.temporary_diagnostics_width.get()
            }
        }
    }

    pub fn report_digest(&self) -> &str {
        &self.report_digest
    }
}

#[allow(clippy::too_many_arguments)]
pub fn measure_preview_subscription_residue(
    authoritative_routing_width: PreviewResidueWidth,
    authoritative_checkpoint_width: PreviewResidueWidth,
    authoritative_replay_width: PreviewResidueWidth,
    authoritative_diagnostics_width: PreviewResidueWidth,
    authoritative_writeback_width: PreviewResidueWidth,
    temporary_execution_width: PreviewResidueWidth,
    temporary_diagnostics_width: PreviewResidueWidth,
) -> PreviewSubscriptionResidueReport {
    PreviewSubscriptionResidueReport::new(
        authoritative_routing_width,
        authoritative_checkpoint_width,
        authoritative_replay_width,
        authoritative_diagnostics_width,
        authoritative_writeback_width,
        temporary_execution_width,
        temporary_diagnostics_width,
    )
}
