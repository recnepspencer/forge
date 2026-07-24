use std::collections::{HashMap, HashSet};

use crate::basis_lifecycle::BasisOperationLane;
use crate::domain_installation::{
    WorthQueryBoundCommitPosture, WorthQueryBoundDomainOperation,
    WorthQueryBoundGraphExecutionReceipt, WorthQueryGraphProviderCallKind,
};

use super::super::compiled::{
    WorthQueryCompiledSemanticAspectDependency, WorthQuerySemanticAspectDependencyLocus,
    WorthQuerySemanticAspectDependencySource, WorthQuerySemanticDependencyRole,
};
use super::graph_read_access::WorthQueryCompiledGraphReadAccess;
use super::operation_definition::SemanticAspectDependencyCompilation;

type CommitAuthorityIdentity = (u64, std::any::TypeId);
type CommitGroups = HashMap<CommitAuthorityIdentity, HashSet<String>>;

#[derive(Clone)]
struct RealizedCall {
    role: String,
    kind: WorthQueryGraphProviderCallKind,
    commit_authority: Option<CommitAuthorityIdentity>,
    commit_roles: Vec<String>,
}

type DeclaredRead = (String, WorthQueryCompiledGraphReadAccess);

impl SemanticAspectDependencyCompilation {
    pub(super) fn push_realized_graph_call(
        &mut self,
        locus: WorthQuerySemanticAspectDependencyLocus,
        receipt: &WorthQueryBoundGraphExecutionReceipt,
    ) {
        let role = super::graph_read_access::WorthQueryCompiledGraphReadAccess::from_realized(
            receipt.kind(),
        )
        .map_or(
            WorthQuerySemanticDependencyRole::SupportAndLifecycle,
            |access| access.dependency_role(),
        );
        self.dependencies
            .push(WorthQueryCompiledSemanticAspectDependency::new(
                locus,
                role,
                WorthQuerySemanticAspectDependencySource::RealizedGraphCall {
                    role: receipt.role().to_owned(),
                    call_kind: receipt.kind(),
                    evidence_identity: receipt.evidence_identity().to_owned(),
                    projection_result_digest: receipt
                        .graph_read_product()
                        .map(|projection| projection.result_digest().to_owned()),
                    commit_graph_roles: receipt.commit_graph_roles().to_vec(),
                },
            ));
        self.counters.realized_graph_call_edges += 1;
    }
}

pub(super) fn realized_calls_match<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    active_read_roles: Option<&[String]>,
    touch_roles: &[String],
    receipts: &[WorthQueryBoundGraphExecutionReceipt],
) -> bool {
    let declared_reads = bound
        .definition()
        .semantics()
        .graph_reads
        .roles()
        .iter()
        .filter(|read| {
            matches!(
                read.participation,
                crate::domain_installation::WorthQueryOperationGraphParticipation::SeparateAuthority { .. }
            ) && active_read_roles.is_none_or(|roles| roles.contains(&read.role))
        })
        .map(|read| {
            (
                read.role.clone(),
                super::graph_read_access::WorthQueryCompiledGraphReadAccess::from_declared(
                    read.access,
                ),
            )
        })
        .collect::<Vec<_>>();
    let expected_commits = expected_commit_groups(bound, touch_roles);
    let realized = receipts
        .iter()
        .map(|receipt| RealizedCall {
            role: receipt.role().to_owned(),
            kind: receipt.kind(),
            commit_authority: receipt.commit_authority_identity(),
            commit_roles: receipt.commit_graph_roles().to_vec(),
        })
        .collect::<Vec<_>>();
    exact_call_contract_matches(&declared_reads, touch_roles, &expected_commits, &realized)
}

fn expected_commit_groups<D, O, F, L: BasisOperationLane>(
    bound: &WorthQueryBoundDomainOperation<D, O, F, L>,
    touch_roles: &[String],
) -> CommitGroups {
    let mut expected = HashMap::new();
    if bound.commit_posture() != WorthQueryBoundCommitPosture::Atomic {
        return expected;
    }
    for participation in bound
        .graph_participations()
        .iter()
        .filter(|participation| touch_roles.contains(&participation.role))
    {
        let Some(authority) = &participation.record.commit_authority else {
            continue;
        };
        expected
            .entry(authority.identity())
            .or_insert_with(HashSet::new)
            .insert(participation.role.clone());
    }
    expected
}

