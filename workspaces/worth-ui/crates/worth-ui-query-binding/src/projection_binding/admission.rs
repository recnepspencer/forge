use super::{
    UiCollectionProjectionBinding, UiProjectionBindingStopReceipt, UiScalarProjectionBinding,
};

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiScalarProjectionBindingAdmission {
    Ready(UiScalarProjectionBinding),
    Stopped(UiProjectionBindingStopReceipt),
}

#[must_use]
#[derive(Debug, Eq, PartialEq)]
pub enum UiCollectionProjectionBindingAdmission {
    Ready(UiCollectionProjectionBinding),
    Stopped(UiProjectionBindingStopReceipt),
}
