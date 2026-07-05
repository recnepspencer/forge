#[derive(Clone, Copy, Debug, Eq, Ord, PartialEq, PartialOrd)]
pub enum UiDeclaredMeasurementConstraintModifier {
    Bounded,
}

pub(crate) fn measurement_constraint_modifier_claim(
    claim: &str,
) -> Option<UiDeclaredMeasurementConstraintModifier> {
    match claim {
        "measurement:constraint:bounded" => Some(UiDeclaredMeasurementConstraintModifier::Bounded),
        _ => None,
    }
}
