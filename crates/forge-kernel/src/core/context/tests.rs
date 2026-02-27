//! Tests for ModelingContext — decision logging, policy resolution, sub-ops.

#[cfg(test)]
mod context_tests {
    use crate::check_tolerance;
    use crate::core::context::schema::ModelingContext;
    use forge_core::envelope::OperationResult;
    use forge_core::tracing::{
        CandidateValueSummary, PolicyResolutionOutcome, PolicyResolutionSource,
    };
    use forge_core::{
        DecisionContext, DecisionId, DecisionKind, DecisionTier, PolicyKind, PolicyQuery,
        TracedDecision,
    };

    fn sample_policy_query(kind: PolicyKind, overridable: bool) -> PolicyQuery {
        PolicyQuery {
            kind,
            location: [1.0, 2.0, 3.0],
            margin: 2.5e-6,
            overridable,
        }
    }

    fn sample_candidate_summary() -> CandidateValueSummary {
        CandidateValueSummary::EnumTag {
            type_name: "WeakSimpleCertificate".into(),
            variant: "WeaklySimple".into(),
        }
    }

    #[test]
    fn default_context_has_no_decisions() {
        let ctx = ModelingContext::new();
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn decisions_are_recorded() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::NearBoundary { threshold: 1e-6 },
            DecisionTier::NearBoundary,
            [1.0, 2.0, 3.0],
            1e-8,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);
        let decisions: Vec<_> = ctx.get_decision_log().decisions().collect();
        assert_eq!(decisions[0].get_id(), DecisionId(1));
    }

    #[test]
    fn check_tolerance_macro_logs_when_within() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-8;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(
            ctx,
            threshold,
            distance,
            location,
            DecisionKind::NearBoundary { threshold }
        );

        assert!(within);
        assert_eq!(ctx.get_decision_count(), 1);
    }

    #[test]
    fn check_tolerance_macro_does_not_log_when_outside() {
        let mut ctx = ModelingContext::new();
        let distance = 1e-3;
        let threshold = 1e-6;
        let location = [0.0, 0.0, 0.0];

        let within = check_tolerance!(
            ctx,
            threshold,
            distance,
            location,
            DecisionKind::NearBoundary { threshold }
        );

        assert!(!within);
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn take_decision_log_drains() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1e-6,
        );
        assert_eq!(ctx.get_decision_count(), 1);

        let log = ctx.take_decision_log();
        assert_eq!(log.len(), 1);
        assert_eq!(ctx.get_decision_count(), 0);
    }

    #[test]
    fn reset_for_new_operation_restarts_decision_ids_at_one() {
        let mut ctx = ModelingContext::new();

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let _ = ctx.take_decision_log();

        ctx.reset_for_new_operation();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );

        let ids: Vec<_> = ctx
            .get_decision_log()
            .decisions()
            .map(|d| d.get_id().0)
            .collect();
        assert_eq!(
            ids,
            vec![1],
            "full operation reset must restore deterministic ID sequence"
        );
    }

    #[test]
    fn reset_for_new_operation_clears_log_drained_and_sub_metadata_sink() {
        let mut ctx = ModelingContext::new();
        ctx.enable_auto_persist();

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let _ = ctx.take_decision_log();
        assert!(
            ctx.log_drained,
            "take_decision_log sets success-path drain flag"
        );

        let mut sub = OperationResult::new(());
        sub.add_warning(forge_core::KernelWarning::AutoDecision {
            decision_id: DecisionId(99),
        });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_deleted = 4;
        sub.set_metrics(metrics);
        ctx.absorb_sub_result(&mut sub);
        assert_eq!(ctx.get_sub_warnings().len(), 1);
        assert_eq!(ctx.get_sub_metrics().entities_deleted, 4);

        ctx.reset_for_new_operation();

        assert!(
            !ctx.log_drained,
            "new operation must not inherit prior success-path drain state"
        );
        assert_eq!(ctx.get_decision_count(), 0);
        assert!(ctx.get_sub_warnings().is_empty());
        assert_eq!(ctx.get_sub_metrics().entities_deleted, 0);
        assert_eq!(ctx.get_sub_accumulated_error_budget(), 0.0);
        assert_eq!(
            ctx.get_tolerance_config().get_error_budget_mm(),
            f64::INFINITY
        );
    }

    #[test]
    fn clear_decision_log_only_preserves_counter_and_sub_metadata() {
        let mut ctx = ModelingContext::new();
        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let mut sub = OperationResult::new(());
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 7;
        sub.set_metrics(metrics);
        ctx.absorb_sub_result(&mut sub);

        ctx.clear_decision_log_only();
        assert_eq!(ctx.get_decision_count(), 0);
        assert_eq!(
            ctx.get_sub_metrics().entities_created,
            7,
            "log-only clear must not wipe metadata sink"
        );

        ctx.log_decision(
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            [0.0, 0.0, 0.0],
            0.0,
            1.0,
        );
        let ids: Vec<_> = ctx
            .get_decision_log()
            .decisions()
            .map(|d| d.get_id().0)
            .collect();
        assert_eq!(
            ids,
            vec![2],
            "log-only clear preserves monotonically increasing IDs"
        );
    }

    #[test]
    fn absorb_sub_result_accumulates_full_metadata() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.get_decision_log_mut().record(TracedDecision::new(
            DecisionId(42),
            DecisionKind::Exact,
            DecisionTier::Deterministic,
            1.0,
            DecisionContext::Degeneracy {
                description: "sub".into(),
            },
        ));
        sub.add_warning(forge_core::KernelWarning::AutoDecision {
            decision_id: DecisionId(42),
        });

        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_modified = 5;
        metrics.policy_decisions_made = 1;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.faces_deleted = 2;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(2e-6);

        ctx.absorb_sub_result(&mut sub);

        assert_eq!(ctx.get_decision_count(), 1);
        assert_eq!(ctx.get_sub_warnings().len(), 1);
        assert_eq!(ctx.get_sub_metrics().entities_modified, 5);
        assert_eq!(ctx.get_sub_metrics().policy_decisions_made, 1);
        assert_eq!(ctx.get_sub_lineage_delta().faces_deleted, 2);
        assert!(ctx.get_sub_accumulated_error_budget() > 0.0);
        assert!(sub.get_decision_log().is_empty());
        assert!(sub.get_warnings().is_empty());
        assert_eq!(sub.get_metrics().entities_modified, 0);
        assert_eq!(sub.get_metrics().policy_decisions_made, 0);
        assert_eq!(sub.get_lineage_delta().faces_deleted, 0);
        assert_eq!(sub.get_accumulated_budget(), 0.0);
    }

    #[test]
    fn absorb_sub_result_is_idempotent_for_child_envelope_metadata() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.add_warning(forge_core::KernelWarning::AutoDecision {
            decision_id: DecisionId(11),
        });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 2;
        metrics.policy_decisions_made = 1;
        sub.set_metrics(metrics);
        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.edges_deleted = 3;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(9.0e-7);

        ctx.absorb_sub_result(&mut sub);
        ctx.absorb_sub_result(&mut sub);

        assert_eq!(
            ctx.get_decision_count(),
            0,
            "no decisions were added in this fixture"
        );
        assert_eq!(
            ctx.get_sub_warnings().len(),
            1,
            "warnings must not double-count"
        );
        assert_eq!(
            ctx.get_sub_metrics().entities_created,
            2,
            "metrics must drain from child"
        );
        assert_eq!(ctx.get_sub_metrics().policy_decisions_made, 1);
        assert_eq!(
            ctx.get_sub_lineage_delta().edges_deleted,
            3,
            "lineage must drain from child"
        );
        assert!((ctx.get_sub_accumulated_error_budget() - 9.0e-7).abs() < f64::EPSILON);
    }

    #[test]
    fn absorb_sub_result_accumulates_then_take_sub_metadata_drains() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        sub.add_warning(forge_core::KernelWarning::AutoDecision {
            decision_id: DecisionId(7),
        });
        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_created = 3;
        metrics.policy_decisions_made = 2;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.vertices_created = 4;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(1.25e-6);

        ctx.absorb_sub_result(&mut sub);
        let drained = ctx.take_sub_metadata();

        assert_eq!(drained.warnings.len(), 1);
        assert_eq!(drained.metrics.entities_created, 3);
        assert_eq!(drained.metrics.policy_decisions_made, 2);
        assert_eq!(drained.lineage_delta.vertices_created, 4);
        assert!((drained.accumulated_error_budget - 1.25e-6).abs() < f64::EPSILON);

        assert!(ctx.get_sub_warnings().is_empty());
        assert_eq!(ctx.get_sub_metrics().entities_created, 0);
        assert_eq!(ctx.get_sub_lineage_delta().vertices_created, 0);
        assert_eq!(ctx.get_sub_accumulated_error_budget(), 0.0);
    }

    #[test]
    fn take_sub_metadata_is_idempotent_after_reset() {
        let mut ctx = ModelingContext::new();

        let first = ctx.take_sub_metadata();
        assert!(first.warnings.is_empty());
        assert_eq!(first.metrics.entities_created, 0);
        assert_eq!(first.accumulated_error_budget, 0.0);

        let second = ctx.take_sub_metadata();
        assert!(second.warnings.is_empty());
        assert_eq!(second.metrics.entities_created, 0);
        assert_eq!(second.accumulated_error_budget, 0.0);
    }

    #[test]
    fn policy_missing_fails_closed_and_emits_typed_adjunct() {
        let mut ctx = ModelingContext::new();
        let query = sample_policy_query(PolicyKind::NearTangency, true);

        let err = ctx
            .resolve_policy_query(
                DecisionId(1201),
                &query,
                Some(5.0e-5),
                sample_candidate_summary(),
            )
            .expect_err("missing policy must fail closed");
        assert!(matches!(
            err,
            forge_core::KernelError::AmbiguousResult { .. }
        ));

        let decisions: Vec<_> = ctx.get_decision_log().decisions().collect();
        assert_eq!(
            decisions.len(),
            1,
            "missing policy path must still emit a traced decision"
        );
        assert!(matches!(
            decisions[0].get_kind(),
            DecisionKind::Ambiguous { .. }
        ));

        let adjuncts = ctx.get_trace_adjuncts();
        assert_eq!(
            adjuncts.records().len(),
            1,
            "missing policy path must emit typed policy adjunct"
        );
        let payload = adjuncts.records()[0]
            .as_policy_payload()
            .expect("policy adjunct kind")
            .expect("decode policy payload");
        assert_eq!(payload.outcome, PolicyResolutionOutcome::EscalatedError);
        assert_eq!(payload.source, PolicyResolutionSource::ForcedSafeFallback);
        assert!(payload.source_scope.is_none());
        assert_eq!(payload.decision_id, DecisionId(1201));
    }

    #[test]
    fn sub_metadata_can_be_folded_into_operation_result_without_double_counting() {
        let mut ctx = ModelingContext::new();
        let mut sub = OperationResult::new(());

        let mut metrics = forge_core::envelope::OperationMetrics::default();
        metrics.entities_modified = 8;
        sub.set_metrics(metrics);

        let mut lineage = forge_core::envelope::LineageDelta::default();
        lineage.faces_created = 2;
        sub.set_lineage_delta(lineage);
        sub.consume_budget(3.0e-6);

        ctx.absorb_sub_result(&mut sub);
        let drained = ctx.take_sub_metadata();

        let mut parent = OperationResult::new("ok");
        for warning in drained.warnings {
            parent.add_warning(warning);
        }
        parent.set_metrics(drained.metrics.clone());
        parent.set_lineage_delta(drained.lineage_delta.clone());
        parent.consume_budget(drained.accumulated_error_budget);

        assert_eq!(parent.get_metrics().entities_modified, 8);
        assert_eq!(parent.get_lineage_delta().faces_created, 2);
        assert!((parent.get_accumulated_budget() - 3.0e-6).abs() < f64::EPSILON);

        let drained_again = ctx.take_sub_metadata();
        assert_eq!(drained_again.metrics.entities_modified, 0);
        assert_eq!(drained_again.lineage_delta.faces_created, 0);
        assert_eq!(drained_again.accumulated_error_budget, 0.0);
    }
}
