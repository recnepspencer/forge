use std::collections::BTreeMap;
use std::panic::{catch_unwind, AssertUnwindSafe};
use std::sync::Arc;

use crate::capabilities::{AspectPlanSource, SchemaSource};
use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, CommitStrategyDescriptorDigest,
    CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyId,
    StrategyExecutionDraft, StrategyExecutorFailure, StrategyObservationContext,
};
use crate::logic::runtime::RelationalRuntime;
use crate::snapshots::data::SnapshotHandle;

use super::FrozenCommitStrategyRegistry;

#[derive(Debug, Clone, PartialEq, Eq)]
pub(crate) enum CommitStrategyExecutionRegistryError {
    DuplicateBinding {
        strategy_id: CommitStrategyId,
    },
    MissingDescriptorRegistration {
        strategy_id: CommitStrategyId,
        descriptor_digest: CommitStrategyDescriptorDigest,
    },
    DescriptorDigestMismatch {
        strategy_id: CommitStrategyId,
        descriptor_digest: CommitStrategyDescriptorDigest,
        registered_digest: CommitStrategyDescriptorDigest,
    },
}

#[derive(Clone)]
struct RegisteredCommitStrategyExecutor {
    descriptor_digest: CommitStrategyDescriptorDigest,
    executor: Arc<dyn CommitStrategyExecutor>,
}

#[derive(Clone, Default)]
pub(crate) struct FrozenCommitStrategyExecutorRegistry {
    executors_by_id: Arc<BTreeMap<CommitStrategyId, RegisteredCommitStrategyExecutor>>,
}

impl std::fmt::Debug for FrozenCommitStrategyExecutorRegistry {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.debug_struct("FrozenCommitStrategyExecutorRegistry")
            .field("bound_executor_count", &self.executors_by_id.len())
            .finish()
    }
}

impl FrozenCommitStrategyExecutorRegistry {
    pub(crate) fn from_registrations(
        bindings: Vec<CommitStrategyExecutionRegistration>,
        descriptor_registry: &FrozenCommitStrategyRegistry,
    ) -> Result<Self, CommitStrategyExecutionRegistryError> {
        let mut executors_by_id = BTreeMap::new();
        for binding in bindings {
            if executors_by_id.contains_key(&binding.strategy_id()) {
                return Err(CommitStrategyExecutionRegistryError::DuplicateBinding {
                    strategy_id: binding.strategy_id(),
                });
            }
            let Some(registration) = descriptor_registry
                .iter()
                .find(|registration| registration.descriptor().id() == binding.strategy_id())
            else {
                return Err(
                    CommitStrategyExecutionRegistryError::MissingDescriptorRegistration {
                        strategy_id: binding.strategy_id(),
                        descriptor_digest: binding.descriptor_digest(),
                    },
                );
            };
            let registered_digest = registration.descriptor().digest();
            if registered_digest != binding.descriptor_digest() {
                return Err(
                    CommitStrategyExecutionRegistryError::DescriptorDigestMismatch {
                        strategy_id: binding.strategy_id(),
                        descriptor_digest: binding.descriptor_digest(),
                        registered_digest,
                    },
                );
            }
            executors_by_id.insert(
                binding.strategy_id(),
                RegisteredCommitStrategyExecutor {
                    descriptor_digest: binding.descriptor_digest(),
                    executor: binding.executor(),
                },
            );
        }
        Ok(Self {
            executors_by_id: Arc::new(executors_by_id),
        })
    }

    fn get(&self, strategy_id: CommitStrategyId) -> Option<&RegisteredCommitStrategyExecutor> {
        self.executors_by_id.get(&strategy_id)
    }
}

pub(crate) struct BoundStrategyExecution<'runtime> {
    request: &'runtime CanonicalStrategyCommitRequest,
    executor: Arc<dyn CommitStrategyExecutor>,
    observation: StrategyObservationContext<'runtime>,
}

