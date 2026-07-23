use std::collections::{BTreeMap, BTreeSet};

use super::*;

pub(crate) fn route_live_subscription_delivery(
    active_subscriptions: &mut ActiveSubscriptionRuntime,
    live_subscriptions: &mut BTreeMap<
        WorthQueryLiveArtifactTarget,
        WorthQueryRuntimeLiveSubscriptionState,
    >,
    live_subscription_index: &crate::runtime::live_subscription_target_index::WorthQueryLiveSubscriptionTargetIndex,
    installed_live_routes: &crate::runtime::installed_live_routing::WorthQueryInstalledLiveRoutes,
    receipt: &WorthQueryMutationReceipt,
) -> Result<Vec<WorthQueryLiveArtifactTarget>, WorthQueryRuntimeError> {
    let mut affected = Vec::new();
    for delta in &receipt.deltas {
        let installed_selection = installed_live_routes.affected_targets(delta);
        let live_selection = live_subscription_index.affected_targets(delta);
        let (targets, union_overlap) =
            merge_selected_targets(live_selection.targets, &installed_selection.targets);
        let mut classified = Vec::with_capacity(targets.len());
        let mut installed_candidates_skipped = 0;
        for target in targets {
            if !live_subscriptions.contains_key(&target) {
                return Err(WorthQueryRuntimeError::MissingLiveSubscription(
                    target.view_name().to_owned(),
                ));
            }
            let installed = installed_live_routes.classify_live_mutation(
                &target,
                delta,
                &installed_selection.targets,
            );
            if installed.is_installed_but_unaffected() {
                installed_candidates_skipped += 1;
                continue;
            }
            classified.push((target, installed.into_impact()));
        }
        let mut shared_work = selection_work(
            live_selection.work,
            installed_selection.work,
            installed_candidates_skipped,
            union_overlap,
        );
        for (target, preclassified) in classified {
            let state = live_subscriptions
                .get_mut(&target)
                .expect("selected live target remains registered");
            let delta_kind = match &preclassified {
                Some(impact) => {
                    maintenance_delta_kind_for_classified_impact(impact.class(), &state.request)
                }
                None => maintenance_delta_kind_for_live_change(
                    &state.request,
                    &delta.kind,
                    delta.admitted_touched_aspects(),
                ),
            };
            let Some(delta_kind) = delta_kind else {
                shared_work.installed_candidates_skipped += usize::from(preclassified.is_some());
                continue;
            };
            let mut routing_work = WorthQueryLiveMutationRoutingWork {
                capability_index_lookups: usize::from(preclassified.is_some()),
                live_target_candidates_visited: 1,
                installed_route_index_probes: 1,
                ..Default::default()
            };
            routing_work.add(shared_work);
            shared_work = Default::default();
            RelevantLiveSubscriptionDeltaRoute {
                active_subscriptions,
                state,
                target: &target,
                receipt,
                delta,
                delta_kind,
                preclassified_installed_impact: preclassified,
                routing_work,
                affected: &mut affected,
            }
            .route()?;
        }
    }
    affected.sort();
    affected.dedup();
    Ok(affected)
}

fn selection_work(
    live: crate::runtime::live_subscription_target_index::WorthQueryLiveTargetSelectionWork,
    installed: crate::runtime::installed_live_routing::WorthQueryInstalledTargetSelectionWork,
    installed_candidates_skipped: usize,
    union_overlap: usize,
) -> WorthQueryLiveMutationRoutingWork {
    WorthQueryLiveMutationRoutingWork {
        live_collection_index_probes: live.collection_index_probes,
        live_relevance_index_probes: live.relevance_index_probes,
        installed_collection_index_probes: installed.collection_index_probes,
        installed_relevance_index_probes: installed.relevance_index_probes,
        installed_target_candidates_selected: installed.candidates_selected,
        installed_candidates_skipped,
        target_overlap_deduplications: live.overlap_deduplications
            + installed.overlap_deduplications
            + union_overlap,
        ..Default::default()
    }
}

fn merge_selected_targets(
    mut live: BTreeSet<WorthQueryLiveArtifactTarget>,
    installed: &BTreeSet<WorthQueryLiveArtifactTarget>,
) -> (BTreeSet<WorthQueryLiveArtifactTarget>, usize) {
    let mut overlap = 0;
    for target in installed {
        overlap += usize::from(!live.insert(target.clone()));
    }
    (live, overlap)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn installed_semantic_target_survives_absence_from_ordinary_view_routes() {
        let ordinary = BTreeSet::from([WorthQueryLiveArtifactTarget::from_view_name(
            "ordinary-only",
        )]);
        let installed_only = WorthQueryLiveArtifactTarget::from_view_name("installed-only");
        let installed = BTreeSet::from([installed_only.clone()]);

        let (merged, overlap) = merge_selected_targets(ordinary, &installed);
        assert!(merged.contains(&installed_only));
        assert_eq!(merged.len(), 2);
        assert_eq!(overlap, 0);
    }
}
