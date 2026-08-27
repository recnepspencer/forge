use worth_query_host::facade::primary_graph;

use super::super::adapters::block_on;
use super::super::schema::*;
use super::{admit_identity_adapter, request_scope, CourtroomWorld};

impl CourtroomWorld {
    pub fn amend_intent(&mut self, revision: u64, lifecycle: &str, gate: &str) {
        self.supersede_intent(revision, 5, lifecycle, "payload", gate);
    }

    pub fn supersede_intent(
        &mut self,
        revision: u64,
        due: u64,
        lifecycle: &str,
        input: &str,
        gate: &str,
    ) {
        self.commit_amendment(revision, due, lifecycle, input, gate);
    }

    fn commit_amendment(
        &mut self,
        revision: u64,
        due: u64,
        lifecycle: &str,
        input: &str,
        gate: &str,
    ) {
        let schema = self.application.installed_schema();
        let principal_binding = schema
            .principal_binding(TemporalPrincipalBinding::reference())
            .unwrap();
        let authentication = admit_identity_adapter(schema);
        let request = request_scope();
        let external = block_on(authentication.authenticate((), &request)).unwrap();
        let principal = self
            .application
            .resolve_authenticated_principal(
                &principal_binding,
                external,
                &request,
                primary_graph::WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .unwrap();
        let intent = self
            .application
            .resolve_entity(
                IntentIdentityField::reference(),
                "intent-1".to_string(),
                &request,
                primary_graph::WorthQueryPrincipalResolutionMode::Ordinary,
            )
            .unwrap();
        let operation = schema
            .installed_operation(AmendTemporal::reference())
            .unwrap();
        let admission = self
            .application
            .authorize_operation(
                &principal,
                &intent,
                &operation,
                Default::default(),
                &request,
            )
            .unwrap();
        let (_, projection, _) = self
            .invariant
            .project_admitted_operation(&admission, |reader, scope| {
                reader
                    .decision_field(scope, IntentRevisionField::reference())
                    .unwrap();
                reader
                    .decision_field(scope, IntentLifecycleField::reference())
                    .unwrap();
                reader
                    .decision_field(scope, IntentGateField::reference())
                    .unwrap();
                reader
                    .decision_field(scope, IntentDueField::reference())
                    .unwrap();
                reader
                    .decision_field(scope, IntentInputField::reference())
                    .unwrap();
            })
            .unwrap()
            .into_parts();
        let reads = self
            .application
            .begin_projected_application_read_attempt(admission, projection)
            .unwrap();
        let mut effects = reads
            .complete_projected_dependencies()
            .unwrap()
            .begin_effect_program();
        let intent = effects.existing_entity(&intent).unwrap();
        effects
            .write_field(&intent, IntentRevisionField::reference(), revision)
            .unwrap();
        effects
            .write_field(
                &intent,
                IntentLifecycleField::reference(),
                lifecycle.to_string(),
            )
            .unwrap();
        effects
            .write_field(&intent, IntentDueField::reference(), due)
            .unwrap();
        effects
            .write_field(&intent, IntentInputField::reference(), input.to_string())
            .unwrap();
        effects
            .write_field(&intent, IntentGateField::reference(), gate.to_string())
            .unwrap();
        self.amendment_ordinal = self.amendment_ordinal.saturating_add(1);
        let idempotency = primary_graph::WorthQueryApplicationIdempotencyBinding::new(
            [0x91 ^ self.amendment_ordinal; 32],
            [0xA0 ^ self.amendment_ordinal; 32],
        );
        let outcome = self
            .application
            .compare_and_commit_application(effects.finish().unwrap(), idempotency);
        assert!(
            matches!(
                outcome,
                primary_graph::WorthQueryApplicationCommitOutcome::Committed(_)
            ),
            "unexpected amendment outcome: {outcome:?}"
        );
    }
}