#[derive(Debug, Clone, PartialEq, Eq)]
pub enum StrategyExecutionError {
    UnknownStrategyId {
        strategy_id: CommitStrategyId,
    },
    UnboundStrategyExecutor {
        strategy_id: CommitStrategyId,
    },
    DescriptorDigestMismatch {
        strategy_id: CommitStrategyId,
        request_digest: CommitStrategyDescriptorDigest,
        bound_digest: CommitStrategyDescriptorDigest,
    },
    UnsupportedReadContract {
        strategy_id: CommitStrategyId,
        detail: Arc<str>,
    },
    UnknownSnapshot {
        snapshot_id: crate::snapshots::data::SnapshotId,
    },
    ExecutorFailed {
        strategy_id: CommitStrategyId,
        failure: StrategyExecutorFailure,
    },
    ExecutorPanicked {
        strategy_id: CommitStrategyId,
    },
}

pub(crate) fn bind_execution<'runtime>(
    runtime: &'runtime RelationalRuntime,
    request: &'runtime CanonicalStrategyCommitRequest,
    snapshot: &'runtime SnapshotHandle,
) -> Result<BoundStrategyExecution<'runtime>, StrategyExecutionError> {
    let descriptor = runtime
        .commit_strategy_registry()
        .get_by_id(request.strategy_id())
        .map(|registration| registration.descriptor())
        .ok_or(StrategyExecutionError::UnknownStrategyId {
            strategy_id: request.strategy_id(),
        })?;
    let binding = runtime
        .commit_strategy_executor_registry()
        .get(request.strategy_id())
        .ok_or(StrategyExecutionError::UnboundStrategyExecutor {
            strategy_id: request.strategy_id(),
        })?;
    if binding.descriptor_digest != request.descriptor_digest() {
        return Err(StrategyExecutionError::DescriptorDigestMismatch {
            strategy_id: request.strategy_id(),
            request_digest: request.descriptor_digest(),
            bound_digest: binding.descriptor_digest,
        });
    }
    validate_supported_read_contract(request.strategy_id(), descriptor.read_contract())?;
    if snapshot.runtime_instance_id != runtime.runtime_instance_id() {
        return Err(StrategyExecutionError::UnknownSnapshot {
            snapshot_id: snapshot.snapshot_id,
        });
    }
    let visibility = runtime.read_truth().project_snapshot(snapshot).ok_or(
        StrategyExecutionError::UnknownSnapshot {
            snapshot_id: snapshot.snapshot_id,
        },
    )?;
    Ok(BoundStrategyExecution {
        request,
        executor: Arc::clone(&binding.executor),
        observation: StrategyObservationContext::new(
            runtime,
            snapshot,
            descriptor.read_contract(),
            runtime.schema_registry(),
            runtime.aspect_plan_catalog(),
            visibility,
        ),
    })
}

fn validate_supported_read_contract(
    strategy_id: CommitStrategyId,
    read_contract: &crate::commit_strategies::data::StrategyReadContract,
) -> Result<(), StrategyExecutionError> {
    use crate::commit_strategies::data::{
        StrategyPacketContract, StrategyReadCostClass, StrategyReadLocalityClass,
        StrategyReadScopeClass, StrategyTraversalBasis,
    };

    match read_contract.scope_class {
        StrategyReadScopeClass::ExplicitTargetsOnly
        | StrategyReadScopeClass::PartitionBoundedScan => {}
        StrategyReadScopeClass::KindBoundedScan => {
            return Err(StrategyExecutionError::UnsupportedReadContract {
                strategy_id,
                detail: Arc::from(
                    "KindBoundedScan is not execution-admissible until bounded cross-partition accounting is implemented",
                ),
            });
        }
        StrategyReadScopeClass::BoundedNeighborhood => {
            return Err(StrategyExecutionError::UnsupportedReadContract {
                strategy_id,
                detail: Arc::from(
                    "BoundedNeighborhood is not execution-admissible until bounded traversal accounting is implemented",
                ),
            });
        }
    }

    match read_contract.locality_class {
        StrategyReadLocalityClass::SinglePartition
        | StrategyReadLocalityClass::PartitionBounded => {}
        StrategyReadLocalityClass::CrossPartitionBounded => {
            return Err(StrategyExecutionError::UnsupportedReadContract {
                strategy_id,
                detail: Arc::from(
                    "CrossPartitionBounded is not execution-admissible until explicit partition-bound enforcement exists",
                ),
            });
        }
    }

    if !matches!(
        read_contract.traversal_basis,
        StrategyTraversalBasis::NoTraversal
    ) {
        return Err(StrategyExecutionError::UnsupportedReadContract {
            strategy_id,
            detail: Arc::from(
                "Traversal-enabled strategy execution is not admissible until traversal accounting is implemented",
            ),
        });
    }

    if !matches!(
        read_contract.packet_contract,
        StrategyPacketContract::ProjectionOnly
    ) {
        return Err(StrategyExecutionError::UnsupportedReadContract {
            strategy_id,
            detail: Arc::from(
                "PlannedPacketOnly is not execution-admissible until packet-planned strategy reads are implemented",
            ),
        });
    }

    if !matches!(
        read_contract.cost_class,
        StrategyReadCostClass::ORequestedSurface
    ) {
        return Err(StrategyExecutionError::UnsupportedReadContract {
            strategy_id,
            detail: Arc::from(
                "Only ORequestedSurface strategy execution is admissible until cost enforcement is implemented",
            ),
        });
    }

    Ok(())
}

