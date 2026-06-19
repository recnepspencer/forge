#[derive(Clone, Copy, Debug, Default, Eq, PartialEq)]
pub struct PlanarBooleanWalkOutcomeCounters {
    walks_classified: usize,
    closed_walks: usize,
    open_walks: usize,
    residual_walks: usize,
    unsupported_walks: usize,
    self_colliding_walks: usize,
    denied_walks: usize,
}

impl PlanarBooleanWalkOutcomeCounters {
    pub(crate) fn classified_walk(&mut self, kind: super::row::PlanarBooleanWalkOutcomeKind) {
        self.walks_classified += 1;
        match kind {
            super::row::PlanarBooleanWalkOutcomeKind::Closed => self.closed_walks += 1,
            super::row::PlanarBooleanWalkOutcomeKind::Open => self.open_walks += 1,
            super::row::PlanarBooleanWalkOutcomeKind::Residual => self.residual_walks += 1,
            super::row::PlanarBooleanWalkOutcomeKind::Unsupported => self.unsupported_walks += 1,
            super::row::PlanarBooleanWalkOutcomeKind::SelfColliding => {
                self.self_colliding_walks += 1
            }
            super::row::PlanarBooleanWalkOutcomeKind::Denied => self.denied_walks += 1,
        }
    }

    pub fn walks_classified(&self) -> usize {
        self.walks_classified
    }

    pub fn closed_walks(&self) -> usize {
        self.closed_walks
    }

    pub fn open_walks(&self) -> usize {
        self.open_walks
    }

    pub fn unsupported_walks(&self) -> usize {
        self.unsupported_walks
    }

    pub fn self_colliding_walks(&self) -> usize {
        self.self_colliding_walks
    }

    pub fn residual_walks(&self) -> usize {
        self.residual_walks
    }

    pub fn denied_walks(&self) -> usize {
        self.denied_walks
    }
}
