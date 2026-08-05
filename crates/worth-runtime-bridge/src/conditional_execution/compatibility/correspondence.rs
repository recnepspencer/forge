use crate::correspondence::BridgeInstalledSemanticCorrespondence;

use super::BridgeConditionalComparisonWork;
use super::{BridgeConditionalContinuityMismatch, BridgeConditionalExecutionAffinityMismatch};

pub(super) fn compare_semantic_correspondences(
    current: &[BridgeInstalledSemanticCorrespondence],
    candidate: &[BridgeInstalledSemanticCorrespondence],
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalContinuityMismatch> {
    if current.len() != candidate.len() {
        return Err(BridgeConditionalContinuityMismatch::CorrespondenceCount);
    }
    for (position, (current, candidate)) in current.iter().zip(candidate).enumerate() {
        work.inspect_correspondence();
        let current_dependency = current.dependency();
        let candidate_dependency = candidate.dependency();
        let ordinal = current_dependency.dependency_ordinal();
        if ordinal != candidate_dependency.dependency_ordinal() || ordinal != position {
            return Err(BridgeConditionalContinuityMismatch::DependencyOrdinal { ordinal });
        }
        if current_dependency.contract != candidate_dependency.contract
            || current_dependency.projection_mask != candidate_dependency.projection_mask
            || current_dependency.binding != candidate_dependency.binding
            || current_dependency.locality != candidate_dependency.locality
            || current_dependency.relevant_changes != candidate_dependency.relevant_changes
            || current_dependency.declared_graph_role != candidate_dependency.declared_graph_role
        {
            return Err(BridgeConditionalContinuityMismatch::DependencyMeaning { ordinal });
        }
        if current_dependency.source_record_identity != candidate_dependency.source_record_identity
        {
            return Err(BridgeConditionalContinuityMismatch::DependencySource { ordinal });
        }
        let current_basis = current.basis();
        let candidate_basis = candidate.basis();
        if current_basis.graph_adapter_identity != candidate_basis.graph_adapter_identity {
            return Err(BridgeConditionalContinuityMismatch::GraphAdapter { ordinal });
        }
        if current_basis.authoritative_source_profile
            != candidate_basis.authoritative_source_profile
        {
            return Err(BridgeConditionalContinuityMismatch::SourceProfile { ordinal });
        }
        let current_targets = current.targets.as_slice();
        let candidate_targets = candidate.targets.as_slice();
        if current_targets.len() != candidate_targets.len() {
            return Err(BridgeConditionalContinuityMismatch::TargetCount { ordinal });
        }
        for (target, (current, candidate)) in
            current_targets.iter().zip(candidate_targets).enumerate()
        {
            work.inspect_target();
            if current.mapping_identity != candidate.mapping_identity
                || current.partition != candidate.partition
                || current.precision != candidate.precision
                || current.admitted_source_widening != candidate.admitted_source_widening
            {
                return Err(BridgeConditionalContinuityMismatch::TargetMeaning { ordinal, target });
            }
        }
    }
    Ok(())
}

pub(super) fn compare_exact_correspondences(
    current: &[BridgeInstalledSemanticCorrespondence],
    candidate: &[BridgeInstalledSemanticCorrespondence],
    work: &mut BridgeConditionalComparisonWork,
) -> Result<(), BridgeConditionalExecutionAffinityMismatch> {
    for (current, candidate) in current.iter().zip(candidate) {
        work.inspect_correspondence();
        let ordinal = current.dependency().dependency_ordinal();
        let current_basis = current.basis();
        let candidate_basis = candidate.basis();
        if current_basis.source_runtime_authority != candidate_basis.source_runtime_authority
            || current_basis.source_installation_generation
                != candidate_basis.source_installation_generation
            || current_basis.source_installation_identity
                != candidate_basis.source_installation_identity
            || current_basis.source_authority_binding_identity
                != candidate_basis.source_authority_binding_identity
            || current_basis.source_basis != candidate_basis.source_basis
            || current_basis.bridge_runtime_key != candidate_basis.bridge_runtime_key
        {
            return Err(
                BridgeConditionalExecutionAffinityMismatch::SourceCorrespondenceAuthority {
                    ordinal,
                },
            );
        }
        if current_basis.graph_adapter_identity != candidate_basis.graph_adapter_identity {
            return Err(BridgeConditionalExecutionAffinityMismatch::GraphAuthority { ordinal });
        }
        if current_basis.graph_participation_identity
            != candidate_basis.graph_participation_identity
            || current_basis.declared_graph_role != candidate_basis.declared_graph_role
        {
            return Err(
                BridgeConditionalExecutionAffinityMismatch::GraphParticipationAuthority { ordinal },
            );
        }
        if current_basis.signal_graph_instance_id != candidate_basis.signal_graph_instance_id
            || current_basis.signal_partitions != candidate_basis.signal_partitions
        {
            return Err(BridgeConditionalExecutionAffinityMismatch::SignalGraphBinding { ordinal });
        }
        for (target, (current, candidate)) in current
            .targets
            .as_slice()
            .iter()
            .zip(candidate.targets.as_slice())
            .enumerate()
        {
            work.inspect_target();
            if current != candidate {
                return Err(BridgeConditionalExecutionAffinityMismatch::TargetAffinity {
                    ordinal,
                    target,
                });
            }
        }
    }
    Ok(())
}
