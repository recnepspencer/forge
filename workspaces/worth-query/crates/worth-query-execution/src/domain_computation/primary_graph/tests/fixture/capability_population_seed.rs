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

pub(super) fn bind_exact_pair_grants(
    bootstrap: &mut WorthQueryPrimaryGraphBootstrap<IdentityExecutionSchema>,
    count: usize,
) {
    for ordinal in 0..count {
        let grant = format!("exact-pair-{ordinal}");
        super::capability_seed::bind_grant_entity(bootstrap, &grant, 90, 110, 0, 50);
        super::capability_seed::bind_actor_relation(
            bootstrap,
            CapabilityGrantee::reference(),
            &format!("{grant}-grantee"),
            "principal-0",
            &grant,
        );
        super::capability_seed::bind_actor_relation(
            bootstrap,
            CapabilityGrantor::reference(),
            &format!("{grant}-grantor"),
            "principal-0",
            &grant,
        );
        super::capability_seed::bind_resource(bootstrap, &grant, "account-1");
        super::capability_seed::bind_related(bootstrap, &grant, "account-2");
    }
}
