#[derive(Debug, Clone, Copy)]
struct StageTaskOrderWitness;

#[derive(Debug, Clone, Copy)]
pub(in crate::logic::planner) struct StageTaskOrderProof(StageTaskOrderWitness);

impl StageTaskOrderProof {
    pub(in crate::logic::planner) fn established() -> Self {
        Self(StageTaskOrderWitness)
    }
}

#[derive(Debug, Clone, Copy)]
pub(in crate::logic::planner) struct ExactStageWidth(usize);

impl ExactStageWidth {
    pub(in crate::logic::planner) fn new(width: usize) -> Self {
        Self(width)
    }

    pub(in crate::logic::planner) fn get(self) -> usize {
        self.0
    }
}
