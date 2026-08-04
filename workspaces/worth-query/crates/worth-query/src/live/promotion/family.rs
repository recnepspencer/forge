#[derive(Clone, Debug, Eq, PartialEq, Ord, PartialOrd, Hash)]
pub enum LiveQueryFamily {
    Detail,
    OrderedCollection,
    BoundedMaterialization,
}

impl LiveQueryFamily {
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::Detail => "detail",
            Self::OrderedCollection => "ordered_collection",
            Self::BoundedMaterialization => "bounded_materialization",
        }
    }
}
