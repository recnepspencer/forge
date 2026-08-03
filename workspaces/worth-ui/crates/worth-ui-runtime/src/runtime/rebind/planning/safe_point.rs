#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindSafePoint {
    PreClassification,
    PostClassification,
    PostScope,
    PostPlan,
    PostReservation,
    FinalCurrentBasisAdmission,
    PreFirstHostEffect,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum UiRebindSafePointPolicy {
    CanonicalPreEffect,
}

impl UiRebindSafePointPolicy {
    pub const fn safe_points(self) -> &'static [UiRebindSafePoint; 7] {
        &[
            UiRebindSafePoint::PreClassification,
            UiRebindSafePoint::PostClassification,
            UiRebindSafePoint::PostScope,
            UiRebindSafePoint::PostPlan,
            UiRebindSafePoint::PostReservation,
            UiRebindSafePoint::FinalCurrentBasisAdmission,
            UiRebindSafePoint::PreFirstHostEffect,
        ]
    }
}
