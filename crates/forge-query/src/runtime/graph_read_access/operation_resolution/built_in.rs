use crate::runtime::ForgeQueryReadBuiltInOperator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct ForgeQueryBuiltInGraphReadOperation {
    operator: ForgeQueryReadBuiltInOperator,
}

impl ForgeQueryBuiltInGraphReadOperation {
    pub fn operator(&self) -> &ForgeQueryReadBuiltInOperator {
        &self.operator
    }

    pub fn operation_label(&self) -> &'static str {
        self.operator.as_str()
    }

    pub(crate) fn admitted(operator: ForgeQueryReadBuiltInOperator) -> Self {
        Self { operator }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("built_in_operation:{}", self.operator.as_str())
    }
}
