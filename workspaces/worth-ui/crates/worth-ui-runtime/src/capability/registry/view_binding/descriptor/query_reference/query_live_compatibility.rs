use worth_query::facade::runtime::{QuerySubscriptionFamily, QuerySubscriptionSupportPosture};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct QueryLiveCompatibility {
    family: QuerySubscriptionFamily,
    posture: QuerySubscriptionSupportPosture,
}

impl QueryLiveCompatibility {
    pub fn from_subscription_posture(
        family: QuerySubscriptionFamily,
        posture: QuerySubscriptionSupportPosture,
    ) -> Self {
        Self { family, posture }
    }

    pub fn declaration_only(family: QuerySubscriptionFamily) -> Self {
        Self::from_subscription_posture(
            family,
            QuerySubscriptionSupportPosture::RuntimeBackedCertified,
        )
    }

    pub fn posture(&self) -> QuerySubscriptionSupportPosture {
        self.posture
    }

    pub fn is_admitted(&self) -> bool {
        self.posture == QuerySubscriptionSupportPosture::RuntimeBackedCertified
    }

    pub fn digest_basis(&self) -> String {
        format!("{}|{}", self.family.as_str(), self.posture.as_str())
    }
}
