use crate::runtime::ForgeQuerySymbolicTargetReference;

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
}

impl ForgeQueryGraphRelationSymbol {
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
