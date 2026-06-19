use crate::runtime::ForgeQuerySymbolicTargetReference;
use forge_relational::facade::identity::KindId;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphEntitySymbol {
    reference: ForgeQuerySymbolicTargetReference,
}

impl ForgeQueryGraphEntitySymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub(crate) fn new(reference: ForgeQuerySymbolicTargetReference) -> Self {
        Self { reference }
    }

    pub(crate) fn reference(&self) -> ForgeQuerySymbolicTargetReference {
        self.reference.clone()
    }
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryGraphRelationSymbol {
    reference: ForgeQuerySymbolicTargetReference,
    relation_kind_id: Option<KindId>,
}

impl ForgeQueryGraphRelationSymbol {
    pub fn symbol(&self) -> &str {
        self.reference.symbol()
    }

    pub fn relation_kind_id(&self) -> Option<KindId> {
        self.relation_kind_id
    }

    pub(crate) fn new(
        reference: ForgeQuerySymbolicTargetReference,
        relation_kind_id: Option<KindId>,
    ) -> Self {
        Self {
            reference,
            relation_kind_id,
        }
    }

    pub(crate) fn reference(&self) -> ForgeQuerySymbolicTargetReference {
        self.reference.clone()
    }
}
