use std::collections::BTreeSet;

#[derive(Clone, Copy, Debug, Eq, Hash, Ord, PartialEq, PartialOrd)]
pub enum WorthServerCompatHttpRouteFamily {
    Read,
    Query,
    Mutation,
    Streaming,
    Upload,
    Download,
    Preflight,
}

impl WorthServerCompatHttpRouteFamily {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::Read => "read",
            Self::Query => "query",
            Self::Mutation => "mutation",
            Self::Streaming => "streaming",
            Self::Upload => "upload",
            Self::Download => "download",
            Self::Preflight => "preflight",
        }
    }
}

#[derive(Clone, Debug, Default, Eq, PartialEq)]
pub struct WorthServerCompatHttpRouteFamilies {
    families: BTreeSet<WorthServerCompatHttpRouteFamily>,
}

impl WorthServerCompatHttpRouteFamilies {
    pub fn new(families: impl IntoIterator<Item = WorthServerCompatHttpRouteFamily>) -> Self {
        Self {
            families: families.into_iter().collect(),
        }
    }

    pub fn all_phase_one() -> Self {
        Self::new([
            WorthServerCompatHttpRouteFamily::Read,
            WorthServerCompatHttpRouteFamily::Query,
            WorthServerCompatHttpRouteFamily::Mutation,
            WorthServerCompatHttpRouteFamily::Streaming,
            WorthServerCompatHttpRouteFamily::Upload,
            WorthServerCompatHttpRouteFamily::Download,
            WorthServerCompatHttpRouteFamily::Preflight,
        ])
    }

    pub fn contains(&self, family: WorthServerCompatHttpRouteFamily) -> bool {
        self.families.contains(&family)
    }

    pub fn iter(&self) -> impl Iterator<Item = WorthServerCompatHttpRouteFamily> + '_ {
        self.families.iter().copied()
    }
}
