use super::capability::*;
use super::IdentityExecutionSchema;
use crate::domain_computation::primary_graph::WorthQueryPrimaryGraphBootstrap;

pub(super) fn bind_same_resource_unrelated_grants(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
    unrelated: usize,
) {
    super::capability_seed::bind_grant(bootstrap);
    for ordinal in 0..unrelated {
        let grant = format!("same-resource-unrelated-{ordinal}");
        super::capability_seed::bind_grant_entity(bootstrap, &grant, 70, 130, 0, 100);
        super::capability_seed::bind_actor_relation(
            bootstrap,
            CapabilityGrantee::reference(),
            &format!("{grant}-grantee"),
            "principal-1",
            &grant,
        );
        super::capability_seed::bind_actor_relation(
            bootstrap,
            CapabilityGrantor::reference(),
            &format!("{grant}-grantor"),
            "principal-1",
            &grant,
        );
        super::capability_seed::bind_resource(bootstrap, &grant, "account-1");
        super::capability_seed::bind_related(bootstrap, &grant, "account-2");
    }
}
