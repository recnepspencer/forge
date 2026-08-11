use crate::live::{
    execute_live_change, patch_envelope_from_payload, replay_bundle_from_patch_envelope,
    BridgeChangeSummary, BridgeSliceCategory, DeliveryContractReplayRecord,
    DeliveryLocalityOutcome, LivePatchConstructionBasis, LivePatchPayload, LivePolicyCounters,
    LocalityMatchClass, LocalityMatchKind, LocalityScopeKind, LocalityWideningDecision,
    PartitionSliceMatch, RegionScopedExecutionReport, RegionScopedLiveCounters,
    RegionScopedLiveError, RegionScopedLiveExecutionEnvelope, RegionScopedLivePlan,
    RegionScopedReplayBundle, RegionSliceMatch, SuppressionReason,
};

#[cfg(test)]
fn classify_locality_match(
    plan: &RegionScopedLivePlan,
    change: &BridgeChangeSummary,
) -> Result<LocalityMatchKind, RegionScopedLiveError> {
    let expected_category = match plan.locality.scope_kind() {
        LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion,
        LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition,
    };

    let exact_match_count = change
        .locality_slices()
        .iter()
        .filter(|slice| {
            slice.category() == &expected_category && slice.scope() == plan.locality.scope()
        })
        .count();
    let peer_scopes: Vec<String> = change
        .locality_slices()
        .iter()
        .filter(|slice| {
            slice.category() == &expected_category && slice.scope() != plan.locality.scope()
        })
        .map(|slice| slice.scope().to_string())
        .collect();
    if exact_match_count > plan.locality_breadth_budget().limit() {
        return Err(RegionScopedLiveError::LocalityBreadthBudgetExceeded {
            limit: plan.locality_breadth_budget().limit(),
            actual: exact_match_count,
        });
    }
    if exact_match_count > 0 {
        if peer_scopes.is_empty() {
            return Ok(match plan.locality.scope_kind() {
                LocalityScopeKind::Region => LocalityMatchKind::InRegionRegionScope,
                LocalityScopeKind::Partition => LocalityMatchKind::InRegionPartitionScope,
            });
        }

        let mut widening_received = vec![format!(
            "{}:{}",
            expected_category.as_str(),
            plan.locality.scope()
        )];
        widening_received.extend(
            peer_scopes
                .iter()
                .map(|scope| format!("{}:{scope}", expected_category.as_str())),
        );

        if peer_scopes.len() > plan.locality_widening_budget().limit() {
            return Err(RegionScopedLiveError::WideningDenied {
                expected: format!("{}:{}", expected_category.as_str(), plan.locality.scope()),
                received: widening_received,
            });
        }

        return match (plan.locality_widening_policy(), plan.locality.scope_kind()) {
            (
                crate::live::LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice,
                LocalityScopeKind::Region,
            ) => Ok(LocalityMatchKind::InRegionRegionScopeWithPeerWidening { peer_scopes }),
            (
                crate::live::LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice,
                LocalityScopeKind::Partition,
            ) => Ok(LocalityMatchKind::InRegionPartitionScopeWithPeerWidening { peer_scopes }),
            (crate::live::LocalityWideningPolicy::DenyAll, _) => {
                Err(RegionScopedLiveError::WideningDenied {
                    expected: format!("{}:{}", expected_category.as_str(), plan.locality.scope()),
                    received: widening_received,
                })
            }
        };
    }

    let has_expected_category = change
        .locality_slices()
        .iter()
        .any(|slice| slice.category() == &expected_category);
    if has_expected_category {
        return Ok(LocalityMatchKind::OffRegionSuppressed);
    }

    let has_coarse_fallback = change
        .locality_slices()
        .iter()
        .any(|slice| slice.category() == &BridgeSliceCategory::CoarseFallback);
    if has_coarse_fallback {
        let received = change
            .locality_slices()
            .iter()
            .map(|slice| format!("{}:{}", slice.category().as_str(), slice.scope()))
            .collect();
        return Err(RegionScopedLiveError::WideningDenied {
            expected: format!("{}:{}", expected_category.as_str(), plan.locality.scope()),
            received,
        });
    }

    Err(RegionScopedLiveError::BridgeSliceIncompatibility)
}

