use super::*;
#[cfg(test)]
pub(crate) fn admit_region_scoped_live_plan(
    live: &LiveQueryPlan,
    locality: LocalityPredicateContract,
) -> Result<RegionScopedLivePlan, RegionScopedLiveError> {
    let semantic_basis = derive_locality_semantic_basis(live.descriptor().relevance_contract());
    let scope_admission = derive_locality_scope_admission(live.descriptor().relevance_contract());
    let admission_class =
        derive_locality_admission_class(&semantic_basis, &scope_admission, &locality)?;
    let stream_lowering_admission =
        derive_stream_lowering_admission_class(&semantic_basis, &admission_class);

    let locality_digest = locality.digest().as_str().to_string();
    let query_digest = live.descriptor().query_digest().as_str().to_string();
    let locality_subscription_digest = hash_parts(&[
        format!("subscription:{}", live.subscription_digest().as_str()),
        format!("locality:{}", locality_digest),
        format!("admission:{}", admission_class.as_str()),
    ]);

    let (
        locality_cost_posture,
        locality_breadth_budget,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
        stream_window_width_budget,
    ) = match admission_class {
        LocalityAdmissionClass::DetailRegion | LocalityAdmissionClass::DetailPartition => (
            LocalityCostPosture::SingleSliceNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget { limit: 1 },
            StreamLoweringCostPosture::SingleDetailCurrentStateMember,
            StreamMemberWidthBudget::single_member(),
            StreamWindowWidthBudget::single_window(),
        ),
        LocalityAdmissionClass::OrderedCollectionPartition => (
            LocalityCostPosture::PartitionScopedMembershipNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::CdcPatchWithProjectedDeltas,
            StreamMemberWidthBudget::cdc_projected_patch(),
            StreamWindowWidthBudget::single_window(),
        ),
        LocalityAdmissionClass::BoundedMaterializationRegion => (
            LocalityCostPosture::BoundedTraversalRegionNarrowing,
            LocalityBreadthBudget::single_surface(),
            LocalityWideningBudget::deny_all(),
            StreamLoweringCostPosture::BoundedMaterializationDeferred,
            StreamMemberWidthBudget::single_member(),
            StreamWindowWidthBudget::single_window(),
        ),
    };

    let subscription_identity = RegionScopedSubscriptionIdentity {
        digest: locality_subscription_digest.clone(),
        query_digest: query_digest.clone(),
        locality_digest: locality_digest.clone(),
        admission_class: admission_class.clone(),
    };
    let relevance_contract = LocalityAwareRelevanceContract {
        digest: hash_parts(&[
            format!("query:{query_digest}"),
            format!("locality:{locality_digest}"),
            format!("admission:{}", admission_class.as_str()),
            format!("semantic_basis:{}", semantic_basis.as_str()),
            format!("scope_admission:{}", scope_admission.as_str()),
            format!(
                "maintenance:{}",
                LocalityMaintenanceClass::NarrowPatch.as_str()
            ),
            format!(
                "stream_lowering_admission:{}",
                stream_lowering_admission.as_str()
            ),
            format!(
                "slice_category:{}",
                match locality.scope_kind() {
                    LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion.as_str(),
                    LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition.as_str(),
                }
            ),
        ]),
        locality_digest: locality_digest.clone(),
        admission_class: admission_class.clone(),
        semantic_basis: semantic_basis.clone(),
        scope_admission: scope_admission.clone(),
        maintenance_class: LocalityMaintenanceClass::NarrowPatch,
        stream_lowering_admission: stream_lowering_admission.clone(),
        expected_slice_category: match locality.scope_kind() {
            LocalityScopeKind::Region => BridgeSliceCategory::EntityRegion,
            LocalityScopeKind::Partition => BridgeSliceCategory::EntityPartition,
        },
    };
    let locality_widening_policy = match admission_class {
        LocalityAdmissionClass::DetailRegion | LocalityAdmissionClass::DetailPartition => {
            LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice
        }
        LocalityAdmissionClass::OrderedCollectionPartition
        | LocalityAdmissionClass::BoundedMaterializationRegion => LocalityWideningPolicy::DenyAll,
    };
    let locality_performance_status = LocalityPerformanceStatus::VerifiedNarrowing;
    let planning_report = RegionScopedPlanningReport {
        query_digest: query_digest.clone(),
        locality_digest: locality_digest.clone(),
        subscription_identity_digest: subscription_identity.digest().to_string(),
        relevance_contract_digest: relevance_contract.digest().to_string(),
        semantic_basis,
        scope_admission,
        stream_lowering_admission,
        widening_policy: locality_widening_policy.clone(),
        performance_status: locality_performance_status.clone(),
    };

    Ok(RegionScopedLivePlan {
        live: live.clone(),
        locality,
        admission_class,
        subscription_identity,
        relevance_contract,
        planning_report,
        locality_cost_posture,
        locality_performance_status,
        locality_breadth_budget,
        locality_widening_policy,
        locality_widening_budget,
        stream_lowering_cost_posture,
        stream_member_width_budget,
        stream_window_width_budget,
    })
}
#[cfg(test)]
fn derive_locality_semantic_basis(
    relevance_contract: &QueryRelevanceContract,
) -> LocalitySemanticBasis {
    if !relevance_contract.traversal_relations().is_empty() {
        LocalitySemanticBasis::BoundedTraversalMaterialization
    } else if relevance_contract.family() == &LiveQueryFamily::OrderedCollection
        || !relevance_contract.ordering_fields().is_empty()
    {
        LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering
    } else {
        LocalitySemanticBasis::DetailProjectionFields
    }
}
#[cfg(test)]
fn derive_locality_scope_admission(
    relevance_contract: &QueryRelevanceContract,
) -> LocalityScopeAdmission {
    match derive_locality_semantic_basis(relevance_contract) {
        LocalitySemanticBasis::DetailProjectionFields => LocalityScopeAdmission::RegionOrPartition,
        LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering => {
            LocalityScopeAdmission::PartitionOnly
        }
        LocalitySemanticBasis::BoundedTraversalMaterialization => {
            LocalityScopeAdmission::RegionOnly
        }
    }
}
#[cfg(test)]
fn derive_locality_admission_class(
    semantic_basis: &LocalitySemanticBasis,
    scope_admission: &LocalityScopeAdmission,
    locality: &LocalityPredicateContract,
) -> Result<LocalityAdmissionClass, RegionScopedLiveError> {
    match (semantic_basis, scope_admission, locality.scope_kind()) {
        (
            LocalitySemanticBasis::DetailProjectionFields,
            LocalityScopeAdmission::RegionOrPartition,
            LocalityScopeKind::Region,
        ) => Ok(LocalityAdmissionClass::DetailRegion),
        (
            LocalitySemanticBasis::DetailProjectionFields,
            LocalityScopeAdmission::RegionOrPartition,
            LocalityScopeKind::Partition,
        ) => Ok(LocalityAdmissionClass::DetailPartition),
        (
            LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering,
            LocalityScopeAdmission::PartitionOnly,
            LocalityScopeKind::Partition,
        ) => Ok(LocalityAdmissionClass::OrderedCollectionPartition),
        (
            LocalitySemanticBasis::BoundedTraversalMaterialization,
            LocalityScopeAdmission::RegionOnly,
            LocalityScopeKind::Region,
        ) => Ok(LocalityAdmissionClass::BoundedMaterializationRegion),
        _ => Err(RegionScopedLiveError::UnsupportedLocalityPredicate),
    }
}
#[cfg(test)]
fn derive_stream_lowering_admission_class(
    semantic_basis: &LocalitySemanticBasis,
    admission_class: &LocalityAdmissionClass,
) -> StreamLoweringAdmissionClass {
    match (semantic_basis, admission_class) {
        (LocalitySemanticBasis::DetailProjectionFields, _)
        | (_, LocalityAdmissionClass::DetailRegion)
        | (_, LocalityAdmissionClass::DetailPartition) => {
            StreamLoweringAdmissionClass::DetailCurrentStateOnly
        }
        (
            LocalitySemanticBasis::OrderedCollectionMembershipAndOrdering,
            LocalityAdmissionClass::OrderedCollectionPartition,
        ) => StreamLoweringAdmissionClass::CollectionCdcProjectedPatchOnly,
        (
            LocalitySemanticBasis::BoundedTraversalMaterialization,
            LocalityAdmissionClass::BoundedMaterializationRegion,
        ) => StreamLoweringAdmissionClass::DeferredBoundedMaterialization,
        _ => StreamLoweringAdmissionClass::DetailCurrentStateOnly,
    }
}
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
                LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice,
                LocalityScopeKind::Region,
            ) => Ok(LocalityMatchKind::InRegionRegionScopeWithPeerWidening { peer_scopes }),
            (
                LocalityWideningPolicy::AllowExactMatchWithSinglePeerSlice,
                LocalityScopeKind::Partition,
            ) => Ok(LocalityMatchKind::InRegionPartitionScopeWithPeerWidening { peer_scopes }),
            (LocalityWideningPolicy::DenyAll, _) => Err(RegionScopedLiveError::WideningDenied {
                expected: format!("{}:{}", expected_category.as_str(), plan.locality.scope()),
                received: widening_received,
            }),
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
                "off_region_suppressed".to_string(),
                format!("off_region:{}", plan.locality().digest().as_str()),
                plan.live()
                    .progress_basis()
                    .current_basis()
                    .proof()
                    .digest()
                    .as_str()
                    .to_string(),
                plan.live()
                    .progress_basis()
                    .replay_digest()
                    .as_str()
                    .to_string(),
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
#[cfg(test)]
pub(crate) fn lower_region_scoped_execution_to_stream_contract(
    plan: &RegionScopedLivePlan,
    execution: &RegionScopedLiveExecutionEnvelope,
    consumer_shape: StreamConsumerShape,
) -> Result<StreamLoweredDeliveryContract, RegionScopedLiveError> {
    match (plan.live().descriptor().family(), &consumer_shape) {
        (LiveQueryFamily::Detail, StreamConsumerShape::DetailCurrentState)
        | (LiveQueryFamily::OrderedCollection, StreamConsumerShape::CdcCollectionPatch) => {}
        _ => return Err(RegionScopedLiveError::UnsupportedStreamConsumerShape),
    }

    let query_delivery_contract = QueryDeliveryContract {
        digest: hash_parts(&[
            format!("query:{}", execution.report().query_digest()),
            format!("locality:{}", plan.locality().digest().as_str()),
            format!("delivery:{}", execution.report().delivery_digest()),
            format!("family:{}", execution.patch_envelope().family().as_str()),
            format!(
                "locality_outcome:{}",
                DeliveryLocalityOutcome::from_region_scoped_report(execution.report()).as_str()
            ),
        ]),
        query_digest: execution.report().query_digest().to_string(),
        locality_digest: plan.locality().digest().as_str().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        family: execution.patch_envelope().family().clone(),
        locality_outcome: DeliveryLocalityOutcome::from_region_scoped_report(execution.report()),
    };
    let request = StreamContractRequest {
        digest: hash_parts(&[
            format!("query:{}", execution.report().query_digest()),
            format!("delivery:{}", execution.report().delivery_digest()),
            format!("consumer_shape:{}", consumer_shape.as_str()),
        ]),
        query_digest: execution.report().query_digest().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        consumer_shape: consumer_shape.clone(),
    };
    let admitted_consumer_contract = AdmittedStreamConsumerContract {
        digest: hash_parts(&[
            format!("query:{}", request.query_digest()),
            format!("delivery:{}", request.delivery_digest()),
            format!("consumer_shape:{}", request.consumer_shape().as_str()),
        ]),
        consumer_shape: consumer_shape.clone(),
    };
    let (member_count, window_width, delivery_width) = stream_contract_widths(
        execution.patch_envelope().payload(),
        execution.report().locality_outcome(),
        &consumer_shape,
    );
    if window_width > plan.stream_window_width_budget().limit() {
        return Err(RegionScopedLiveError::StreamWindowWidthBudgetExceeded {
            limit: plan.stream_window_width_budget().limit(),
            actual: window_width,
        });
    }
    if delivery_width > plan.stream_member_width_budget().limit() {
        return Err(RegionScopedLiveError::StreamMemberWidthBudgetExceeded {
            limit: plan.stream_member_width_budget().limit(),
            actual: delivery_width,
        });
    }
    let member_projection = StreamMemberProjection {
        digest: hash_parts(&[
            format!("consumer_shape:{}", consumer_shape.as_str()),
            format!("member_count:{member_count}"),
            format!("delivery_width:{delivery_width}"),
        ]),
        consumer_shape: consumer_shape.clone(),
        member_count,
        delivery_width,
    };
    let window_compatibility = StreamWindowCompatibility {
        digest: hash_parts(&[
            format!("consumer_shape:{}", consumer_shape.as_str()),
            format!("window_width:{window_width}"),
            format!("budget_limit:{}", plan.stream_window_width_budget().limit()),
        ]),
        consumer_shape: consumer_shape.clone(),
        window_width,
        budget_limit: plan.stream_window_width_budget().limit(),
    };
    let stream_contract_digest = StreamContractDigest(hash_parts(&[
        format!("query_delivery:{}", query_delivery_contract.digest()),
        format!("request:{}", request.digest()),
        format!("admitted_consumer:{}", admitted_consumer_contract.digest()),
        format!("members:{member_count}"),
        format!("window_width:{window_width}"),
        format!("width:{delivery_width}"),
        format!(
            "cost_posture:{}",
            plan.stream_lowering_cost_posture().as_str()
        ),
    ]));
    let delivery_contract_lowering = DeliveryContractLowering {
        digest: hash_parts(&[
            format!("query_delivery:{}", query_delivery_contract.digest()),
            format!("request:{}", request.digest()),
            format!("admitted_consumer:{}", admitted_consumer_contract.digest()),
            format!("stream_contract:{}", stream_contract_digest.as_str()),
        ]),
        query_delivery_digest: query_delivery_contract.digest().to_string(),
        request_digest: request.digest().to_string(),
        admitted_consumer_contract_digest: admitted_consumer_contract.digest().to_string(),
        stream_contract_digest: stream_contract_digest.as_str().to_string(),
    };
    let replay_record = execution
        .region_scoped_replay_bundle()
        .replay_record()
        .with_stream_contract_digest(stream_contract_digest.as_str());
    let mut counter_snapshot = execution.counters().clone();
    counter_snapshot.absorb(&LivePolicyCounters::from_stream_lowered_delivery(
        &StreamLoweredDeliveryContract {
            query_digest: execution.report().query_digest().to_string(),
            locality_digest: plan.locality().digest().as_str().to_string(),
            delivery_digest: execution.report().delivery_digest().to_string(),
            query_delivery_contract: query_delivery_contract.clone(),
            stream_contract_digest: stream_contract_digest.clone(),
            delivery_contract_lowering: delivery_contract_lowering.clone(),
            request: request.clone(),
            admitted_consumer_contract: admitted_consumer_contract.clone(),
            member_projection: member_projection.clone(),
            window_compatibility: window_compatibility.clone(),
            replay_record: replay_record.clone(),
            counter_snapshot: LivePolicyCounters::default(),
            member_count,
            window_width,
            delivery_width,
            cost_posture: plan.stream_lowering_cost_posture().clone(),
        },
    ));

    Ok(StreamLoweredDeliveryContract {
        query_digest: execution.report().query_digest().to_string(),
        locality_digest: plan.locality().digest().as_str().to_string(),
        delivery_digest: execution.report().delivery_digest().to_string(),
        query_delivery_contract,
        stream_contract_digest,
        delivery_contract_lowering,
        request,
        admitted_consumer_contract,
        member_projection,
        window_compatibility,
        replay_record,
        counter_snapshot,
        member_count,
        window_width,
        delivery_width,
        cost_posture: plan.stream_lowering_cost_posture().clone(),
    })
}
#[cfg(test)]
fn stream_contract_widths(
    payload: &LivePatchPayload,
    locality_outcome: &DeliveryLocalityOutcome,
    consumer_shape: &StreamConsumerShape,
) -> (usize, usize, usize) {
    match (payload, consumer_shape) {
        (LivePatchPayload::Detail(_), StreamConsumerShape::DetailCurrentState) => {
            let window_width = match locality_outcome {
                DeliveryLocalityOutcome::InRegionRegionWithPeerWidening { peer_scopes }
                | DeliveryLocalityOutcome::InRegionPartitionWithPeerWidening { peer_scopes } => {
                    1 + peer_scopes.len()
                }
                _ => 1,
            };
            (1, window_width, 1)
        }
        (LivePatchPayload::OrderedCollection(patch), StreamConsumerShape::CdcCollectionPatch) => {
            (1, 1, 1 + patch.projected_field_deltas().len())
        }
        (LivePatchPayload::Suppressed(_), _) => (1, 1, 1),
        _ => (1, 1, 1),
    }
}
