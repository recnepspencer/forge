use crate::data::aspect::{Aspect, AspectMask};
use crate::data::comparator::ComparatorPolicyResolver;
use crate::data::error::SignalError;
use crate::data::output_equivalence::OutputEquivalencePolicy;
use crate::data::proof::invalidation::output_commit::{
    CommittedProducedAspectDelta, ProducedAspectDelta,
};
use crate::data::proof::invalidation::progression::{
    CommittedDirectInvalidation, PreparedDirectInvalidation,
};
use crate::logic::evaluation::{
    AppliedEffectReport, EvaluationEffect, EvaluationVerdict, SuppressionReason,
};

use super::{
    ApplyCommitPacket, DirectInvalidationPreparationReceipt, OutputCommitPacket,
    OutputCommitPublicationReceipt, PreparedParallelApplyCommitPacket, SignalGraph,
};

#[derive(Clone, Copy, Debug, PartialEq, Eq)]
pub(super) enum OutputCommitPreparationSeam {
    SemanticDecision,
    ProducedDelta,
    DirectCauseAdmission,
    PacketPrevalidation,
}

enum PerformedOutputPublication {
    Changed(CommittedDirectInvalidation),
    Stable,
}

impl PerformedOutputPublication {
    fn committed(&self) -> Option<&CommittedProducedAspectDelta> {
        match self {
            Self::Changed(commit) => Some(commit.publication()),
            Self::Stable => None,
        }
    }

    fn report(
        &self,
        verdict: EvaluationVerdict,
        comparison: crate::logic::evaluation::EffectComparison,
        suppressed_downstream: u64,
    ) -> AppliedEffectReport {
        debug_assert!(
            self.committed().is_none() || !comparison.propagation_suppressed,
            "changed output authority cannot report suppressed publication"
        );
        AppliedEffectReport {
            verdict,
            comparison,
            suppressed_downstream,
            temporal_eligibility: None,
        }
    }
}

impl SignalGraph {
    pub(crate) fn apply_effect(
        &mut self,
        effect: EvaluationEffect,
        output_equivalence: OutputEquivalencePolicy,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
        defer_snapshot_commit: bool,
    ) -> Result<
        (
            AppliedEffectReport,
            Option<crate::logic::evaluation::PendingDependencySnapshot>,
        ),
        SignalError,
    > {
        let apply =
            self.build_apply_commit_packet(effect, output_equivalence, defer_snapshot_commit)?;
        let packet = self.prepare_output_commit_packet(apply, comparator_resolver)?;
        Ok(self.publish_output_commit_packet(packet))
    }

    pub(crate) fn prepare_output_commit_packet(
        &mut self,
        apply: ApplyCommitPacket,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<OutputCommitPacket, SignalError> {
        self.prepare_output_commit_packet_with_probe(apply, comparator_resolver, |_| Ok(()))
    }

    #[cfg_attr(not(test), allow(dead_code))]
    fn prepare_output_commit_packet_with_probe(
        &mut self,
        mut apply: ApplyCommitPacket,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
        mut probe: impl FnMut(OutputCommitPreparationSeam) -> Result<(), SignalError>,
    ) -> Result<OutputCommitPacket, SignalError> {
        self.apply_semantic_output_commit_decision(&mut apply, comparator_resolver)?;
        self.rebuild_semantic_artifact_write(&mut apply)?;
        probe(OutputCommitPreparationSeam::SemanticDecision)?;
        let produced_delta = self.prepare_produced_delta(&apply)?;
        probe(OutputCommitPreparationSeam::ProducedDelta)?;
        let direct_causes = match (produced_delta.as_ref(), &apply.effect.operational.verdict) {
            (Some(delta), _) => {
                Some(self.prepare_direct_output_causes(delta, comparator_resolver)?)
            }
            (None, EvaluationVerdict::Deferred { .. }) => None,
            (None, _) => {
                Some(self.prepare_stable_output_resolution(apply.effect.operational.node)?)
            }
        };
        probe(OutputCommitPreparationSeam::DirectCauseAdmission)?;
        let prepared_direct = produced_delta.map(|delta| {
            PreparedDirectInvalidation::from_semantic_decision(
                delta,
                DirectInvalidationPreparationReceipt::after_preparation(),
            )
        });
        let packet = OutputCommitPacket {
            apply,
            prepared_direct,
            direct_causes,
        };
        self.prevalidate_output_commit_packet(&packet)?;
        probe(OutputCommitPreparationSeam::PacketPrevalidation)?;
        Ok(packet)
    }

    fn rebuild_semantic_artifact_write(
        &self,
        apply: &mut ApplyCommitPacket,
    ) -> Result<(), SignalError> {
        let previous = self
            .node_runtime_artifact_reuse_boundary_snapshot(apply.effect.operational.node)?
            .map(
                |trace| crate::logic::evaluation::PreviousArtifactWarmSnapshot {
                    output_identity: trace.output_identity,
                    continuity_token: trace.continuity_token,
                    reuse_boundary_authority: trace.reuse_boundary_authority,
                },
            );
        let mut comparison = self.compare_effect(
            &apply.effect,
            previous.as_ref(),
            self.node_eval_config(apply.effect.operational.node)?
                .output_equivalence
                .clone(),
        )?;
        if matches!(
            apply.effect.operational.verdict,
            EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ComparatorMatch
                    | SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged,
            }
        ) {
            comparison.propagation_suppressed = true;
        }
        apply.comparison = comparison;
        apply.artifact_write =
            self.build_effect_artifact_write(&apply.effect, previous.as_ref(), apply.comparison)?;
        Ok(())
    }

