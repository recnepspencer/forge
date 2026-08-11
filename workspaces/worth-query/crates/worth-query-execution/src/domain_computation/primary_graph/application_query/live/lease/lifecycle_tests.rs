mod tests {
    use std::time::{Duration, Instant};

    use worth_query_admission::facade::authenticated_principal::{
        WorthQueryCancellationSource, WorthQueryRequestScope,
    };
    use worth_relational::facade::history::CommitId;
    use worth_runtime_bridge::facade::{
        BridgeExecutionBasisLifecycleObserver, BridgeExecutionBasisLifecycleSignalStatus,
    };

    use crate::domain_computation::primary_graph::tests::fixture::{
        installed_authorization_world, live_account_parameters, Account, AccountIdentity,
        AccountSummaryParameters, Activity, AuthorizationWorld, IdentityExecutionSchema,
        LiveAccountActivityCause, LiveAccountActivityQuery, LiveAccountActivityResult, Principal,
    };
    use crate::domain_computation::primary_graph::{
        application_attempt::WorthQueryAdmittedApplicationEmissionBatch,
        WorthQueryApplicationLiveCloseOutcome, WorthQueryApplicationLiveControls,
        WorthQueryApplicationLiveLease, WorthQueryApplicationLiveOutcome,
        WorthQueryAuthenticatedPrincipal, WorthQueryPrincipalResolutionMode,
    };

    type TestLiveLease<'runtime, 'principal> = WorthQueryApplicationLiveLease<
        'runtime,
        'principal,
        IdentityExecutionSchema,
        LiveAccountActivityQuery,
        AccountSummaryParameters,
        LiveAccountActivityResult,
        Principal,
        u64,
        Account,
        Activity,
        LiveAccountActivityCause,
    >;

    struct LiveContext {
        world: AuthorizationWorld,
        request: WorthQueryRequestScope,
        principal: WorthQueryAuthenticatedPrincipal<IdentityExecutionSchema, Principal, u64>,
    }

    impl LiveContext {
        fn new(request: WorthQueryRequestScope) -> Self {
            let world = installed_authorization_world(true);
            let external = world.authenticate("alice", Duration::from_secs(60), &request);
            let principal = world
                .application
                .resolve_authenticated_principal(
                    &world.binding,
                    external,
                    &request,
                    WorthQueryPrincipalResolutionMode::Ordinary,
                )
                .unwrap();
            Self {
                world,
                request,
                principal,
            }
        }

        fn with_already_expired_deadline_after_open(&self, lease: &mut TestLiveLease<'_, '_>) {
            let live_cancellation = WorthQueryCancellationSource::new();
            // Instant deadline comparison is `now >= deadline`, so binding the
            // current instant makes the next poll sample DeadlineExceeded
            // without sleeping or racing open.
            lease.replace_request(WorthQueryRequestScope::new(
                Instant::now(),
                live_cancellation.token(),
            ));
        }

        fn open(&self) -> TestLiveLease<'_, '_> {
            let query = self
                .world
                .application
                .installed_schema()
                .application_query(LiveAccountActivityQuery::reference())
                .unwrap();
            let account = self
                .world
                .application
                .resolve_entity(
                    AccountIdentity::reference(),
                    "account-1".to_owned(),
                    &self.request,
                    WorthQueryPrincipalResolutionMode::Ordinary,
                )
                .unwrap();
            self.world
                .application
                .open_application_query_live::<
                    LiveAccountActivityQuery,
                    AccountSummaryParameters,
                    LiveAccountActivityResult,
                    Principal,
                    u64,
                    Account,
                    Activity,
                    LiveAccountActivityCause,
                >(
                    query,
                    &self.principal,
                    account,
                    live_account_parameters("account-1"),
                    WorthQueryApplicationLiveControls::bounded(
                        self.request.clone(),
                        4,
                        16,
                        2_048,
                    )
                    .unwrap(),
                )
                .unwrap()
        }

        fn basis_is_live(
            &self,
            basis: &worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
        ) -> bool {
            let graph = self
                .world
                .application
                .runtime
                .primary_graph()
                .expect("test world publishes a primary graph");
            graph
                .integration_handle()
                .with_runtime_mut(|runtime| runtime.snapshots().execution_basis_is_live(basis))
        }

        fn close_source(&self) {
            self.world
                .application
                .primary_provider
                .live_delivery
                .close();
        }

        fn overflow_source(&self) {
            for ordinal in 1..=65 {
                self.world
                    .application
                    .primary_provider
                    .live_delivery
                    .publish(
                        CommitId(ordinal),
                        WorthQueryAdmittedApplicationEmissionBatch::admit(Vec::new(), 0).unwrap(),
                    )
                    .unwrap();
            }
        }
    }

    #[test]
    fn explicit_close_releases_relational_bridge_signal_and_queue_resources() {
        let context = LiveContext::new(
            crate::domain_computation::primary_graph::tests::fixture::live_scope(),
        );
        let lease = context.open();
        let (bridge, relational) = observers(&lease);
        let provider_session_baseline = context.world.application.provider_session_resource_count();
        assert_live(&context, &bridge, &relational);

        let WorthQueryApplicationLiveCloseOutcome::Completed(completion) = lease.close() else {
            panic!("live close must complete its opening graph-read session");
        };
        assert_eq!(completion.release().released_reservation_count(), 1);
        assert!(completion.basis_release().released());
        assert_eq!(
            context.world.application.provider_session_resource_count(),
            provider_session_baseline
        );
        assert_terminal(
            &context,
            &bridge,
            &relational,
            BridgeExecutionBasisLifecycleSignalStatus::Fulfilled,
        );
    }

    #[test]
    fn cancellation_and_deadline_terminalize_all_live_resources() {
        let cancellation = WorthQueryCancellationSource::new();
        let request = WorthQueryRequestScope::new(
            Instant::now() + Duration::from_secs(60),
            cancellation.token(),
        );
        let context = LiveContext::new(request);
        let mut lease = context.open();
        let (bridge, relational) = observers(&lease);
        cancellation.cancel();
        assert!(matches!(
            lease.poll(),
            WorthQueryApplicationLiveOutcome::Cancelled
        ));
        assert_terminal(
            &context,
            &bridge,
            &relational,
            BridgeExecutionBasisLifecycleSignalStatus::Cancelled,
        );

        let deadline_context = LiveContext::new(
            crate::domain_computation::primary_graph::tests::fixture::live_scope(),
        );
        let mut deadline_lease = deadline_context.open();
        let (deadline_bridge, deadline_relational) = observers(&deadline_lease);
        // Live-lease deadlines still sample wall-clock Instant, not the Gate
        // 8.3 injectable runtime clock. Open under a non-expiring scope, then
        // bind an already-expired deadline only to the poll phase under test.
        deadline_context.with_already_expired_deadline_after_open(&mut deadline_lease);
        assert!(matches!(
            deadline_lease.poll(),
            WorthQueryApplicationLiveOutcome::DeadlineExceeded
        ));
        assert_terminal(
            &deadline_context,
            &deadline_bridge,
            &deadline_relational,
            BridgeExecutionBasisLifecycleSignalStatus::Cancelled,
        );
    }

    #[test]
    fn source_close_overflow_and_abandoned_drop_release_all_live_resources() {
        let closed_context = LiveContext::new(
            crate::domain_computation::primary_graph::tests::fixture::live_scope(),
        );
        let mut closed = closed_context.open();
        let (closed_bridge, closed_relational) = observers(&closed);
        closed_context.close_source();
        assert!(matches!(
            closed.poll(),
            WorthQueryApplicationLiveOutcome::Closed
        ));
        assert_terminal(
            &closed_context,
            &closed_bridge,
            &closed_relational,
            BridgeExecutionBasisLifecycleSignalStatus::Fulfilled,
        );

        let overflow_context = LiveContext::new(
            crate::domain_computation::primary_graph::tests::fixture::live_scope(),
        );
        let mut overflow = overflow_context.open();
        let (overflow_bridge, overflow_relational) = observers(&overflow);
        overflow_context.overflow_source();
        let WorthQueryApplicationLiveOutcome::Overflow(missed) = overflow.poll() else {
            panic!("retention loss must terminalize as overflow");
        };
        assert_eq!(missed.missed_commit_batches(), 1);
        assert_terminal(
            &overflow_context,
            &overflow_bridge,
            &overflow_relational,
            BridgeExecutionBasisLifecycleSignalStatus::Cancelled,
        );

        let abandoned_context = LiveContext::new(
            crate::domain_computation::primary_graph::tests::fixture::live_scope(),
        );
        let abandoned = abandoned_context.open();
        let (abandoned_bridge, abandoned_relational) = observers(&abandoned);
        drop(abandoned);
        assert_terminal(
            &abandoned_context,
            &abandoned_bridge,
            &abandoned_relational,
            BridgeExecutionBasisLifecycleSignalStatus::Cancelled,
        );
    }

    fn observers(
        lease: &TestLiveLease<'_, '_>,
    ) -> (
        BridgeExecutionBasisLifecycleObserver,
        worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
    ) {
        let basis = lease.basis.as_ref().expect("open lease retains its basis");
        (
            basis.bridge.lifecycle_observer(),
            basis.relational.identity().clone(),
        )
    }

    fn assert_live(
        context: &LiveContext,
        bridge: &BridgeExecutionBasisLifecycleObserver,
        relational: &worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
    ) {
        assert!(context.basis_is_live(relational));
        let observation = bridge.observe().unwrap();
        assert!(observation.reservation_active());
        assert_eq!(
            observation.signal_status(),
            Some(BridgeExecutionBasisLifecycleSignalStatus::Active)
        );
        assert_eq!(
            observation
                .managed_queue_pressure()
                .expect("live lease owns one bounded queue")
                .queue_depth(),
            0
        );
    }

    fn assert_terminal(
        context: &LiveContext,
        bridge: &BridgeExecutionBasisLifecycleObserver,
        relational: &worth_relational::facade::runtime::RelationalExecutionBasisIdentity,
        status: BridgeExecutionBasisLifecycleSignalStatus,
    ) {
        assert!(!context.basis_is_live(relational));
        let observation = bridge.observe().unwrap();
        assert!(!observation.reservation_active());
        assert_eq!(observation.signal_status(), Some(status));
        assert_eq!(
            observation
                .managed_queue_pressure()
                .expect("terminal request retains an empty queue observation")
                .queue_depth(),
            0
        );
    }
}
