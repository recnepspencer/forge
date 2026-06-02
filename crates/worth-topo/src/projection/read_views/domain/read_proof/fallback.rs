#[allow(dead_code)]
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord)]
pub enum TopologyReadFallbackPosture {
    None,
}

#[allow(dead_code)]
impl TopologyReadFallbackPosture {
    pub fn as_str(self) -> &'static str {
        match self {
            Self::None => "none",
        }
    }
}