    fn prepare_produced_delta(
        &self,
        apply: &ApplyCommitPacket,
    ) -> Result<Option<ProducedAspectDelta>, SignalError> {
        if !matches!(
            apply.effect.operational.verdict,
            EvaluationVerdict::Recomputed
        ) || apply.comparison.propagation_suppressed
        {
            return Ok(None);
        }
        let producer = apply.effect.operational.node;
        Ok(ProducedAspectDelta::from_committed_result(
            producer,
            self.cause_sets.reserve_output_commit_ordinal(),
            self.node_aspect_version(producer)?,
            apply.effect.operational.aspect_version,
            self.get_contract(producer)?.semantics.produces,
            apply.effect.changed_aspect_regions(),
            apply.effect.changed_regions(),
        ))
    }

    fn apply_semantic_output_commit_decision(
        &self,
        apply: &mut ApplyCommitPacket,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<(), SignalError> {
        let node = apply.effect.operational.node;
        let previous = self.node_aspect_version(node)?;
        if matches!(
            apply.effect.operational.verdict,
            EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ComparatorMatch
                    | SuppressionReason::OutputIdentityUnchanged
                    | SuppressionReason::ContinuityTokenUnchanged,
            }
        ) {
            apply.effect.operational.aspect_version = previous;
            apply.effect.operational.output_change = crate::data::output::OutputChange::Unchanged;
            return Ok(());
        }
        if !matches!(
            apply.effect.operational.verdict,
            EvaluationVerdict::Recomputed
        ) {
            return Ok(());
        }
        if !self.node_runtime_artifact_state_present(node)?
            && apply.effect.operational.aspect_version != previous
        {
            return Ok(());
        }
        if apply.comparison.propagation_suppressed {
            apply.effect.operational.aspect_version = previous;
            apply.effect.operational.output_change = crate::data::output::OutputChange::Unchanged;
            apply.effect.operational.verdict = EvaluationVerdict::Suppressed {
                reason: SuppressionReason::OutputIdentityUnchanged,
            };
            return Ok(());
        }
        let candidate = apply.effect.operational.aspect_version;
        let config = self.node_eval_config(node)?;
        let produces = self.get_contract(node)?.semantics.produces;
        let mut committed = previous;
        for (index, (&cached, &current)) in
            previous.slots().iter().zip(candidate.slots()).enumerate()
        {
            let aspect = Aspect::new(index as u8);
            if produces.contains(AspectMask::from_aspect(aspect))
                && config.output_equivalence.has_meaningful_change(
                    aspect,
                    cached,
                    current,
                    comparator_resolver,
                )?
            {
                committed = committed.with(aspect, current);
            }
        }
        apply.effect.operational.aspect_version = committed;
        if committed == previous {
            apply.effect.operational.output_change = crate::data::output::OutputChange::Unchanged;
            apply.effect.operational.verdict = EvaluationVerdict::Suppressed {
                reason: SuppressionReason::ComparatorMatch,
            };
        }
        Ok(())
    }

