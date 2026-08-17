use std::rc::Rc;

use super::{WorthUiPreparedApplicationAuthorities, WorthUiPreparedApplicationAuthorityInput};
use crate::facade::prepared_application_authority::generation_identity::{
    WorthUiPreparedApplicationGenerationIdentity, WorthUiPreparedGenerationIdentityInput,
};
use crate::facade::prepared_application_authority::lowering_authority::{
    WorthUiPreparedApplicationLoweringAuthority, WorthUiPreparedApplicationLoweringInput,
};

pub(super) fn derive_prepared_application_authorities(
    input: &WorthUiPreparedApplicationAuthorityInput,
) -> WorthUiPreparedApplicationAuthorities {
    let generation_identity = WorthUiPreparedApplicationGenerationIdentity::derive(
        WorthUiPreparedGenerationIdentityInput {
            capability_snapshot: input.capability_snapshot.digest(),
            canonical_artifact: input.canonical_artifact.identity(),
            lineage: input.generation_lineage.clone(),
            declaration_source: input.declaration_source_identity.clone(),
            semantic_package: input.semantic_handoff.identity().clone(),
            graph_authority_digest: input.graph_snapshot.authority_digest(),
            query_binding_plan: &input.query_binding_plan,
            intent_application_fact_digest: input.intent_application_facts.digest_basis(),
            intent_execution_binding_digest: input.intent_execution_bindings.digest_basis(),
            visual_inspection_policy: input.visual_inspection_policy,
            change_profile: input.change_profile,
        },
    );
    let lowering_authority = WorthUiPreparedApplicationLoweringAuthority::seal(
        WorthUiPreparedApplicationLoweringInput {
            generation_identity: generation_identity.clone(),
            source_candidate_basis: input.canonical_artifact.candidate_basis(),
            source_artifact_authority: input.canonical_artifact.runtime_artifact_authority().0,
            graph_authority_identity: input.graph_snapshot.authority_identity(),
            capability_snapshot: Rc::clone(&input.capability_snapshot),
            query_binding_plan: input.query_binding_plan.clone(),
        },
    );
    WorthUiPreparedApplicationAuthorities {
        generation_identity,
        lowering_authority,
    }
}
