macro_rules! grouped_and_contribution_workflow_methods {
    () => {
        fn declare_topology_grouped_operator<I>(
            &self,
            input: TopologyOperatorGroupedInput<I>,
        ) -> Result<TopologyOperatorGroupedDeclaration<I>, TopologyOperatorGroupedDeclarationStop>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone,
        {
            self.declare_grouped(input)
        }

        fn orchestrate_topology_grouped_operator_outcome<I>(
            &self,
            declaration: TopologyOperatorGroupedDeclaration<I>,
        ) -> TopologyOperatorGroupedOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone,
        {
            self.orchestrate_grouped_outcome(declaration)
        }

        fn topology_grouped_operator_support<I>(
            &self,
            declaration: &TopologyOperatorGroupedDeclaration<I>,
        ) -> ForgeQueryGroupedSupportReport
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.grouped_support_report(declaration)
        }

        fn grouped_topology_operator_contributions_checked<I>(
            &self,
            input: TopologyOperatorGroupedContributionInput<I>,
        ) -> Result<
            TopologyOperatorGroupedContributionComposition<I>,
            TopologyOperatorGroupedContributionStop<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain> + Clone,
        {
            self.grouped_contributions_checked(input)
        }

        fn orchestrate_topology_operator_with_contributions<I>(
            &self,
            input: TopologyOperatorContributionInput<I>,
        ) -> Result<
            TopologyOperatorContributionArtifact<I>,
            TopologyOperatorContributionCheckedOutcome<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_with_contributions(input)
        }

        fn orchestrate_topology_operator_with_contributions_outcome<I>(
            &self,
            input: TopologyOperatorContributionInput<I>,
        ) -> TopologyOperatorContributionOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_with_contributions_outcome(input)
        }

        fn orchestrate_topology_operator_with_contributions_checked<I>(
            &self,
            input: TopologyOperatorContributionInput<I>,
        ) -> TopologyOperatorContributionChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_with_contributions_checked(input)
        }

        fn orchestrate_topology_operator_with_contributions_proof<I>(
            &self,
            input: TopologyOperatorContributionInput<I>,
        ) -> TopologyOperatorContributionProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_with_contributions_proof(input)
        }

        fn recover_topology_operator_contribution_checked<I>(
            &self,
            checked: TopologyOperatorContributionChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_contribution_composed_checked(checked)
        }

        fn recover_topology_operator_contribution_proof<I>(
            &self,
            proof: TopologyOperatorContributionProof<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_contribution_composed_proof(proof)
        }
    };
}
