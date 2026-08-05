use crate::model::BankPrincipalId;

use super::{DeathNoticeId, EstateCaseId};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub struct EstateDeathNotificationRequest {
    estate: EstateCaseId,
    notice: DeathNoticeId,
    subject: BankPrincipalId,
}

impl EstateDeathNotificationRequest {
    pub const fn new(
        estate: EstateCaseId,
        notice: DeathNoticeId,
        subject: BankPrincipalId,
    ) -> Self {
        Self {
            estate,
            notice,
            subject,
        }
    }

    pub const fn estate(self) -> EstateCaseId {
        self.estate
    }

    pub const fn notice(self) -> DeathNoticeId {
        self.notice
    }

    pub const fn subject(self) -> BankPrincipalId {
        self.subject
    }
}
