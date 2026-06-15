#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceFamily {
    Plane,
    AnalyticNonPlanar,
    Freeform,
    GeneratedFeature,
    Unknown,
}

impl SurfaceFamily {
    pub const ALL: [Self; 5] = [
        Self::Plane,
        Self::AnalyticNonPlanar,
        Self::Freeform,
        Self::GeneratedFeature,
        Self::Unknown,
    ];

    pub fn human_label(self) -> &'static str {
        match self {
            Self::Plane => "plane surface",
            Self::AnalyticNonPlanar => "analytic non-planar surface",
            Self::Freeform => "freeform surface",
            Self::GeneratedFeature => "generated feature surface",
            Self::Unknown => "unknown surface family",
        }
    }

    pub fn is_certified_in_milestone(self) -> bool {
        matches!(self, Self::Plane)
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum SurfaceSupportStatus {
    Certified,
    Unsupported,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct SurfaceSupportMatrixRow {
    family: SurfaceFamily,
    status: SurfaceSupportStatus,
    human_reason: String,
}

impl SurfaceSupportMatrixRow {
    pub(crate) fn for_family(family: SurfaceFamily) -> Self {
        let status = if family.is_certified_in_milestone() {
            SurfaceSupportStatus::Certified
        } else {
            SurfaceSupportStatus::Unsupported
        };
        Self {
            family,
            status,
            human_reason: surface_family_support_reason(family).to_string(),
        }
    }

    pub fn family(&self) -> SurfaceFamily {
        self.family
    }

    pub fn status(&self) -> SurfaceSupportStatus {
        self.status
    }

    pub fn human_reason(&self) -> &str {
        &self.human_reason
    }
}

pub(crate) fn support_matrix_rows() -> Vec<SurfaceSupportMatrixRow> {
    SurfaceFamily::ALL
        .into_iter()
        .map(SurfaceSupportMatrixRow::for_family)
        .collect()
}

fn surface_family_support_reason(family: SurfaceFamily) -> &'static str {
    match family {
        SurfaceFamily::Plane => "plane surfaces are certified for M6.5 surface support.",
        SurfaceFamily::AnalyticNonPlanar => {
            "analytic non-planar surfaces are not admitted for M6.5 surface support."
        }
        SurfaceFamily::Freeform => "freeform surfaces are not admitted for M6.5 surface support.",
        SurfaceFamily::GeneratedFeature => {
            "generated feature surfaces are not admitted for M6.5 surface support."
        }
        SurfaceFamily::Unknown => "unknown surface families are not admitted for M6.5 support.",
    }
}
