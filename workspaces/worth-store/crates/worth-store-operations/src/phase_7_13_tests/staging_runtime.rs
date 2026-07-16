pub(crate) struct CurrentStagingAuthorizationPort;

impl crate::StagingAuthorizationContinuationPort for CurrentStagingAuthorizationPort {
    fn observe_revocation(
        &self,
        _request: crate::StagingAuthorizationContinuationRequest,
    ) -> Result<crate::AuthorizationRevocationObservation, crate::AuthorizationProviderFailure>
    {
        Ok(crate::AuthorizationRevocationObservation::NotRevoked { observed_at: 40 })
    }
}

impl worth_store_recovery_physics::StagedWalApplicationPort for CurrentStagingAuthorizationPort {
    fn apply_staged_wal(
        &self,
        request: worth_store_recovery_physics::StagedWalApplicationRequest<'_>,
    ) -> Result<
        worth_store_recovery_physics::StagedWalApplicationProviderReceipt,
        worth_store_recovery_physics::StagedWalApplicationDenial,
    > {
        apply_fixture_wal(request)
    }
}

pub(crate) fn apply_fixture_wal(
    request: worth_store_recovery_physics::StagedWalApplicationRequest<'_>,
) -> Result<
    worth_store_recovery_physics::StagedWalApplicationProviderReceipt,
    worth_store_recovery_physics::StagedWalApplicationDenial,
> {
    let source = request.replay_source();
    let staging = request.staging();
    Ok(
        worth_store_recovery_physics::StagedWalApplicationProviderReceipt::observed(
            request.application_identity(),
            staging.staging_plan_fingerprint(),
            source.identity(),
            source.interval(),
            source.frame_count(),
            request.target_frontier_identity(),
            true,
        ),
    )
}
