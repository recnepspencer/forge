pub(super) fn from_host(
    transform: worth_ui_host_contract::UiHostCoordinateTransform,
) -> worth_ui_inspection::UiVisualCoordinateObservation {
    worth_ui_inspection::UiVisualCoordinateObservation::from_runtime_projection(
        worth_ui_inspection::UiVisualCoordinateObservationInput {
            native_client_origin: transform.native_client_origin(),
            client_physical_dimensions: transform.client_physical_dimensions(),
            viewport_logical_dimensions: transform.viewport_logical_dimensions(),
            scale: transform.scale(),
            translation: transform.translation(),
            orientation: orientation(transform.orientation()),
            rounding: rounding(transform.rounding()),
        },
    )
}

fn orientation(
    orientation: worth_ui_host_contract::UiHostCoordinateOrientation,
) -> worth_ui_inspection::UiVisualCoordinateOrientation {
    match orientation {
        worth_ui_host_contract::UiHostCoordinateOrientation::TopLeftOrigin => {
            worth_ui_inspection::UiVisualCoordinateOrientation::TopLeftOrigin
        }
        worth_ui_host_contract::UiHostCoordinateOrientation::BottomLeftOrigin => {
            worth_ui_inspection::UiVisualCoordinateOrientation::BottomLeftOrigin
        }
    }
}

fn rounding(
    rounding: worth_ui_host_contract::UiHostCoordinateRounding,
) -> worth_ui_inspection::UiVisualCoordinateRounding {
    match rounding {
        worth_ui_host_contract::UiHostCoordinateRounding::PixelCenterNearest => {
            worth_ui_inspection::UiVisualCoordinateRounding::PixelCenterNearest
        }
        worth_ui_host_contract::UiHostCoordinateRounding::FloorEdges => {
            worth_ui_inspection::UiVisualCoordinateRounding::FloorEdges
        }
    }
}
