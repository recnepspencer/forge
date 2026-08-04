//! Fixed-buffer final expansion ownership.

use super::super::super::expansion::fast_expansion_sum_zeroelim;
use super::translated_geometry::LiftedBaseDeterminants;

pub(in crate::predicates::vendored::incircle::adaptive) struct FinalExpansion {
    first: [f64; 1152],
    second: [f64; 1152],
    length: usize,
    current_first: bool,
}

impl FinalExpansion {
    pub(in crate::predicates::vendored::incircle::adaptive) fn from_initial(
        lifted: &LiftedBaseDeterminants,
    ) -> Self {
        let mut abdet = [0.; 64];
        let ablen = fast_expansion_sum_zeroelim(
            &lifted.adet[..lifted.alen],
            &lifted.bdet[..lifted.blen],
            &mut abdet,
        );
        let mut first = [0.; 1152];
        let length =
            fast_expansion_sum_zeroelim(&abdet[..ablen], &lifted.cdet[..lifted.clen], &mut first);
        Self {
            first,
            second: [0.; 1152],
            length,
            current_first: true,
        }
    }

    pub(in crate::predicates::vendored::incircle::adaptive) fn append_expansion(
        &mut self,
        term: &[f64],
    ) {
        if self.current_first {
            self.length =
                fast_expansion_sum_zeroelim(&self.first[..self.length], term, &mut self.second);
        } else {
            self.length =
                fast_expansion_sum_zeroelim(&self.second[..self.length], term, &mut self.first);
        }
        self.current_first = !self.current_first;
    }

    pub(in crate::predicates::vendored::incircle::adaptive) fn initial_sum(&self) -> f64 {
        self.first[..self.length].iter().sum()
    }

    pub(in crate::predicates::vendored::incircle::adaptive) fn highest_component(&self) -> f64 {
        if self.current_first {
            self.first[self.length - 1]
        } else {
            self.second[self.length - 1]
        }
    }
}
