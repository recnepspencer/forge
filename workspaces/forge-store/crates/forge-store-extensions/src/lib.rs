#![forbid(unsafe_code)]

mod customization;

pub use customization::{
    layout_customization_catalog, ExtensionFamilyPosture, FutureLayoutCustomizationAdmission,
    FutureLayoutCustomizationAdmissionRequest, FutureLayoutCustomizationDeferred,
    FutureLayoutCustomizationDenial, FutureLayoutCustomizationOutcome, FutureLayoutTarget,
    FutureLayoutTargetDeclaration,
};
