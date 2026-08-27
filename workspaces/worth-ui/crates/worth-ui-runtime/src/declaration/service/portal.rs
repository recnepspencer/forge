#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredPortalSurfaceContract {
    MountedOverlay,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) struct UiDeclaredPortalPlacementGeometry {
    preferred_width: u16,
    maximum_height: u16,
    anchor_gap: u8,
    viewport_margin: u8,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum UiDeclaredPortalPlacementGeometryDenial {
    EmptyExtent,
    MarginConsumesExtent,
}

impl UiDeclaredPortalSurfaceContract {
    pub(crate) const fn family(self) -> crate::capability::UiRuntimeServiceFamily {
        crate::capability::UiRuntimeServiceFamily::Portal
    }
}

impl UiDeclaredPortalPlacementGeometry {
    pub(crate) const fn dropdown() -> Self {
        Self {
            preferred_width: 280,
            maximum_height: 320,
            anchor_gap: 8,
            viewport_margin: 16,
        }
    }

    pub(crate) const fn checked(
        preferred_width: u16,
        maximum_height: u16,
        anchor_gap: u8,
        viewport_margin: u8,
    ) -> Result<Self, UiDeclaredPortalPlacementGeometryDenial> {
        if preferred_width == 0 || maximum_height == 0 {
            return Err(UiDeclaredPortalPlacementGeometryDenial::EmptyExtent);
        }
        if viewport_margin as u16 * 2 >= preferred_width
            || viewport_margin as u16 * 2 >= maximum_height
        {
            return Err(UiDeclaredPortalPlacementGeometryDenial::MarginConsumesExtent);
        }
        Ok(Self {
            preferred_width,
            maximum_height,
            anchor_gap,
            viewport_margin,
        })
    }

    pub(crate) const fn preferred_width(self) -> u16 {
        self.preferred_width
    }

    pub(crate) const fn maximum_height(self) -> u16 {
        self.maximum_height
    }

    pub(crate) const fn anchor_gap(self) -> u8 {
        self.anchor_gap
    }

    pub(crate) const fn viewport_margin(self) -> u8 {
        self.viewport_margin
    }
}