pub(crate) fn execute_bound_strategy(
    bound: BoundStrategyExecution<'_>,
) -> Result<StrategyExecutionDraft, StrategyExecutionError> {
    let strategy_id = bound.request.strategy_id();
    let result = catch_unwind(AssertUnwindSafe(|| {
        bound.executor.execute(bound.request, &bound.observation)
    }));
    match result {
        Ok(Ok(result)) => Ok(StrategyExecutionDraft::from_measured_result(
            bound.request,
            result,
            bound.observation.measured_summary(),
        )),
        Ok(Err(failure)) => Err(StrategyExecutionError::ExecutorFailed {
            strategy_id,
            failure,
        }),
        Err(_) => Err(StrategyExecutionError::ExecutorPanicked { strategy_id }),
    }
}

#[cfg(test)]
mod tests {
    use std::sync::OnceLock;

    use super::{
        bind_execution, execute_bound_strategy, CommitStrategyExecutionRegistryError,
        FrozenCommitStrategyExecutorRegistry, StrategyExecutionError,
    };
    use crate::commit_strategies::data::{
        CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact,
        CanonicalStrategyInputDigest, CanonicalStrategyOutputArtifact, CommitStrategyDescriptor,
        CommitStrategyExecutionRegistration, CommitStrategyExecutor, CommitStrategyFamilyName,
        CommitStrategyId, CommitStrategyRegistration, CommitStrategySemanticName,
        CommitStrategyVersion, PersistentArtifactName, StrategyExecutionResult,
        StrategyInputSchemaName, StrategyInputSchemaVersion, StrategyIntentName,
        StrategyMutationProgram, StrategyOutputSchemaName, StrategyPacketContract,
        StrategyReadContract, StrategyReadCostClass, StrategyReadLocalityClass,
        StrategyReadScopeClass, StrategyTraversalBasis,
    };
    use crate::commit_strategies::FrozenCommitStrategyRegistry;
    use crate::identity::data::{EntityId, KindId, PartitionId};
    use crate::logic::builder::RelationalRuntimeBuilder;
    use crate::snapshots::data::SnapshotHandle;

    fn empty_result() -> StrategyExecutionResult {
        StrategyExecutionResult::new(
            CanonicalStrategyOutputArtifact::new(
                StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                b"status=ok".to_vec(),
                PersistentArtifactName::new("strategy.intent.reconcile"),
            ),
            StrategyMutationProgram::new(Vec::<crate::transactions::data::WorkerIntentBatch>::new()),
        )
    }

    #[derive(Clone, Copy)]
    struct EchoExecutor;

    impl CommitStrategyExecutor for EchoExecutor {
        fn execute(
            &self,
            _request: &CanonicalStrategyCommitRequest,
            observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, crate::commit_strategies::data::StrategyExecutorFailure>
        {
            let _ = observation
                .visibility()
                .entity_records_in(PartitionId(1), KindId(100))?;
            Ok(empty_result())
        }
    }

