use bank_domain::{estate::BankEstateWorld, schema::*};
use worth_query_host::facade::primary_graph::{
    WorthQueryPrimaryGraphBootstrap, WorthQueryPrimaryGraphInstallationDenial,
};

use super::{
    super::principal_key,
    keys::{emergency, grant, review},
    relation_seed,
};

pub(super) fn bind(
    graph: &mut WorthQueryPrimaryGraphBootstrap<BankSchema>,
    world: &BankEstateWorld,
) -> Result<(), WorthQueryPrimaryGraphInstallationDenial> {
    for value in world.emergency_accesses() {
        let access = emergency(value.id.get());
        relation_seed::bind(
            graph,
            EmergencyRequester::reference(),
            format!("emergency-requester:{}", value.id.get()),
            principal_key(value.requester.get()),
            access.clone(),
        )?;
        if let Some(approver) = value.approver {
            relation_seed::bind(
                graph,
                EmergencyApprover::reference(),
                format!("emergency-approver:{}", value.id.get()),
                principal_key(approver.get()),
                access.clone(),
            )?;
        }
        relation_seed::bind(
            graph,
            EmergencyGrant::reference(),
            format!("emergency-grant:{}", value.id.get()),
            access.clone(),
            grant(value.grant.get()),
        )?;
        relation_seed::bind(
            graph,
            EmergencyReview::reference(),
            format!("emergency-review:{}", value.id.get()),
            access,
            review(value.review.get()),
        )?;
    }
    Ok(())
}
