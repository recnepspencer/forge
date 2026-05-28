#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyDomainQueryFallbackPosture {
    None,
}

#[allow(dead_code)]
impl TopologyDomainQueryFallbackPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}




