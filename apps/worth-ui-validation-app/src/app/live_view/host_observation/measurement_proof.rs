use worth_ui::facade::{
    WorthUiAdmittedHostFrameObservationReceipt, WorthUiHostObservationAdmissionDenial,
    WorthUiMeasuredProductViewReceipt,
};

#[derive(Clone, Debug, PartialEq)]
pub struct ValidationLiveViewFrameMeasurementProof {
    outcome: ValidationHostFrameObservationOutcome,
    measured_product_view: Option<WorthUiMeasuredProductViewReceipt>,
    measurement_denial: Option<WorthUiHostObservationAdmissionDenial>,
}

#[derive(Clone, Debug, PartialEq)]
pub enum ValidationHostFrameObservationOutcome {
    Admitted(WorthUiAdmittedHostFrameObservationReceipt),
    Denied(Vec<WorthUiHostObservationAdmissionDenial>),
    Unavailable(ValidationHostFrameObservationUnavailable),
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum ValidationHostFrameObservationUnavailable {
    ProjectionUnavailable,
    SurfaceNodeUnavailable,
}

impl ValidationLiveViewFrameMeasurementProof {
    pub(super) fn from_admitted(
        admitted: WorthUiAdmittedHostFrameObservationReceipt,
        measurement: Result<
            WorthUiMeasuredProductViewReceipt,
            WorthUiHostObservationAdmissionDenial,
        >,
    ) -> Self {
        let (measured_product_view, measurement_denial) =
            classify_measured_product_view_result(measurement);
        Self {
            outcome: ValidationHostFrameObservationOutcome::Admitted(admitted),
            measured_product_view,
            measurement_denial,
        }
    }

    pub(super) fn denied(denials: Vec<WorthUiHostObservationAdmissionDenial>) -> Self {
        Self {
            outcome: ValidationHostFrameObservationOutcome::Denied(denials),
            measured_product_view: None,
            measurement_denial: None,
        }
    }

    pub(super) fn unavailable(reason: ValidationHostFrameObservationUnavailable) -> Self {
        Self {
            outcome: ValidationHostFrameObservationOutcome::Unavailable(reason),
            measured_product_view: None,
            measurement_denial: None,
        }
    }

    pub fn outcome(&self) -> &ValidationHostFrameObservationOutcome {
        &self.outcome
    }

    pub fn admitted(&self) -> Option<&WorthUiAdmittedHostFrameObservationReceipt> {
        match &self.outcome {
            ValidationHostFrameObservationOutcome::Admitted(receipt) => Some(receipt),
            ValidationHostFrameObservationOutcome::Denied(_)
            | ValidationHostFrameObservationOutcome::Unavailable(_) => None,
        }
    }

    pub fn measured_product_view(&self) -> Option<&WorthUiMeasuredProductViewReceipt> {
        self.measured_product_view.as_ref()
    }

    pub fn measurement_denial(&self) -> Option<&WorthUiHostObservationAdmissionDenial> {
        self.measurement_denial.as_ref()
    }
}

fn classify_measured_product_view_result(
    measurement: Result<WorthUiMeasuredProductViewReceipt, WorthUiHostObservationAdmissionDenial>,
) -> (
    Option<WorthUiMeasuredProductViewReceipt>,
    Option<WorthUiHostObservationAdmissionDenial>,
) {
    match measurement {
        Ok(receipt) => (Some(receipt), None),
        Err(denial) => (None, Some(denial)),
    }
}
