#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub struct RoadmapScope {
    roadmap: &'static str,
    sequence: &'static str,
}

impl RoadmapScope {
    pub const fn new(roadmap: &'static str, sequence: &'static str) -> Self {
        Self { roadmap, sequence }
    }

    pub const fn roadmap(&self) -> &'static str {
        self.roadmap
    }

    pub const fn sequence(&self) -> &'static str {
        self.sequence
    }
}

pub const ROADMAP_2_SCOPE: &str = "Roadmap 2";
pub const ROADMAP_2_S1_SCOPE: RoadmapScope = RoadmapScope::new(ROADMAP_2_SCOPE, "S.1");
