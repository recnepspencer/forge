use std::sync::Arc;

use sha2::{Digest, Sha256};

use crate::commit_strategies::data::{
    CanonicalStrategyCommitRequest, CanonicalStrategyInputArtifact, CanonicalStrategyInputDigest,
    RawStrategyCommitRequest, StrategyCommitRequestError, StrategyRequestCanonicalization,
};

use super::FrozenCommitStrategyRegistry;

pub(crate) fn canonicalize_request(
    registry: &FrozenCommitStrategyRegistry,
    request: &RawStrategyCommitRequest,
) -> Result<CanonicalStrategyCommitRequest, StrategyCommitRequestError> {
    let registration = registry
        .get_by_name(request.strategy_name())
        .ok_or_else(|| StrategyCommitRequestError::UnknownStrategyName {
            strategy_name: request.strategy_name().clone(),
        })?;
    let descriptor = registration.descriptor();
    let canonical_bytes =
        canonicalize_request_bytes(request.input_bytes(), descriptor.request_canonicalization())
            .map_err(|detail| StrategyCommitRequestError::InvalidCanonicalInput {
                strategy_name: request.strategy_name().clone(),
                detail,
            })?;
    let digest = digest_bytes(&canonical_bytes);
    let input_artifact = CanonicalStrategyInputArtifact::new(
        descriptor.input_schema_name().clone(),
        descriptor.input_schema_version(),
        descriptor.request_canonicalization(),
        canonical_bytes,
        digest,
        descriptor.artifact_name().clone(),
    );
    Ok(CanonicalStrategyCommitRequest::new(
        descriptor.id(),
        descriptor.digest(),
        input_artifact,
        request.caller_provenance().clone(),
    ))
}

fn canonicalize_request_bytes(
    bytes: &[u8],
    canonicalization: StrategyRequestCanonicalization,
) -> Result<Arc<[u8]>, Arc<str>> {
    match canonicalization {
        StrategyRequestCanonicalization::NativeCanonicalBytesV1 => Ok(bytes.to_vec().into()),
    }
}

fn digest_bytes(bytes: &[u8]) -> CanonicalStrategyInputDigest {
    let digest = Sha256::digest(bytes);
    CanonicalStrategyInputDigest(digest.into())
}

#[cfg(test)]
mod tests {
    use super::canonicalize_request;
    use crate::commit_strategies::data::{
        CanonicalStrategyInputDigest, CommitStrategyDescriptor, CommitStrategyFamilyName,
        CommitStrategyId, CommitStrategyRegistration, CommitStrategySemanticName,
        CommitStrategyVersion, PersistentArtifactName, RawStrategyCommitRequest,
        StrategyCallerProvenance, StrategyInputSchemaName, StrategyInputSchemaVersion,
        StrategyIntentName, StrategyOutputSchemaName, StrategyPacketContract, StrategyReadContract,
        StrategyReadCostClass, StrategyReadLocalityClass, StrategyReadScopeClass,
        StrategyRequestCanonicalization, StrategyRequestOrigin, StrategyTraversalBasis,
    };
    use crate::commit_strategies::FrozenCommitStrategyRegistry;
    use sha2::Digest;

    fn registry() -> FrozenCommitStrategyRegistry {
        FrozenCommitStrategyRegistry::from_registrations(vec![CommitStrategyRegistration::new(
            CommitStrategyDescriptor::new(
                CommitStrategyId(7),
                CommitStrategySemanticName::new("strategy.intent.reconcile"),
                CommitStrategyFamilyName::new("strategy.intent"),
                CommitStrategyVersion::new(1, 0),
                StrategyIntentName::new("reconcile.desired.state"),
                StrategyInputSchemaName::new("intent.reconcile.input.v1"),
                StrategyInputSchemaVersion(1),
                StrategyOutputSchemaName::new("intent.reconcile.output.v1"),
                StrategyRequestCanonicalization::NativeCanonicalBytesV1,
                StrategyReadContract {
                    scope_class: StrategyReadScopeClass::ExplicitTargetsOnly,
                    locality_class: StrategyReadLocalityClass::SinglePartition,
                    traversal_basis: StrategyTraversalBasis::NoTraversal,
                    packet_contract: StrategyPacketContract::ProjectionOnly,
                    cost_class: StrategyReadCostClass::ORequestedSurface,
                },
                PersistentArtifactName::new("strategy.intent.reconcile"),
            ),
        )
        .expect("valid registration")])
        .expect("valid registry")
    }

    #[test]
    fn canonical_request_binds_registered_strategy_and_stabilizes_json_order() {
        let registry = registry();
        let left = RawStrategyCommitRequest::from_canonical_bytes(
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            b"native-left".to_vec(),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Api,
                actor_identity: Some("actor-1".to_string()),
                correlation_id: Some("corr-1".to_string()),
            },
        );
        let right = RawStrategyCommitRequest::from_canonical_bytes(
            CommitStrategySemanticName::new("strategy.intent.reconcile"),
            b"native-left".to_vec(),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Api,
                actor_identity: Some("actor-1".to_string()),
                correlation_id: Some("corr-1".to_string()),
            },
        );

        let left = canonicalize_request(&registry, &left).expect("left canonical request");
        let right = canonicalize_request(&registry, &right).expect("right canonical request");

        assert_eq!(left.strategy_id(), CommitStrategyId(7));
        assert_eq!(left.canonical_input().canonical_bytes(), b"native-left");
        assert_eq!(
            left.canonical_input().digest(),
            right.canonical_input().digest()
        );
        assert_eq!(
            left.canonical_input().schema_version(),
            StrategyInputSchemaVersion(1)
        );
        assert_eq!(
            left.canonical_input().canonicalization(),
            StrategyRequestCanonicalization::NativeCanonicalBytesV1
        );
        let digest = CanonicalStrategyInputDigest(sha2::Sha256::digest(b"native-left").into());
        assert_eq!(left.canonical_input().digest(), digest);
    }

    #[test]
    fn canonical_request_rejects_unknown_strategy_name() {
        let registry = registry();
        let request = RawStrategyCommitRequest::from_canonical_bytes(
            CommitStrategySemanticName::new("strategy.unknown"),
            b"unknown".to_vec(),
            StrategyCallerProvenance {
                request_origin: StrategyRequestOrigin::Api,
                actor_identity: None,
                correlation_id: None,
            },
        );

        let error = canonicalize_request(&registry, &request).unwrap_err();
        assert_eq!(
            error,
            crate::commit_strategies::data::StrategyCommitRequestError::UnknownStrategyName {
                strategy_name: CommitStrategySemanticName::new("strategy.unknown"),
            }
        );
    }
}
