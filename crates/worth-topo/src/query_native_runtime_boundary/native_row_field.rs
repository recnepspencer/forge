use forge_foundational::facade::{AspectKey, CanonicalFieldPath, FieldKey};
use forge_query::facade::{
    ForgeQueryAspectMutationBuilder, ForgeQueryAspectTouch, ForgeQueryAuthoredAspectValue,
    ForgeQueryGraphRelationMutationBuilder,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub(crate) enum TopologyNativeQueryRowField {
    TopologyKind,
    TopologyStructure,
    TopologyOwnership,
    TopologyBoundary,
    TopologyRadial,
    TopologySourceIdentity,
    TopologyTargetIdentity,
    NamingPersistentName,
}

impl TopologyNativeQueryRowField {
    pub(crate) fn from_query_aspect_path(path: schema::facade::QueryAspectPath) -> Option<Self> {
        match path {
            schema::facade::QueryAspectPath::TOPOLOGY_STRUCTURE => Some(Self::TopologyStructure),
            schema::facade::QueryAspectPath::TOPOLOGY_OWNERSHIP => Some(Self::TopologyOwnership),
            schema::facade::QueryAspectPath::TOPOLOGY_BOUNDARY => Some(Self::TopologyBoundary),
            schema::facade::QueryAspectPath::TOPOLOGY_RADIAL => Some(Self::TopologyRadial),
            schema::facade::QueryAspectPath::NAMING_PERSISTENT_NAME => {
                Some(Self::NamingPersistentName)
            }
            _ => None,
        }
    }

    pub(crate) fn touch(self) -> ForgeQueryAspectTouch {
        ForgeQueryAspectTouch::aspect_field_path(self.aspect_key(), self.field_path())
    }

    pub(crate) fn authored_string(self, value: impl Into<String>) -> ForgeQueryAuthoredAspectValue {
        ForgeQueryAuthoredAspectValue::string(value)
    }

    pub(crate) fn set_on(
        self,
        builder: ForgeQueryAspectMutationBuilder,
        value: impl Into<String>,
    ) -> ForgeQueryAspectMutationBuilder {
        builder.set_aspect(self.touch(), self.authored_string(value))
    }

    pub(crate) fn set_on_relation(
        self,
        builder: ForgeQueryGraphRelationMutationBuilder,
        value: impl Into<String>,
    ) -> ForgeQueryGraphRelationMutationBuilder {
        builder.set_aspect(self.touch(), self.authored_string(value))
    }

    pub(crate) fn row_segments(self) -> [&'static str; 2] {
        match self {
            Self::TopologyKind => ["topology", "kind"],
            Self::TopologyStructure => ["topology", "structure"],
            Self::TopologyOwnership => ["topology", "ownership"],
            Self::TopologyBoundary => ["topology", "boundary"],
            Self::TopologyRadial => ["topology", "radial"],
            Self::TopologySourceIdentity => ["topology", "source_identity"],
            Self::TopologyTargetIdentity => ["topology", "target_identity"],
            Self::NamingPersistentName => ["naming", "persistent_name"],
        }
    }

    fn aspect_key(self) -> AspectKey {
        AspectKey::new(self.row_segments()[0]).expect("worth query row aspect key must admit")
    }

    fn field_path(self) -> CanonicalFieldPath {
        CanonicalFieldPath::single(
            FieldKey::new(self.row_segments()[1]).expect("worth query row field key must admit"),
        )
    }
}