    #[derive(Clone, Copy)]
    struct CrossPartitionExecutor;

    impl CommitStrategyExecutor for CrossPartitionExecutor {
        fn execute(
            &self,
            _request: &CanonicalStrategyCommitRequest,
            observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, crate::commit_strategies::data::StrategyExecutorFailure>
        {
            let _ = observation
                .visibility()
                .entity::<TestEntityProjection>(EntityId::new(PartitionId(1), 0, 0))?;
            let _ = observation
                .visibility()
                .entity::<TestEntityProjection>(EntityId::new(PartitionId(2), 0, 0))?;
            Ok(empty_result())
        }
    }

    #[derive(Clone, Copy)]
    struct UnsupportedContractExecutor;

    impl CommitStrategyExecutor for UnsupportedContractExecutor {
        fn execute(
            &self,
            _request: &CanonicalStrategyCommitRequest,
            _observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, crate::commit_strategies::data::StrategyExecutorFailure>
        {
            Ok(empty_result())
        }
    }

    #[derive(Clone, Copy)]
    struct PanicExecutor;

    impl CommitStrategyExecutor for PanicExecutor {
        fn execute(
            &self,
            _request: &CanonicalStrategyCommitRequest,
            _observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
        ) -> Result<StrategyExecutionResult, crate::commit_strategies::data::StrategyExecutorFailure>
        {
            panic!("panic containment probe")
        }
    }

    struct TestEntityProjection;

    impl crate::visibility::materialization::read_records::EntityRecordProjection
        for TestEntityProjection
    {
        const KIND: KindId = KindId(100);

        fn from_record(
            _record: crate::visibility::materialization::read_records::EntityProjectionRecord<'_>,
        ) -> Option<Self> {
            Some(Self)
        }
    }

    struct MissingAspectProjection;

    impl crate::visibility::materialization::read_records::EntityRecordProjection
        for MissingAspectProjection
    {
        const KIND: KindId = KindId(100);

        fn projection_scope(
        ) -> crate::visibility::materialization::read_records::ProjectionAspectScope {
            static REQUIRED: OnceLock<Box<[forge_foundational::facade::AspectKey]>> =
                OnceLock::new();
            crate::visibility::materialization::read_records::ProjectionAspectScope::whole_aspects(
                REQUIRED
                    .get_or_init(|| {
                        vec![forge_foundational::facade::AspectKey::new("missing.aspect").unwrap()]
                            .into_boxed_slice()
                    })
                    .iter()
                    .cloned(),
            )
        }

        fn from_record(
            _record: crate::visibility::materialization::read_records::EntityProjectionRecord<'_>,
        ) -> Option<Self> {
            Some(Self)
        }
    }

    fn descriptor() -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            CommitStrategyId(41),
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
            StrategyInputSchemaName::new("intent.reconcile.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::PartitionBoundedScan,
                locality_class: StrategyReadLocalityClass::SinglePartition,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::ORequestedSurface,
            },
            PersistentArtifactName::new("strategy.intent.reconcile"),
        )
    }

    fn canonical_request() -> CanonicalStrategyCommitRequest {
        let descriptor = descriptor();
        CanonicalStrategyCommitRequest::new(
            descriptor.id(),
            descriptor.digest(),
            CanonicalStrategyInputArtifact::new(
                descriptor.input_schema_name().clone(),
                descriptor.input_schema_version(),
                b"a=1".to_vec().into(),
                CanonicalStrategyInputDigest([7; 32]),
                descriptor.artifact_name().clone(),
            ),
            crate::commit_strategies::data::StrategyCallerProvenance {
                request_origin: crate::commit_strategies::data::StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            },
        )
    }

