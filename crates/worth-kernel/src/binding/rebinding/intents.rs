use worth_spatial::facade::bindings::{
    LocalTopologyReplacementNeighborhood, SpatialAdmittedPrimitiveBinding, SpatialBindingKind,
};

#[derive(Clone, Debug, PartialEq)]
pub enum AuthorPrimitiveRebindingIntent {
    ReplaceSurfaceBinding {
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    },
    ReplaceCurveBinding {
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    },
    ReplacePCurveBinding {
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    },
    ReplaceGeometryBinding {
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    },
}

impl AuthorPrimitiveRebindingIntent {
    pub fn replace_surface_binding(
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceSurfaceBinding {
            prior_binding,
            neighborhood,
        }
    }

    pub fn replace_curve_binding(
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceCurveBinding {
            prior_binding,
            neighborhood,
        }
    }

    pub fn replace_pcurve_binding(
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplacePCurveBinding {
            prior_binding,
            neighborhood,
        }
    }

    pub fn replace_geometry_binding(
        prior_binding: SpatialAdmittedPrimitiveBinding,
        neighborhood: LocalTopologyReplacementNeighborhood,
    ) -> Self {
        Self::ReplaceGeometryBinding {
            prior_binding,
            neighborhood,
        }
    }

    pub fn prior_binding(&self) -> &SpatialAdmittedPrimitiveBinding {
        match self {
            Self::ReplaceSurfaceBinding { prior_binding, .. }
            | Self::ReplaceCurveBinding { prior_binding, .. }
            | Self::ReplacePCurveBinding { prior_binding, .. }
            | Self::ReplaceGeometryBinding { prior_binding, .. } => prior_binding,
        }
    }

    pub fn neighborhood(&self) -> &LocalTopologyReplacementNeighborhood {
        match self {
            Self::ReplaceSurfaceBinding { neighborhood, .. }
            | Self::ReplaceCurveBinding { neighborhood, .. }
            | Self::ReplacePCurveBinding { neighborhood, .. }
            | Self::ReplaceGeometryBinding { neighborhood, .. } => neighborhood,
        }
    }

    pub fn binding_kind(&self) -> SpatialBindingKind {
        self.prior_binding().kind()
    }

    pub fn rebinding_kind_label(&self) -> &'static str {
        match self {
            Self::ReplaceSurfaceBinding { .. } => "surface_rebinding",
            Self::ReplaceCurveBinding { .. } => "curve_rebinding",
            Self::ReplacePCurveBinding { .. } => "pcurve_rebinding",
            Self::ReplaceGeometryBinding { .. } => "geometry_rebinding",
        }
    }
}
