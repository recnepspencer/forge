use super::super::S8LoweredAccessReceipt;

#[derive(Debug, PartialEq, Eq)]
enum S8IndexedLoweringPayload {
    Lowered(S8LoweredAccessReceipt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8IndexedLoweringOutcome {
    case: S8IndexedLoweringPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8IndexedLoweringView<'a> {
    Lowered(&'a S8LoweredAccessReceipt),
}

impl S8IndexedLoweringOutcome {
    pub(crate) fn lowered(value: S8LoweredAccessReceipt) -> Self {
        Self::from_owner_payload(S8IndexedLoweringPayload::Lowered(value))
    }

    fn from_owner_payload(case: S8IndexedLoweringPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8IndexedLoweringView<'_> {
        match &self.case {
            S8IndexedLoweringPayload::Lowered(value) => S8IndexedLoweringView::Lowered(value),
        }
    }

    fn into_owner_payload(self) -> S8IndexedLoweringPayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum S8DegradedLoweringPayload {
    DegradedLowered(S8LoweredAccessReceipt),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8DegradedLoweringOutcome {
    case: S8DegradedLoweringPayload,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8DegradedLoweringView<'a> {
    DegradedLowered(&'a S8LoweredAccessReceipt),
}

impl S8DegradedLoweringOutcome {
    pub(crate) fn lowered(value: S8LoweredAccessReceipt) -> Self {
        Self::from_owner_payload(S8DegradedLoweringPayload::DegradedLowered(value))
    }

    fn from_owner_payload(case: S8DegradedLoweringPayload) -> Self {
        Self { case }
    }

    pub fn view(&self) -> S8DegradedLoweringView<'_> {
        match &self.case {
            S8DegradedLoweringPayload::DegradedLowered(value) => {
                S8DegradedLoweringView::DegradedLowered(value)
            }
        }
    }

    fn into_owner_payload(self) -> S8DegradedLoweringPayload {
        self.case
    }
}

#[derive(Debug, PartialEq, Eq)]
enum LoweringOwnerOutcome {
    Indexed(S8IndexedLoweringOutcome),
    Degraded(S8DegradedLoweringOutcome),
}

#[derive(Debug, PartialEq, Eq)]
pub struct S8AccessLoweringOutcome {
    owner: LoweringOwnerOutcome,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum S8AccessLoweringView<'a> {
    Lowered(&'a S8LoweredAccessReceipt),
}

impl S8AccessLoweringOutcome {
    pub(crate) fn lower(receipt: S8LoweredAccessReceipt) -> Self {
        let owner = if receipt.path_kind().is_degraded_exact_scan() {
            LoweringOwnerOutcome::Degraded(S8DegradedLoweringOutcome::lowered(receipt))
        } else {
            LoweringOwnerOutcome::Indexed(S8IndexedLoweringOutcome::lowered(receipt))
        };
        Self { owner }
    }
    pub fn view(&self) -> S8AccessLoweringView<'_> {
        match &self.owner {
            LoweringOwnerOutcome::Indexed(value) => match value.view() {
                S8IndexedLoweringView::Lowered(receipt) => S8AccessLoweringView::Lowered(receipt),
            },
            LoweringOwnerOutcome::Degraded(value) => match value.view() {
                S8DegradedLoweringView::DegradedLowered(receipt) => {
                    S8AccessLoweringView::Lowered(receipt)
                }
            },
        }
    }
    pub fn into_lowered(self) -> S8LoweredAccessReceipt {
        match self.owner {
            LoweringOwnerOutcome::Indexed(value) => match value.into_owner_payload() {
                S8IndexedLoweringPayload::Lowered(receipt) => receipt,
            },
            LoweringOwnerOutcome::Degraded(value) => match value.into_owner_payload() {
                S8DegradedLoweringPayload::DegradedLowered(receipt) => receipt,
            },
        }
    }
}
