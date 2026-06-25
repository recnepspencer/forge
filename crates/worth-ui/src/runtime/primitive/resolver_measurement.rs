use crate::capability::{DensityTokenId, WorthUiDensityValue};
use crate::runtime::WorthUiRuntimeHost;

use super::{
    WorthUiBoxEdges, WorthUiPrimitiveProofDenial, WorthUiPrimitiveResolvedInsets,
    WorthUiPrimitiveResolvedMeasurement, WorthUiValidatedPrimitivePropSet,
};

pub(super) fn resolve_measurement_receipt(
    runtime: &WorthUiRuntimeHost,
    props: &WorthUiValidatedPrimitivePropSet,
) -> Result<super::WorthUiPrimitiveMeasurementReceipt, WorthUiPrimitiveProofDenial> {
    Ok(super::WorthUiPrimitiveMeasurementReceipt::new(
        resolve_padding(runtime, props.padding_token())?,
        resolve_measurement(runtime, props.radius_token())?,
    ))
}

pub(super) fn resolve_measurement(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<WorthUiPrimitiveResolvedMeasurement, WorthUiPrimitiveProofDenial> {
    let token = parse_density_token(token_text)?;
    let Some(descriptor) = runtime.inspect_active_density_token_descriptor(&token) else {
        return Err(
            WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
                token: token_text.to_owned(),
            },
        );
    };
    let points = match descriptor.value() {
        WorthUiDensityValue::Padding(value) => value.horizontal_points(),
        WorthUiDensityValue::Spacing(value) => value.points(),
        WorthUiDensityValue::HitTargetMinimum(value) => value.points(),
        WorthUiDensityValue::Posture(_) => {
            return Err(WorthUiPrimitiveProofDenial::WrongPrimitiveMeasurementKind {
                token: token_text.to_owned(),
                expected: "padding, spacing, or length".to_owned(),
                actual: "posture".to_owned(),
            });
        }
    };
    Ok(WorthUiPrimitiveResolvedMeasurement::new(&token, points))
}

fn resolve_padding(
    runtime: &WorthUiRuntimeHost,
    token_text: &str,
) -> Result<WorthUiPrimitiveResolvedInsets, WorthUiPrimitiveProofDenial> {
    let token = parse_density_token(token_text)?;
    let Some(descriptor) = runtime.inspect_active_density_token_descriptor(&token) else {
        return Err(
            WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
                token: token_text.to_owned(),
            },
        );
    };
    let edges = match descriptor.value() {
        WorthUiDensityValue::Padding(value) => WorthUiBoxEdges::new(
            value.top().points(),
            value.right().points(),
            value.bottom().points(),
            value.left().points(),
        ),
        WorthUiDensityValue::Spacing(value) => WorthUiBoxEdges::uniform(value.points()),
        WorthUiDensityValue::HitTargetMinimum(value) => WorthUiBoxEdges::uniform(value.points()),
        WorthUiDensityValue::Posture(_) => {
            return Err(WorthUiPrimitiveProofDenial::WrongPrimitiveMeasurementKind {
                token: token_text.to_owned(),
                expected: "padding, spacing, or length".to_owned(),
                actual: "posture".to_owned(),
            });
        }
    };
    Ok(WorthUiPrimitiveResolvedInsets::new(&token, edges))
}

fn parse_density_token(token_text: &str) -> Result<DensityTokenId, WorthUiPrimitiveProofDenial> {
    DensityTokenId::new(token_text).map_err(|_| {
        WorthUiPrimitiveProofDenial::MissingPrimitiveMeasurementToken {
            token: token_text.to_owned(),
        }
    })
}
