use worth_query_decl::facade::{
    application_schema::ApplicationEffectPayload, worth_query_effect,
};

use crate::estate::EstateDeathNotificationRequest;
use crate::schema::BankSchema;

impl ApplicationEffectPayload for EstateDeathNotificationRequest {
    fn retained_bytes(&self) -> u64 {
        u64::try_from(std::mem::size_of::<Self>()).unwrap_or(u64::MAX)
    }
}

worth_query_effect!(
    pub EstateDeathNotificationEffect(EstateDeathNotificationRequest) in BankSchema
);

#[cfg(test)]
mod tests {
    use worth_query_decl::facade::application_schema::ApplicationEffectPayload;

    use super::EstateDeathNotificationRequest;
    use crate::estate::{DeathNoticeId, EstateCaseId};
    use crate::model::BankPrincipalId;

    #[test]
    fn death_notification_request_retains_exact_fixed_width() {
        let request = EstateDeathNotificationRequest::new(
            EstateCaseId::new(1).unwrap(),
            DeathNoticeId::new(2).unwrap(),
            BankPrincipalId::new(3).unwrap(),
        );

        assert_eq!(
            request.retained_bytes(),
            u64::try_from(std::mem::size_of::<EstateDeathNotificationRequest>()).unwrap()
        );
    }
}
