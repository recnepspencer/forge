use worth_query_decl::facade::application_capability::{
    ApplicationCapabilityGovernedInputIdentity, ApplicationCapabilityRequest,
    ApplicationCapabilityRequestProjection, ApplicationCapabilityRequestProjectionDenial,
};

use crate::{
    estate::EstateAction,
    schema::{BankSchema, EstateActionContext, EstateCase, NotifyDeathEstateCapability},
};

use super::estate_request;

impl ApplicationCapabilityRequest<BankSchema, NotifyDeathEstateCapability> for EstateAction {
    type Scope = EstateCase;
    type Context = EstateActionContext;

    fn governed_input_identity(&self) -> Option<ApplicationCapabilityGovernedInputIdentity> {
        let EstateAction::NotifyDeath {
            estate,
            notice,
            subject,
        } = *self
        else {
            return None;
        };
        Some(ApplicationCapabilityGovernedInputIdentity::four_u64([
            estate.get(),
            notice.get(),
            subject.get(),
            0,
        ]))
    }

    fn capability_request(
        &self,
    ) -> Result<
        ApplicationCapabilityRequestProjection<BankSchema, Self::Scope, Self::Context>,
        ApplicationCapabilityRequestProjectionDenial,
    > {
        let EstateAction::NotifyDeath { estate, .. } = *self else {
            return Err(ApplicationCapabilityRequestProjectionDenial::input_variant(
                "NotifyDeathEstateOperation",
            ));
        };
        Ok(estate_request(self, estate))
    }
}

#[cfg(test)]
mod tests {
    use std::collections::BTreeSet;

    use super::*;
    use crate::{
        estate::{DeathNoticeId, EstateCaseId},
        model::BankPrincipalId,
    };

    #[test]
    fn governed_identity_covers_every_notification_dimension() {
        let identities = [
            action(1, 2, 3),
            action(4, 2, 3),
            action(1, 4, 3),
            action(1, 2, 4),
        ]
        .map(|action| {
            <EstateAction as ApplicationCapabilityRequest<
                BankSchema,
                NotifyDeathEstateCapability,
            >>::governed_input_identity(&action)
            .unwrap()
            .identity()
        });
        assert_eq!(identities.into_iter().collect::<BTreeSet<_>>().len(), 4);
    }

    fn action(estate: u64, notice: u64, subject: u64) -> EstateAction {
        EstateAction::NotifyDeath {
            estate: EstateCaseId::new(estate).unwrap(),
            notice: DeathNoticeId::new(notice).unwrap(),
            subject: BankPrincipalId::new(subject).unwrap(),
        }
    }
}
