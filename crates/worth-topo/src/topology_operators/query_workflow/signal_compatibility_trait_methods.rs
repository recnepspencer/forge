macro_rules! signal_compatibility_workflow_methods {
    () => {
        fn orchestrate_topology_operator_signal_compatibility<I>(
            &self,
            input: TopologyOperatorSignalCompatibilityInput<I>,
        ) -> TopologyOperatorSignalCompatibilityOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_signal_compatibility(input)
        }

        fn orchestrate_topology_operator_signal_compatibility_outcome<I>(
            &self,
            input: TopologyOperatorSignalCompatibilityInput<I>,
        ) -> ForgeQueryOrdinaryOutcome<TopologyOperatorSignalCompatibilityArtifact<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_signal_compatibility_outcome(input)
        }

        fn orchestrate_topology_operator_signal_compatibility_checked<I>(
            &self,
            input: TopologyOperatorSignalCompatibilityInput<I>,
        ) -> TopologyOperatorSignalCompatibilityChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_signal_compatibility_checked(input)
        }

        fn orchestrate_topology_operator_signal_compatibility_proof<I>(
            &self,
            input: TopologyOperatorSignalCompatibilityInput<I>,
        ) -> TopologyOperatorSignalCompatibilityProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_signal_compatibility_proof(input)
        }

        fn recover_topology_operator_signal_compatibility_checked<I>(
            &self,
            checked: TopologyOperatorSignalCompatibilityChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_signal_compatibility_checked(checked)
        }

        fn recover_topology_operator_signal_compatibility_proof<I>(
            &self,
            proof: TopologyOperatorSignalCompatibilityProof<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_signal_compatibility_proof(proof)
        }
    };
}