fn exact_call_contract_matches(
    declared_reads: &[DeclaredRead],
    touch_roles: &[String],
    expected_commits: &CommitGroups,
    realized: &[RealizedCall],
) -> bool {
    let declared_read_map = declared_reads.iter().cloned().collect::<HashMap<_, _>>();
    let realized_reads = realized
        .iter()
        .filter(|call| {
            matches!(
                call.kind,
                WorthQueryGraphProviderCallKind::Observe | WorthQueryGraphProviderCallKind::Project
            )
        })
        .filter_map(|call| {
            super::graph_read_access::WorthQueryCompiledGraphReadAccess::from_realized(call.kind)
                .map(|access| (call.role.clone(), access))
        })
        .collect::<HashMap<_, _>>();
    let realized_read_count = realized
        .iter()
        .filter(|call| {
            matches!(
                call.kind,
                WorthQueryGraphProviderCallKind::Observe | WorthQueryGraphProviderCallKind::Project
            )
        })
        .count();
    let expected_touches = touch_roles.iter().cloned().collect::<HashSet<_>>();
    let realized_touches = realized
        .iter()
        .filter(|call| call.kind == WorthQueryGraphProviderCallKind::TouchEffect)
        .map(|call| call.role.clone())
        .collect::<HashSet<_>>();
    let realized_touch_count = realized
        .iter()
        .filter(|call| call.kind == WorthQueryGraphProviderCallKind::TouchEffect)
        .count();

    declared_read_map.len() == declared_reads.len()
        && realized_reads.len() == realized_read_count
        && declared_read_map == realized_reads
        && expected_touches.len() == touch_roles.len()
        && realized_touches.len() == realized_touch_count
        && expected_touches == realized_touches
        && noncommit_metadata_is_empty(realized)
        && realized_commit_groups(realized).as_ref() == Some(expected_commits)
}

fn noncommit_metadata_is_empty(realized: &[RealizedCall]) -> bool {
    realized.iter().all(|call| {
        call.kind == WorthQueryGraphProviderCallKind::CommitAdmission
            || (call.commit_authority.is_none() && call.commit_roles.is_empty())
    })
}

fn realized_commit_groups(realized: &[RealizedCall]) -> Option<CommitGroups> {
    let commit_calls = realized
        .iter()
        .filter(|call| call.kind == WorthQueryGraphProviderCallKind::CommitAdmission);
    let mut actual = HashMap::new();
    for call in commit_calls {
        let authority = call.commit_authority?;
        let roles = call.commit_roles.iter().cloned().collect::<HashSet<_>>();
        if roles.is_empty()
            || roles.len() != call.commit_roles.len()
            || actual.insert(authority, roles).is_some()
        {
            return None;
        }
    }
    Some(actual)
}

#[cfg(test)]
mod tests {
    use super::*;

    struct GraphA;
    struct GraphB;

    fn authority<T: 'static>() -> CommitAuthorityIdentity {
        (7, std::any::TypeId::of::<T>())
    }

    fn call(
        role: &str,
        kind: WorthQueryGraphProviderCallKind,
        commit_authority: Option<CommitAuthorityIdentity>,
        commit_roles: &[&str],
    ) -> RealizedCall {
        RealizedCall {
            role: role.into(),
            kind,
            commit_authority,
            commit_roles: commit_roles.iter().map(|role| (*role).into()).collect(),
        }
    }

    fn valid_contract() -> (
        Vec<DeclaredRead>,
        Vec<String>,
        CommitGroups,
        Vec<RealizedCall>,
    ) {
        let reads = vec![("input".into(), WorthQueryCompiledGraphReadAccess::Observe)];
        let touches = vec!["output".into()];
        let commits = HashMap::from([(authority::<GraphA>(), HashSet::from(["output".into()]))]);
        let calls = vec![
            call("input", WorthQueryGraphProviderCallKind::Observe, None, &[]),
            call(
                "output",
                WorthQueryGraphProviderCallKind::TouchEffect,
                None,
                &[],
            ),
            call(
                "commit",
                WorthQueryGraphProviderCallKind::CommitAdmission,
                Some(authority::<GraphA>()),
                &["output"],
            ),
        ];
        (reads, touches, commits, calls)
    }

    fn admits(
        parts: &(
            Vec<DeclaredRead>,
            Vec<String>,
            CommitGroups,
            Vec<RealizedCall>,
        ),
    ) -> bool {
        exact_call_contract_matches(&parts.0, &parts.1, &parts.2, &parts.3)
    }

    #[test]
    fn exact_graph_call_multiset_rejects_omission_duplication_and_authority_drift() {
        let valid = valid_contract();
        assert!(admits(&valid));

        let mut missing_read = valid_contract();
        missing_read.3.remove(0);
        assert!(!admits(&missing_read));

        let mut duplicate_read = valid_contract();
        duplicate_read.3.insert(
            1,
            call("input", WorthQueryGraphProviderCallKind::Observe, None, &[]),
        );
        assert!(!admits(&duplicate_read));

        let mut missing_touch = valid_contract();
        missing_touch.3.remove(1);
        assert!(!admits(&missing_touch));

        let mut duplicate_touch = valid_contract();
        duplicate_touch.3.insert(
            2,
            call(
                "output",
                WorthQueryGraphProviderCallKind::TouchEffect,
                None,
                &[],
            ),
        );
        assert!(!admits(&duplicate_touch));

        let mut wrong_authority = valid_contract();
        wrong_authority.3[2].commit_authority = Some(authority::<GraphB>());
        assert!(!admits(&wrong_authority));

        let mut wrong_group = valid_contract();
        wrong_group.3[2].commit_roles = vec!["other".into()];
        assert!(!admits(&wrong_group));

        let mut contaminated = valid_contract();
        contaminated.3[0].commit_authority = Some(authority::<GraphA>());
        contaminated.3[0].commit_roles = vec!["output".into()];
        assert!(!admits(&contaminated));
    }
}
