macro_rules! declaration_and_route_workflow_methods {
    () => {
        fn declare_topology_operator<I>(
            &self,
            declaration: I,
        ) -> Result<
            TopologyOperatorCanonicalDeclaration<I>,
            TopologyOperatorDeclarationAdmissionError<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.declare(declaration)
        }

        fn review_topology_operator<I>(
            &self,
            declaration: TopologyOperatorCanonicalDeclaration<I>,
        ) -> Result<
            TopologyOperatorDeclarationLegalityEvidence<I>,
            TopologyOperatorDeclarationLegalityDenial<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.review_legality(declaration)
        }

        fn orchestrate_topology_operator_outcome<I>(
            &self,
            declaration: I,
        ) -> TopologyOperatorDeclarationOutcome<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_entry_outcome(declaration)
        }

        fn orchestrate_topology_operator_envelope<I>(
            &self,
            declaration: I,
        ) -> Result<TopologyOperatorEnvelope<I>, TopologyOperatorEnvelopeTerminalError<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_entry(declaration)
        }

        fn orchestrate_topology_operator_envelope_checked<I>(
            &self,
            declaration: I,
        ) -> TopologyOperatorEnvelopeChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_entry_checked(declaration)
        }

        fn orchestrate_topology_operator_envelope_proof<I>(
            &self,
            declaration: I,
        ) -> TopologyOperatorEnvelopeProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_declaration_entry_proof(declaration)
        }

        fn declare_review_and_progress_topology_operator<I>(
            &self,
            declaration: I,
        ) -> Result<TopologyOperatorProgressedDeclaration<I>, TopologyOperatorProgressionError<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.declare_review_and_progress(declaration)
        }

        fn orchestrate_topology_operator_route<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> Result<TopologyOperatorRoutePlan<I>, TopologyOperatorRoutePlanTerminalError<I>>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_routes_from_progressed(progressed)
        }

        fn orchestrate_topology_operator_route_checked<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorRoutePlanChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_routes_from_progressed_checked(progressed)
        }

        fn orchestrate_topology_operator_route_proof<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorRoutePlanProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_routes_from_progressed_proof(progressed)
        }

        fn orchestrate_topology_operator_receipt_checked<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorDeclarationReceiptChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_receipt_from_progressed_checked(progressed)
        }

        fn orchestrate_topology_operator_receipt<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> Result<
            TopologyOperatorDeclarationReceipt<I>,
            TopologyOperatorDeclarationReceiptTerminalError<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_receipt_from_progressed(progressed)
        }

        fn orchestrate_topology_operator_receipt_proof<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorDeclarationReceiptProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_receipt_from_progressed_proof(progressed)
        }

        fn orchestrate_topology_operator_envelope_from_progressed<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> Result<
            TopologyOperatorEnvelope<I>,
            TopologyOperatorEnvelopeFromProgressedTerminalError<I>,
        >
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_envelope_from_progressed(progressed)
        }

        fn orchestrate_topology_operator_envelope_from_progressed_checked<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorEnvelopeFromProgressedChecked<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_envelope_from_progressed_checked(progressed)
        }

        fn orchestrate_topology_operator_envelope_from_progressed_proof<I>(
            &self,
            progressed: TopologyOperatorProgressedDeclaration<I>,
        ) -> TopologyOperatorEnvelopeFromProgressedProof<I>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.orchestrate_envelope_from_progressed_proof(progressed)
        }

        fn recover_topology_operator_route_checked<I>(
            &self,
            checked: TopologyOperatorRoutePlanChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_declaration_route_plan_checked(checked)
        }

        fn recover_topology_operator_envelope_checked<I>(
            &self,
            checked: TopologyOperatorEnvelopeChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_declaration_entry_checked(checked)
        }

        fn recover_topology_operator_envelope_proof<I>(
            &self,
            proof: &TopologyOperatorEnvelopeProof<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_declaration_entry_proof(proof)
        }

        fn recover_topology_operator_receipt_checked<I>(
            &self,
            checked: TopologyOperatorDeclarationReceiptChecked<I>,
        ) -> Option<ForgeQueryRecoveryBrief>
        where
            I: ForgeQueryDeclarationInput<TopologyQueryDomain>,
        {
            self.recover_from_declaration_receipt_checked(checked)
        }
    };
}
