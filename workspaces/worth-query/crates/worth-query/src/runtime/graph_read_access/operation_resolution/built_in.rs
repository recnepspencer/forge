use crate::runtime::WorthQueryReadBuiltInOperator;

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthQueryBuiltInGraphReadOperation {
    operator: WorthQueryReadBuiltInOperator,
}

impl WorthQueryBuiltInGraphReadOperation {
    pub fn operator(&self) -> &WorthQueryReadBuiltInOperator {
        &self.operator
    }

    pub fn operation_label(&self) -> &'static str {
        self.operator.as_str()
    }

    pub(crate) fn admitted(operator: WorthQueryReadBuiltInOperator) -> Self {
        Self { operator }
    }

    pub(crate) fn digest_part(&self) -> String {
        format!("built_in_operation:{}", self.operator.as_str())
    }
}