    fn unsupported_descriptor() -> CommitStrategyDescriptor {
        CommitStrategyDescriptor::new(
            CommitStrategyId(42),
            CommitStrategySemanticName::new("strategy.intent.unsupported"),
            CommitStrategyFamilyName::new("strategy.intent"),
            CommitStrategyVersion::new(1, 0),
            StrategyIntentName::new("reconcile.desired.state"),
            StrategyInputSchemaName::new("intent.reconcile.input.v1"),
            StrategyInputSchemaVersion(1),
            StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
            StrategyReadContract {
                scope_class: StrategyReadScopeClass::KindBoundedScan,
                locality_class: StrategyReadLocalityClass::CrossPartitionBounded,
                traversal_basis: StrategyTraversalBasis::NoTraversal,
                packet_contract: StrategyPacketContract::ProjectionOnly,
                cost_class: StrategyReadCostClass::OPartitionBoundedSurface,
            },
            PersistentArtifactName::new("strategy.intent.unsupported"),
        )
    }

    fn canonical_request_for(
        descriptor: &CommitStrategyDescriptor,
    ) -> CanonicalStrategyCommitRequest {
        CanonicalStrategyCommitRequest::new(
            descriptor.id(),
            descriptor.digest(),
            CanonicalStrategyInputArtifact::new(
                descriptor.input_schema_name().clone(),
                descriptor.input_schema_version(),
                b"a=1".to_vec().into(),
                CanonicalStrategyInputDigest([7; 32]),
                descriptor.artifact_name().clone(),
            ),
            crate::commit_strategies::data::StrategyCallerProvenance {
                request_origin: crate::commit_strategies::data::StrategyRequestOrigin::Test,
                actor_identity: None,
                correlation_id: None,
            },
        )
    }

    #[test]
    fn executor_registry_rejects_binding_without_descriptor_registration() {
        let descriptor = descriptor();
        let registry = FrozenCommitStrategyRegistry::from_registrations(Vec::new()).unwrap();

        let error = FrozenCommitStrategyExecutorRegistry::from_registrations(
            vec![CommitStrategyExecutionRegistration::new(
                &descriptor,
                EchoExecutor,
            )],
            &registry,
        )
        .unwrap_err();

        assert_eq!(
            error,
            CommitStrategyExecutionRegistryError::MissingDescriptorRegistration {
                strategy_id: CommitStrategyId(41),
                descriptor_digest: descriptor.digest(),
            }
        );
    }

    #[test]
    fn bound_execution_executes_against_snapshot_and_contains_panics() {
        let descriptor = descriptor();
        let registration = CommitStrategyRegistration::new(descriptor.clone()).unwrap();
        let executor_binding = CommitStrategyExecutionRegistration::new(&descriptor, EchoExecutor);
        let panic_binding = CommitStrategyExecutionRegistration::new(&descriptor, PanicExecutor);

        let mut runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration.clone())
            .commit_strategy_executor(executor_binding)
            .build();
        let snapshot = runtime.visibility_authority().snapshot();
        let request = canonical_request();

        let bound = bind_execution(&runtime, &request, &snapshot).expect("bind strategy");
        let draft = execute_bound_strategy(bound).expect("execute strategy");

        assert_eq!(
            draft.output().schema_name().as_str(),
            "intent.reconcile.output.v1"
        );
        assert_eq!(draft.summary().projected_partition_reads, 0);
        assert_eq!(draft.summary().entity_record_reads, 0);

        let mut panic_runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .commit_strategy_executor(panic_binding)
            .build();
        let panic_snapshot: SnapshotHandle = panic_runtime.visibility_authority().snapshot();

        let panic_bound =
            bind_execution(&panic_runtime, &request, &panic_snapshot).expect("bind strategy");
        let error = execute_bound_strategy(panic_bound).unwrap_err();
        assert_eq!(
            error,
            StrategyExecutionError::ExecutorPanicked {
                strategy_id: CommitStrategyId(41)
            }
        );
    }

