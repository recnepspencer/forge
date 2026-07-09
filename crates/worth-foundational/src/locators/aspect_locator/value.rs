use super::{AspectFieldLocator, AspectLocator};

#[derive(Debug, Clone, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum AspectValueLocator {
    WholeAspect(AspectLocator),
    StructField(AspectFieldLocator),
}

impl AspectValueLocator {
    pub fn whole_aspect(aspect: AspectLocator) -> Self {
        Self::WholeAspect(aspect)
    }

    pub fn struct_field(field: AspectFieldLocator) -> Self {
        Self::StructField(field)
    }
}
