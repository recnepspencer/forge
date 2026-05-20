use crate::construction::PrimitiveConstructionIntent;
use worth_spatial::facade::{SpatialDirectionWitnessRef, SpatialFrameRef, SpatialPlacementSpec};

pub trait ApplyCreatePlacement: Sized {
    fn apply_create_placement(self, placement: SpatialPlacementSpec) -> Self;
}

impl ApplyCreatePlacement for PrimitiveConstructionIntent {
    fn apply_create_placement(self, placement: SpatialPlacementSpec) -> Self {
        self.with_placement_spec(placement)
    }
}

#[derive(Clone, Debug, PartialEq)]
pub struct CreateSpatialIntent<S> {
    subject: S,
    placement: SpatialPlacementSpec,
}

impl<S> CreateSpatialIntent<S> {
    pub fn new(subject: S) -> Self {
        Self {
            subject,
            placement: SpatialPlacementSpec::world(),
        }
    }

    pub fn at(self, origin: [f64; 3]) -> Self {
        Self {
            placement: self.placement.at(origin),
            ..self
        }
    }

    pub fn between(self, start: [f64; 3], end: [f64; 3]) -> Self {
        Self {
            placement: self.placement.between(start, end),
            ..self
        }
    }

    pub fn facing(self, facing: [f64; 3]) -> Self {
        Self {
            placement: self.placement.facing(facing),
            ..self
        }
    }

    pub fn facing_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            placement: self.placement.facing_witness(direction_witness),
            ..self
        }
    }

    pub fn relative_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.relative_to(frame),
            ..self
        }
    }

    pub fn on(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.on(frame),
            ..self
        }
    }

    pub fn r#in(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.r#in(frame),
            ..self
        }
    }

    pub fn inside(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.inside(frame),
            ..self
        }
    }

    pub fn aligned_with(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.aligned_with(frame),
            ..self
        }
    }

    pub fn parallel_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.parallel_to(frame),
            ..self
        }
    }

    pub fn perpendicular_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            placement: self.placement.perpendicular_to(frame),
            ..self
        }
    }

    pub fn placement_spec(&self) -> SpatialPlacementSpec {
        self.placement.clone()
    }

    pub fn subject(&self) -> &S {
        &self.subject
    }

    pub fn into_parts(self) -> (S, SpatialPlacementSpec) {
        (self.subject, self.placement)
    }
}

impl<S: ApplyCreatePlacement> CreateSpatialIntent<S> {
    pub fn finish(self) -> S {
        self.subject.apply_create_placement(self.placement)
    }
}

impl PrimitiveConstructionIntent {
    pub fn created(self) -> CreateSpatialIntent<Self> {
        CreateSpatialIntent::new(self)
    }
}
