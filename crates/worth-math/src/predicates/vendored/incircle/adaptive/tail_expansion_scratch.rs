//! Fixed scratch buffers for exact tail contribution construction.

pub(in crate::predicates::vendored::incircle::adaptive) struct TailExpansionScratch {
    pub(in crate::predicates::vendored::incircle::adaptive) temp8: [f64; 8],
    pub(in crate::predicates::vendored::incircle::adaptive) temp16a: [f64; 16],
    pub(in crate::predicates::vendored::incircle::adaptive) temp16b: [f64; 16],
    pub(in crate::predicates::vendored::incircle::adaptive) temp16c: [f64; 16],
    pub(in crate::predicates::vendored::incircle::adaptive) temp32a: [f64; 32],
    pub(in crate::predicates::vendored::incircle::adaptive) temp32b: [f64; 32],
    pub(in crate::predicates::vendored::incircle::adaptive) temp48: [f64; 48],
    pub(in crate::predicates::vendored::incircle::adaptive) temp64: [f64; 64],
}

impl TailExpansionScratch {
    pub(in crate::predicates::vendored::incircle::adaptive) fn new() -> Self {
        Self {
            temp8: [0.; 8],
            temp16a: [0.; 16],
            temp16b: [0.; 16],
            temp16c: [0.; 16],
            temp32a: [0.; 32],
            temp32b: [0.; 32],
            temp48: [0.; 48],
            temp64: [0.; 64],
        }
    }
}