    fn prevalidate_output_commit_packet(
        &self,
        packet: &OutputCommitPacket,
    ) -> Result<(), SignalError> {
        let producer = packet.apply.effect.operational.node;
        self.validate_handle(producer)?;
        if let Some(snapshot) = packet.apply.pending_snapshot.as_ref() {
            if snapshot.node != producer {
                return Err(SignalError::internal(
                    "prepared output commit snapshot belongs to another producer",
                ));
            }
            self.validate_handle(snapshot.node)?;
        }
        if let Some(delta) = packet
            .prepared_direct
            .as_ref()
            .map(PreparedDirectInvalidation::delta)
        {
            if delta.producer != producer {
                return Err(SignalError::internal(
                    "prepared output delta belongs to another producer",
                ));
            }
        }
        if let Some(causes) = packet.direct_causes.as_ref() {
            causes.validate_packet(
                producer,
                packet
                    .prepared_direct
                    .as_ref()
                    .map(PreparedDirectInvalidation::delta),
            )?;
        }
        Ok(())
    }

    fn publish_output_commit_packet(
        &mut self,
        packet: OutputCommitPacket,
    ) -> (
        AppliedEffectReport,
        Option<crate::logic::evaluation::PendingDependencySnapshot>,
    ) {
        let OutputCommitPacket {
            apply,
            prepared_direct,
            direct_causes,
        } = packet;
        let ApplyCommitPacket {
            mut effect,
            comparison,
            artifact_write,
            pending_snapshot,
            defer_snapshot_commit,
        } = apply;
        let suppressed_downstream = direct_causes
            .as_ref()
            .map_or(0, |prepared| prepared.suppressed_downstream_count());
        self.transition_effect_state(&mut effect, artifact_write)
            .expect("prevalidated output state publication must be non-fallible");
        if !defer_snapshot_commit {
            self.commit_effect_snapshot(&mut effect)
                .expect("prevalidated dependency snapshot publication must be non-fallible");
        }
        if let Some(direct_causes) = direct_causes {
            self.publish_direct_output_causes(direct_causes)
                .expect("prevalidated direct cause publication must be non-fallible");
        }
        if let Some(delta) = prepared_direct
            .as_ref()
            .map(PreparedDirectInvalidation::delta)
        {
            self.cause_sets.publish_output_commit(delta.clone());
        }
        let publication_receipt = OutputCommitPublicationReceipt::after_atomic_publication();
        let performed = prepared_direct.map_or(PerformedOutputPublication::Stable, |prepared| {
            let publication = CommittedProducedAspectDelta::after_publication(
                prepared.delta().clone(),
                &publication_receipt,
            );
            PerformedOutputPublication::Changed(CommittedDirectInvalidation::after_publication(
                prepared,
                publication,
                &publication_receipt,
            ))
        });
        self.record_effect_telemetry(
            performed.committed(),
            &effect,
            &comparison,
            suppressed_downstream,
        );
        (
            performed.report(
                effect.operational.verdict,
                comparison,
                suppressed_downstream,
            ),
            pending_snapshot,
        )
    }

    #[cfg_attr(not(feature = "parallel"), allow(dead_code))]
    pub(crate) fn publish_prepared_parallel_apply_commit_packet(
        &mut self,
        packet: PreparedParallelApplyCommitPacket,
        comparator_resolver: &mut impl ComparatorPolicyResolver,
    ) -> Result<
        (
            AppliedEffectReport,
            Option<crate::logic::evaluation::PendingDependencySnapshot>,
        ),
        SignalError,
    > {
        let packet = self.prepare_output_commit_packet(packet.0, comparator_resolver)?;
        Ok(self.publish_output_commit_packet(packet))
    }
}

#[cfg(test)]
#[path = "output_commit_tests.rs"]
mod tests;
