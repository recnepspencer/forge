use super::bound_pair::PlanarBooleanPredicateBoundPair;

#[derive(Clone, Copy, Debug)]
pub struct PlanarBooleanEventClassifierInput<'a> {
    bound_pair: &'a PlanarBooleanPredicateBoundPair,
}

impl<'a> PlanarBooleanEventClassifierInput<'a> {
    pub fn from_predicate_bound_pair(bound_pair: &'a PlanarBooleanPredicateBoundPair) -> Self {
        Self { bound_pair }
    }

    pub fn bound_pair(&self) -> &'a PlanarBooleanPredicateBoundPair {
        self.bound_pair
    }

    pub fn segment_pair_identity(&self) -> &str {
        self.bound_pair.segment_pair_identity()
    }

    pub fn predicate_binding_identity(&self) -> &str {
        self.bound_pair.predicate_binding_identity()
    }

    pub fn predicate_bound_pair_identity(&self) -> &str {
        self.bound_pair.bound_pair_identity()
    }
}
