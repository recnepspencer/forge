macro_rules! continuation_workflow_methods {
    () => {
        fn prepare_topology_operator_continuation<I>(
            &self,
            request: TopologyOperatorContinuationTarget<I>,
        ) -> TopologyOperatorPreparedContinuationOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.prepare_continuation_from_target(request)
        }

        fn prepare_topology_operator_continuation_outcome<I>(
            &self,
            request: TopologyOperatorContinuationTarget<I>,
        ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorPreparedContinuation<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.prepare_continuation_from_target_outcome(request)
        }

        fn prepare_topology_operator_continuation_checked<I>(
            &self,
            request: TopologyOperatorContinuationTarget<I>,
        ) -> TopologyOperatorPreparedContinuationChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.prepare_continuation_from_target_checked(request)
        }

        fn prepare_topology_operator_continuation_proof<I>(
            &self,
            request: TopologyOperatorContinuationTarget<I>,
        ) -> TopologyOperatorPreparedContinuationProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.prepare_continuation_from_target_proof(request)
        }

        fn execute_topology_operator_prepared_continuation<I>(
            &self,
            prepared: TopologyOperatorPreparedContinuation<I>,
        ) -> TopologyOperatorContinuationExecutionOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.execute_prepared_continuation(prepared)
        }

        fn execute_topology_operator_prepared_continuation_outcome<I>(
            &self,
            prepared: TopologyOperatorPreparedContinuation<I>,
        ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorContinuationExecution<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.execute_prepared_continuation_outcome(prepared)
        }

        fn execute_topology_operator_prepared_continuation_checked<I>(
            &self,
            prepared: TopologyOperatorPreparedContinuation<I>,
        ) -> TopologyOperatorContinuationExecutionChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.execute_prepared_continuation_checked(prepared)
        }

        fn execute_topology_operator_prepared_continuation_proof<I>(
            &self,
            prepared: TopologyOperatorPreparedContinuation<I>,
        ) -> TopologyOperatorContinuationExecutionProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.execute_prepared_continuation_proof(prepared)
        }

        fn recover_topology_operator_prepared_continuation_checked<I>(
            &self,
            checked: TopologyOperatorPreparedContinuationChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_prepared_continuation_checked(checked)
        }

        fn recover_topology_operator_prepared_continuation_proof<I>(
            &self,
            proof: TopologyOperatorPreparedContinuationProof<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_prepared_continuation_proof(proof)
        }

        fn recover_topology_operator_continuation_execution_checked<I>(
            &self,
            checked: TopologyOperatorContinuationExecutionChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_continuation_execution_checked(checked)
        }

        fn recover_topology_operator_continuation_execution_proof<I>(
            &self,
            proof: TopologyOperatorContinuationExecutionProof<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_continuation_execution_proof(proof)
        }

        fn recover_topology_operator_outcome<T>(
            &self,
            outcome: &ForgeQueryOrdinaryOutcome<T>,
        ) -> Option<ForgeQueryRecoveryBrief> {
            self.recover_from_outcome(outcome)
        }
    };
}
