use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum ForgeServerCompatHttpRouteFamily {
    Read,
    Mutation,
    Streaming,
    Upload,
    Download,
    Preflight,
}

impl ForgeServerCompatHttpRouteFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Mutation => "mutation",
            Self::Streaming => "streaming",
            Self::Upload => "upload",
            Self::Download => "download",
            Self::Preflight => "preflight",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct ForgeServerCompatHttpRouteFamilies {
    families: BTreeSet<ForgeServerCompatHttpRouteFamily>,
}

impl ForgeServerCompatHttpRouteFamilies {
    pub fn new(families: impl IntoIterator<Item = ForgeServerCompatHttpRouteFamily>) -> Self {
        Self {
            families: families.into_iter().collect(),
        }
    }

    pub fn all_phase_one() -> Self {
        Self::new([
            ForgeServerCompatHttpRouteFamily::Read,
            ForgeServerCompatHttpRouteFamily::Mutation,
            ForgeServerCompatHttpRouteFamily::Streaming,
            ForgeServerCompatHttpRouteFamily::Upload,
            ForgeServerCompatHttpRouteFamily::Download,
            ForgeServerCompatHttpRouteFamily::Preflight,
        ])
    }

    pub fn contains(&self, family: ForgeServerCompatHttpRouteFamily) -> bool {
        self.families.contains(&family)
    }

    pub fn iter(&self) -> impl Iterator<Item = ForgeServerCompatHttpRouteFamily> + '_ {
        self.families.iter().copied()
    }
}
