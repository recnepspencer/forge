#[derive(Clone, Debug, PartialEq)]
pub enum SpatialFrameRef {
    World,
    ShapeLocal,
    Workplane {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
    FeatureLocal {
        name: String,
        origin: [f64; 3],
        normal: [f64; 3],
    },
}

impl SpatialFrameRef {
    pub fn world() -> Self {
        Self::World
    }

    pub fn shape_local() -> Self {
        Self::ShapeLocal
    }

    pub fn workplane(name: impl Into<String>, origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self::Workplane {
            name: name.into(),
            origin,
            normal,
        }
    }

    pub fn feature_local(name: impl Into<String>, origin: [f64; 3], normal: [f64; 3]) -> Self {
        Self::FeatureLocal {
            name: name.into(),
            origin,
            normal,
        }
    }
}
