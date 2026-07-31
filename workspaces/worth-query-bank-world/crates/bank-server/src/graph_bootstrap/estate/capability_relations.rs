use bank_domain::{estate::BankEstateWorld, schema::*};
use worth_query_host::facade::primary_graph::{
    WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
};

use super::{
    super::{account_key, institution_key, principal_key},
    keys::{branch, estate, grant},
    relation_seed,
};

pub(super) fn bind(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.grants() {
        let capability = grant(value.id.get());
        relation_seed::bind(
            graph,
            CapabilityGrantee::reference(),
            format!("capability-grantee:{}", value.id.get()),
            principal_key(value.grantee.get()),
            capability.clone(),
        )?;
        relation_seed::bind(
            graph,
            CapabilityGrantor::reference(),
            format!("capability-grantor:{}", value.id.get()),
            principal_key(value.grantor.get()),
            capability.clone(),
        )?;
        relation_seed::bind(
            graph,
            CapabilityEstate::reference(),
            format!("capability-estate:{}", value.id.get()),
            capability.clone(),
            estate(value.scope.estate.get()),
        )?;
        relation_seed::bind(
            graph,
            CapabilityInstitution::reference(),
            format!("capability-institution:{}", value.id.get()),
            capability.clone(),
            institution_key(value.scope.institution.get()),
        )?;
        relation_seed::bind(
            graph,
            CapabilityBranch::reference(),
            format!("capability-branch:{}", value.id.get()),
            capability.clone(),
            branch(value.scope.branch.get()),
        )?;
        if let Some(account) = value.scope.account {
            relation_seed::bind(
                graph,
                CapabilityAccount::reference(),
                format!("capability-account:{}", value.id.get()),
                capability.clone(),
                account_key(account),
            )?;
        }
        if let Some(parent) = value.parent {
            relation_seed::bind(
                graph,
                CapabilityParent::reference(),
                format!("capability-parent:{}", value.id.get()),
                capability,
                grant(parent.get()),
            )?;
        }
    }
    Ok(())
}
