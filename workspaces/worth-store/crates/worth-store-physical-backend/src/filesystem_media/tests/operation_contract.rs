use super::super::*;

#[test]
fn operation_inventory_is_complete_and_contractual() {
    assert_eq!(MediaOperationRole::ALL.len(), 27);

    for operation in MediaOperationRole::ALL {
        let contract = operation.contract();
        assert_eq!(
            contract.observation(),
            MediaObservationAudience::RuntimeDiagnostics
        );
        assert_eq!(
            contract.fault_control(),
            MediaFaultControlAudience::CertificationOnly
        );
        assert_transfer_contract_is_coherent(operation, contract);
        assert_synchronization_contract_is_coherent(contract);
        assert_optional_capability_contract_is_coherent(operation, contract);
        assert_namespace_entry_authority_is_coherent(operation, contract);
    }
}

fn assert_namespace_entry_authority_is_coherent(
    operation: MediaOperationRole,
    contract: MediaOperationContract,
) {
    if matches!(
        operation,
        MediaOperationRole::ReadMetadata | MediaOperationRole::Delete
    ) {
        assert_eq!(
            contract.handle(),
            MediaHandleRequirement::OwnerIssuedNamespaceEntry
        );
    }
}

fn assert_transfer_contract_is_coherent(
    operation: MediaOperationRole,
    contract: MediaOperationContract,
) {
    match contract.transfer() {
        MediaTransferCardinality::BoundedByteTransfer => {
            if operation == MediaOperationRole::PublishMutationLeaseObservation {
                assert_eq!(
                    contract.partial_effect(),
                    MediaPartialEffect::BytePrefixOrBarrierIndeterminate
                );
                assert_eq!(contract.counter(), MediaCounterClass::OwnershipPublication);
            } else {
                assert_eq!(contract.partial_effect(), MediaPartialEffect::BytePrefix);
                assert!(matches!(
                    contract.counter(),
                    MediaCounterClass::ReadTransfer | MediaCounterClass::WriteTransfer
                ));
            }
        }
        MediaTransferCardinality::SingleObservation => {
            if operation == MediaOperationRole::ReadMetadata {
                assert_eq!(contract.counter(), MediaCounterClass::MetadataObservation);
            } else {
                assert!(matches!(
                    operation,
                    MediaOperationRole::InspectNamespaceEntry
                        | MediaOperationRole::ValidateRootIdentity
                        | MediaOperationRole::ObserveRootProfile
                ));
                assert_eq!(contract.counter(), MediaCounterClass::AdmissionObservation);
            }
        }
        MediaTransferCardinality::DirectorySequence => {
            assert_eq!(operation, MediaOperationRole::ListDirectory);
            assert_eq!(contract.counter(), MediaCounterClass::DirectoryObservation);
        }
        MediaTransferCardinality::None => assert!(!matches!(
            contract.counter(),
            MediaCounterClass::ReadTransfer | MediaCounterClass::WriteTransfer
        )),
    }
}

fn assert_synchronization_contract_is_coherent(contract: MediaOperationContract) {
    match contract.synchronization() {
        MediaSynchronizationMeaning::None => {
            assert_ne!(
                contract.counter(),
                MediaCounterClass::SynchronizationBarrier
            )
        }
        MediaSynchronizationMeaning::FileData
        | MediaSynchronizationMeaning::FileDataAndMetadata
        | MediaSynchronizationMeaning::ParentNamespacePublication => {
            assert!(matches!(
                contract.partial_effect(),
                MediaPartialEffect::BarrierCompletionMayBeIndeterminate
                    | MediaPartialEffect::BytePrefixOrBarrierIndeterminate
            ));
            assert!(matches!(
                contract.counter(),
                MediaCounterClass::SynchronizationBarrier | MediaCounterClass::OwnershipPublication
            ));
            assert_eq!(contract.retry(), MediaRetryRule::InspectAfterPossibleEffect);
        }
    }
}

fn assert_optional_capability_contract_is_coherent(
    operation: MediaOperationRole,
    contract: MediaOperationContract,
) {
    match contract.capability() {
        MediaCapabilityRequirement::QualifiedAllocationMode => {
            assert_eq!(operation, MediaOperationRole::Allocate)
        }
        MediaCapabilityRequirement::QualifiedDataOnlySynchronization => {
            assert_eq!(operation, MediaOperationRole::SynchronizeFileData)
        }
        _ => {}
    }
}