#[cfg(test)]
pub(crate) fn execute_region_scoped_live_change(
    plan: &RegionScopedLivePlan,
    change: &BridgeChangeSummary,
) -> Result<RegionScopedLiveExecutionEnvelope, RegionScopedLiveError> {
    let locality_match = classify_locality_match(plan, change)?;
    let locality_counters = LivePolicyCounters::from_locality_match(&locality_match);

    match locality_match {
        LocalityMatchKind::InRegionRegionScope
        | LocalityMatchKind::InRegionPartitionScope
        | LocalityMatchKind::InRegionRegionScopeWithPeerWidening { .. }
        | LocalityMatchKind::InRegionPartitionScopeWithPeerWidening { .. } => {
            let mut execution = execute_live_change(plan.live(), change)?;
            let mut counters = execution.counters().clone();
            counters.absorb(&locality_counters);
            counters.add_locality_replay_change_count(1);
            let (locality_outcome, locality_match_class, widening_decision) = match &locality_match
            {
                LocalityMatchKind::InRegionRegionScope => (
                    DeliveryLocalityOutcome::InRegionRegion,
                    LocalityMatchClass::RegionMatch(RegionSliceMatch {
                        scope: plan.locality().scope().to_string(),
                        locality_digest: plan.locality().digest().as_str().to_string(),
                    }),
                    None,
                ),
                LocalityMatchKind::InRegionPartitionScope => (
                    DeliveryLocalityOutcome::InRegionPartition,
                    LocalityMatchClass::PartitionMatch(PartitionSliceMatch {
                        scope: plan.locality().scope().to_string(),
                        locality_digest: plan.locality().digest().as_str().to_string(),
                    }),
                    None,
                ),
                LocalityMatchKind::InRegionRegionScopeWithPeerWidening { peer_scopes } => (
                    DeliveryLocalityOutcome::InRegionRegionWithPeerWidening {
                        peer_scopes: peer_scopes.clone(),
                    },
                    LocalityMatchClass::RegionMatch(RegionSliceMatch {
                        scope: plan.locality().scope().to_string(),
                        locality_digest: plan.locality().digest().as_str().to_string(),
                    }),
                    Some(LocalityWideningDecision::Admitted {
                        matched_scope: plan.locality().scope().to_string(),
                        peer_scopes: peer_scopes.clone(),
                    }),
                ),
                LocalityMatchKind::InRegionPartitionScopeWithPeerWidening { peer_scopes } => (
                    DeliveryLocalityOutcome::InRegionPartitionWithPeerWidening {
                        peer_scopes: peer_scopes.clone(),
                    },
                    LocalityMatchClass::PartitionMatch(PartitionSliceMatch {
                        scope: plan.locality().scope().to_string(),
                        locality_digest: plan.locality().digest().as_str().to_string(),
                    }),
                    Some(LocalityWideningDecision::Admitted {
                        matched_scope: plan.locality().scope().to_string(),
                        peer_scopes: peer_scopes.clone(),
                    }),
                ),
                LocalityMatchKind::OffRegionSuppressed => unreachable!(),
            };
            let report = RegionScopedExecutionReport {
                query_digest: execution.report().query_digest().to_string(),
                locality_digest: plan.locality().digest().as_str().to_string(),
                locality_outcome,
                locality_match_class,
                widening_decision,
                result_digest: execution.report().result_digest().to_string(),
                delivery_digest: execution.report().delivery_digest().to_string(),
                replay_digest: execution.report().replay_digest().to_string(),
            };
            execution.counters = counters.clone();
            let mut replay_bundle = execution.replay_bundle().clone();
            replay_bundle.counter_snapshot = counters.clone();
            let replay_record =
                DeliveryContractReplayRecord::from_region_execution(&report, &replay_bundle);
            Ok(RegionScopedLiveExecutionEnvelope {
                report,
                patch_envelope: execution.patch_envelope().clone(),
                replay_bundle: RegionScopedReplayBundle {
                    locality_digest: plan.locality().digest().as_str().to_string(),
                    replay_record,
                    bundle: replay_bundle,
                },
                counters: RegionScopedLiveCounters { snapshot: counters },
            })
        }
        LocalityMatchKind::OffRegionSuppressed => {
            let payload = LivePatchPayload::Suppressed(SuppressionReason::OffRegionChange {
                scope_kind: plan.locality().scope_kind().clone(),
                scope: plan.locality().scope().to_string(),
                locality_digest: plan.locality().digest().as_str().to_string(),
            });
            let patch_envelope = patch_envelope_from_payload(
                plan.live(),
                payload,
                LivePatchConstructionBasis {
                    outcome_kind: "off_region_suppressed".to_string(),
                    outcome_digest: format!("off_region:{}", plan.locality().digest().as_str()),
                    basis_digest: plan
                        .live()
                        .progress_basis()
                        .current_basis()
                        .proof()
                        .digest()
                        .as_str()
                        .to_string(),
                    replay_digest: plan
                        .live()
                        .progress_basis()
                        .replay_digest()
                        .as_str()
                        .to_string(),
                },
            );
            let mut locality_counters = locality_counters;
            locality_counters.add_locality_replay_change_count(1);
            let replay_bundle = replay_bundle_from_patch_envelope(
                patch_envelope.clone(),
                locality_counters.clone(),
            );
            let report = RegionScopedExecutionReport {
                query_digest: plan.live().descriptor().query_digest().as_str().to_string(),
                locality_digest: plan.locality().digest().as_str().to_string(),
                locality_outcome: DeliveryLocalityOutcome::OffRegionSuppressed,
                locality_match_class: LocalityMatchClass::OffRegionSuppressed {
                    locality_digest: plan.locality().digest().as_str().to_string(),
                },
                widening_decision: None,
                result_digest: patch_envelope.result_digest().to_string(),
                delivery_digest: patch_envelope.delivery_digest().to_string(),
                replay_digest: patch_envelope.replay_digest().to_string(),
            };
            let replay_record =
                DeliveryContractReplayRecord::from_region_execution(&report, &replay_bundle);
            Ok(RegionScopedLiveExecutionEnvelope {
                report,
                patch_envelope,
                replay_bundle: RegionScopedReplayBundle {
                    locality_digest: plan.locality().digest().as_str().to_string(),
                    replay_record,
                    bundle: replay_bundle,
                },
                counters: RegionScopedLiveCounters {
                    snapshot: locality_counters,
                },
            })
        }
    }
}