    #[test]
    fn single_partition_locality_rejects_cross_partition_strategy_reads() {
        let descriptor = descriptor();
        let registration = CommitStrategyRegistration::new(descriptor.clone()).unwrap();
        let executor_binding =
            CommitStrategyExecutionRegistration::new(&descriptor, CrossPartitionExecutor);

        let mut runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .commit_strategy_executor(executor_binding)
            .build();
        let snapshot = runtime.visibility_authority().snapshot();
        let request = canonical_request();

        let bound = bind_execution(&runtime, &request, &snapshot).expect("bind strategy");
        let error = execute_bound_strategy(bound).unwrap_err();

        assert!(matches!(
            error,
            StrategyExecutionError::ExecutorFailed {
                strategy_id: CommitStrategyId(41),
                failure,
            } if failure.class
                == crate::commit_strategies::data::StrategyExecutorFailureClass::ReadContractViolation
                && failure.detail.contains("SinglePartition locality")
        ));
    }

    #[test]
    fn foreign_snapshot_handle_is_rejected_for_strategy_execution() {
        let descriptor = descriptor();
        let registration = CommitStrategyRegistration::new(descriptor.clone()).unwrap();
        let executor_binding = CommitStrategyExecutionRegistration::new(&descriptor, EchoExecutor);

        let mut left_runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration.clone())
            .commit_strategy_executor(executor_binding.clone())
            .build();
        let foreign_snapshot = left_runtime.visibility_authority().snapshot();

        let right_runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .commit_strategy_executor(executor_binding)
            .build();
        let request = canonical_request();

        let error = match bind_execution(&right_runtime, &request, &foreign_snapshot) {
            Ok(_) => panic!("foreign snapshot should be rejected"),
            Err(error) => error,
        };
        assert_eq!(
            error,
            StrategyExecutionError::UnknownSnapshot {
                snapshot_id: foreign_snapshot.snapshot_id
            }
        );
    }

    #[test]
    fn unsupported_read_contract_is_rejected_before_execution() {
        let descriptor = unsupported_descriptor();
        let registration = CommitStrategyRegistration::new(descriptor.clone()).unwrap();
        let executor_binding =
            CommitStrategyExecutionRegistration::new(&descriptor, UnsupportedContractExecutor);

        let mut runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .commit_strategy_executor(executor_binding)
            .build();
        let snapshot = runtime.visibility_authority().snapshot();
        let request = canonical_request_for(&descriptor);

        let error = match bind_execution(&runtime, &request, &snapshot) {
            Ok(_) => panic!("unsupported read contract should be rejected"),
            Err(error) => error,
        };
        assert!(matches!(
            error,
            StrategyExecutionError::UnsupportedReadContract {
                strategy_id: CommitStrategyId(42),
                ..
            }
        ));
    }

    #[test]
    fn projection_contract_violation_is_typed_not_panic_bucket() {
        let descriptor = descriptor();
        let registration = CommitStrategyRegistration::new(descriptor.clone()).unwrap();

        #[derive(Clone, Copy)]
        struct ProjectionViolationExecutor;

        impl CommitStrategyExecutor for ProjectionViolationExecutor {
            fn execute(
                &self,
                _request: &CanonicalStrategyCommitRequest,
                observation: &crate::commit_strategies::data::StrategyObservationContext<'_>,
            ) -> Result<
                StrategyExecutionResult,
                crate::commit_strategies::data::StrategyExecutorFailure,
            > {
                let _ = observation
                    .visibility()
                    .entity::<MissingAspectProjection>(EntityId::new(PartitionId(1), 0, 0))?;
                Ok(empty_result())
            }
        }

        let executor_binding =
            CommitStrategyExecutionRegistration::new(&descriptor, ProjectionViolationExecutor);
        let mut runtime = RelationalRuntimeBuilder::new()
            .commit_strategy(registration)
            .commit_strategy_executor(executor_binding)
            .build();
        let snapshot = runtime.visibility_authority().snapshot();
        let request = canonical_request();

        let bound = bind_execution(&runtime, &request, &snapshot).expect("bind strategy");
        let error = execute_bound_strategy(bound).unwrap_err();
        assert!(matches!(
            error,
            StrategyExecutionError::ExecutorFailed {
                strategy_id: CommitStrategyId(41),
                failure,
            } if failure.class
                == crate::commit_strategies::data::StrategyExecutorFailureClass::ProjectionContractViolation
        ));
    }
}
