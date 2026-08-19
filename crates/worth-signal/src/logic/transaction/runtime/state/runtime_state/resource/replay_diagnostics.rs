use crate::data::resource::{
    ResourceBoundaryPerformanceEnvelope, ResourceDiagnosticsExpansionBudget,
    ResourceDiagnosticsExpansionDenial, ResourceDiagnosticsSummary, ResourceNodeDeclaration,
    ResourceReplayAvailabilityClass, ResourceReplayAvailabilityDenialClass,
    ResourceReplayAvailabilityReport, ResourceReplayReconstructionReport,
};

use super::super::SignalRuntime;

impl<D, I, E, Ctx, T> SignalRuntime<D, I, E, Ctx, T>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
    I: Copy + Ord,
    T: Copy + Ord,
{
    pub fn resource_replay_availability(
        &mut self,
        declaration: &ResourceNodeDeclaration,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        self.resource_replay_availability_with_optional_cold_reconstruction_budget(
            declaration,
            None,
        )
    }

    pub fn resource_replay_availability_with_cold_reconstruction_budget(
        &mut self,
        declaration: &ResourceNodeDeclaration,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        self.resource_replay_availability_with_optional_cold_reconstruction_budget(
            declaration,
            Some(budget),
        )
    }

    fn resource_replay_availability_with_optional_cold_reconstruction_budget(
        &mut self,
        declaration: &ResourceNodeDeclaration,
        budget: Option<ResourceDiagnosticsExpansionBudget>,
    ) -> Result<ResourceReplayAvailabilityReport, crate::data::error::SignalError> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        let summary_read = self.resource_runtime_summary_read_report();
        let compatibility = self.admit_resource_policy_restore_compatibility(declaration)?;
        let unavailable_count = summary_read
            .summary()
            .retained_history_unavailable_count()
            .saturating_add(
                summary_read
                    .summary()
                    .retained_denied_completion_unavailable_count(),
            )
            .saturating_add(
                summary_read
                    .summary()
                    .retained_retry_lineage_unavailable_count(),
            ) as u32;

        if capture_telemetry {
            self.telemetry
                .resource
                .resource_replay_availability_decision_count += 1;
        }

        let (
            class,
            denial_class,
            restore_compatibility,
            restore_compatibility_denial,
            diagnostics_summary,
            diagnostics_denial,
        ) = match compatibility {
            Ok(proof) if unavailable_count == 0 => {
                if capture_telemetry {
                    self.telemetry
                        .resource
                        .resource_replay_availability_retained_count += 1;
                }
                (
                    ResourceReplayAvailabilityClass::Retained,
                    None,
                    Some(proof),
                    None,
                    None,
                    None,
                )
            }
            Ok(proof) if proof.replay_decision_class().denies_unavailable_history() => {
                if capture_telemetry {
                    self.telemetry
                        .resource
                        .resource_replay_availability_denied_count += 1;
                    self.telemetry
                        .resource
                        .resource_replay_budget_history_unavailable_count += 1;
                }
                (
                    ResourceReplayAvailabilityClass::Denied,
                    Some(ResourceReplayAvailabilityDenialClass::BudgetHistoryUnavailable),
                    Some(proof),
                    None,
                    None,
                    None,
                )
            }
            Ok(proof) => match budget {
                None => {
                    if capture_telemetry {
                        self.telemetry
                            .resource
                            .resource_replay_availability_omitted_count += 1;
                    }
                    (
                        ResourceReplayAvailabilityClass::Omitted,
                        None,
                        Some(proof),
                        None,
                        None,
                        None,
                    )
                }
                Some(budget) => match self.try_resource_diagnostics_summary(budget) {
                    Ok(summary) => {
                        if capture_telemetry {
                            self.telemetry
                                .resource
                                .resource_replay_availability_reconstructed_count += 1;
                        }
                        (
                            ResourceReplayAvailabilityClass::Reconstructed,
                            None,
                            Some(proof),
                            None,
                            Some(summary),
                            None,
                        )
                    }
                    Err(denial) => {
                        if capture_telemetry {
                            self.telemetry
                                .resource
                                .resource_replay_availability_unavailable_count += 1;
                        }
                        (
                            ResourceReplayAvailabilityClass::Unavailable,
                            None,
                            Some(proof),
                            None,
                            None,
                            Some(denial),
                        )
                    }
                },
            },
            Err(denial) => {
                if capture_telemetry {
                    self.telemetry
                        .resource
                        .resource_replay_availability_denied_count += 1;
                }
                (
                    ResourceReplayAvailabilityClass::Denied,
                    Some(ResourceReplayAvailabilityDenialClass::RestoreCompatibilityDenied),
                    None,
                    Some(denial),
                    None,
                    None,
                )
            }
        };

        let compatibility_width = restore_compatibility
            .as_ref()
            .map(|proof| proof.compatibility().compared_width())
            .or_else(|| {
                restore_compatibility_denial
                    .as_ref()
                    .map(|denial| denial.compatibility().compared_width())
            })
            .unwrap_or(0);
        let diagnostics_width = diagnostics_summary
            .as_ref()
            .map(|summary| summary.performance().input_width())
            .or_else(|| {
                diagnostics_denial
                    .as_ref()
                    .map(|denial| denial.performance().input_width())
            })
            .unwrap_or(0);
        let performance = ResourceBoundaryPerformanceEnvelope::replay_availability(
            summary_read
                .performance()
                .input_width()
                .saturating_add(compatibility_width)
                .saturating_add(diagnostics_width),
            u32::from(class != ResourceReplayAvailabilityClass::Denied),
            u32::from(class == ResourceReplayAvailabilityClass::Denied),
            u32::from(diagnostics_summary.is_some() || diagnostics_denial.is_some()),
        );
        if capture_telemetry {
            self.telemetry
                .resource
                .record_boundary_performance_envelope(performance);
        }

        Ok(ResourceReplayAvailabilityReport::new(
            class,
            denial_class,
            summary_read,
            restore_compatibility,
            restore_compatibility_denial,
            diagnostics_summary,
            diagnostics_denial,
            performance,
        ))
    }

    pub fn reconstruct_resource_replay_summary(&mut self) -> ResourceReplayReconstructionReport {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        self.resource.reconstruct_replay_summary_optional(
            capture_telemetry.then_some(&mut self.telemetry.resource),
        )
    }

    pub fn resource_diagnostics_summary_with_cold_reconstruction_budget(
        &mut self,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceDiagnosticsSummary, ResourceDiagnosticsExpansionDenial> {
        self.try_resource_diagnostics_summary(budget)
    }

    pub fn resource_diagnostics_summary_with_unbounded_cold_reconstruction(
        &mut self,
    ) -> ResourceDiagnosticsSummary {
        self.resource_diagnostics_summary_with_cold_reconstruction_budget(
            ResourceDiagnosticsExpansionBudget::allow_cold_reconstruction(u32::MAX),
        )
        .expect("unbounded resource diagnostics budget should admit replay reconstruction")
    }

    pub fn try_resource_diagnostics_summary(
        &mut self,
        budget: ResourceDiagnosticsExpansionBudget,
    ) -> Result<ResourceDiagnosticsSummary, ResourceDiagnosticsExpansionDenial> {
        let capture_telemetry = self.graph.captures_observation_surface(
            crate::logic::transaction::SignalObservationSurface::OptionalTelemetry,
        );
        let runtime_summary = self.resource_runtime_summary();
        let latest_branch_restore_report = self.latest_resource_branch_restore_report();
        let estimated_replay_width = self.resource.replay_reconstruction_width();
        let estimated_forensic_width = estimated_replay_width;
        let branch_restore_width = u32::from(latest_branch_restore_report.is_some());
        let effective_policy = self.resource.effective_diagnostics_policy();
        if capture_telemetry {
            self.telemetry
                .resource
                .resource_diagnostics_policy_decision_count += 1;
        }
        if let Some(class) = match effective_policy.class() {
            crate::data::resource::ResourceDiagnosticsDecisionClass::RetainedOnly => Some(
                crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyRetainedOnly,
            ),
            crate::data::resource::ResourceDiagnosticsDecisionClass::DenyColdExpansion => Some(
                crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyColdReconstructionDisabled,
            ),
            crate::data::resource::ResourceDiagnosticsDecisionClass::BudgetedExpansion => {
                let policy_limit = effective_policy
                    .max_replay_reconstruction_width()
                    .unwrap_or(u32::MAX);
                if estimated_replay_width > policy_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded,
                    )
                } else {
                    budget.denial_class(estimated_replay_width, estimated_forensic_width)
                }
            }
            crate::data::resource::ResourceDiagnosticsDecisionClass::ForensicExpansionBudget => {
                let policy_replay_limit = effective_policy
                    .max_replay_reconstruction_width()
                    .unwrap_or(u32::MAX);
                let policy_forensic_limit = effective_policy
                    .max_forensic_reconstruction_width()
                    .unwrap_or(u32::MAX);
                if estimated_replay_width > policy_replay_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyReplayReconstructionBudgetExceeded,
                    )
                } else if estimated_forensic_width > policy_forensic_limit {
                    Some(
                        crate::data::resource::ResourceDiagnosticsExpansionDenialClass::PolicyForensicReconstructionBudgetExceeded,
                    )
                } else {
                    budget.denial_class(estimated_replay_width, estimated_forensic_width)
                }
            }
        } {
            let performance = ResourceBoundaryPerformanceEnvelope::diagnostics_expansion_denied(
                1_u32.saturating_add(effective_policy.descriptor_width()),
                estimated_replay_width,
                branch_restore_width,
            );
            self.with_resource_telemetry(|telemetry| telemetry.resource_diagnostics_expansion_count += 1);
            if capture_telemetry {
                self.telemetry
                    .resource
                    .resource_diagnostics_expansion_input_width = self
                    .telemetry
                    .resource
                    .resource_diagnostics_expansion_input_width
                    .max(performance.input_width() as u64);
                self.telemetry
                    .resource
                    .record_boundary_performance_envelope(performance);
            }
            return Err(ResourceDiagnosticsExpansionDenial::new(
                class,
                effective_policy.class(),
                budget,
                estimated_replay_width,
                estimated_forensic_width,
                performance,
                effective_policy.decision_digest().clone(),
            ));
        }
        let replay_reconstruction = self.reconstruct_resource_replay_summary();
        let replay_reconstruction_width = replay_reconstruction
            .descriptor_width()
            .saturating_add(replay_reconstruction.lifecycle_summary_width())
            .saturating_add(replay_reconstruction.denied_completion_width())
            .saturating_add(replay_reconstruction.in_flight_width());
        let performance = ResourceBoundaryPerformanceEnvelope::diagnostics_expansion(
            1_u32.saturating_add(effective_policy.descriptor_width()),
            replay_reconstruction_width,
            branch_restore_width,
        );
        self.with_resource_telemetry(|telemetry| {
            telemetry.resource_diagnostics_expansion_count += 1
        });
        if capture_telemetry {
            self.telemetry
                .resource
                .resource_diagnostics_expansion_input_width = self
                .telemetry
                .resource
                .resource_diagnostics_expansion_input_width
                .max(performance.input_width() as u64);
            self.telemetry
                .resource
                .resource_diagnostics_cold_reconstruction_count += 1;
            self.telemetry
                .resource
                .record_boundary_performance_envelope(performance);
        }
        Ok(ResourceDiagnosticsSummary::new(
            runtime_summary,
            latest_branch_restore_report,
            replay_reconstruction,
            budget,
            effective_policy.class(),
            performance,
            effective_policy.decision_digest().clone(),
        ))
    }
}
