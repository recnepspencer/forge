use crate::authored_refs::{SpatialAxis, SpatialDirectionWitnessRef, SpatialFrameRef};

#[derive(Clone, Debug, PartialEq)]
pub struct SpatialPlacementSpec {
    origin: [f64; 3],
    direction_witness: SpatialDirectionWitnessRef,
    reference_frame: SpatialFrameRef,
}

impl SpatialPlacementSpec {
    pub fn world() -> Self {
        Self {
            origin: [0.0, 0.0, 0.0],
            direction_witness: SpatialDirectionWitnessRef::world_direction([0.0, 0.0, 1.0]),
            reference_frame: SpatialFrameRef::world(),
        }
    }

    pub fn at(self, origin: [f64; 3]) -> Self {
        Self { origin, ..self }
    }

    pub fn facing(self, facing: [f64; 3]) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::world_direction(facing))
    }

    pub fn facing_witness(self, direction_witness: SpatialDirectionWitnessRef) -> Self {
        Self {
            direction_witness,
            ..self
        }
    }

    pub fn between(self, start: [f64; 3], end: [f64; 3]) -> Self {
        self.at([
            (start[0] + end[0]) * 0.5,
            (start[1] + end[1]) * 0.5,
            (start[2] + end[2]) * 0.5,
        ])
    }

    pub fn relative_to(self, frame: SpatialFrameRef) -> Self {
        Self {
            reference_frame: frame,
            ..self
        }
    }

    pub fn on(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame.clone()).aligned_with(frame)
    }

    pub fn r#in(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame)
    }

    pub fn inside(self, frame: SpatialFrameRef) -> Self {
        self.relative_to(frame)
    }

    pub fn aligned_with(self, frame: SpatialFrameRef) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::frame_axis(
            frame.clone(),
            SpatialAxis::W,
        ))
        .relative_to(frame)
    }

    pub fn parallel_to(self, frame: SpatialFrameRef) -> Self {
        self.aligned_with(frame)
    }

    pub fn perpendicular_to(self, frame: SpatialFrameRef) -> Self {
        self.facing_witness(SpatialDirectionWitnessRef::frame_perpendicular_axis(
            frame.clone(),
            SpatialAxis::W,
        ))
        .relative_to(frame)
    }

    pub fn origin(&self) -> [f64; 3] {
        self.origin
    }

    pub fn direction_witness(&self) -> &SpatialDirectionWitnessRef {
        &self.direction_witness
    }

    pub fn reference_frame(&self) -> &SpatialFrameRef {
        &self.reference_frame
    }
}

impl Default for SpatialPlacementSpec {
    fn default() -> Self {
        Self::world()
    }
}
