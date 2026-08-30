#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct UiCommandWonInspectionRecord {
    command: String,
    scope: crate::capability::UiCommandRouteScope,
    losers: Box<[(String, super::UiCommandRouteLossReason)]>,
    invocation: u64,
}

impl UiCommandWonInspectionRecord {
    pub(super) fn from_receipt(receipt: &super::UiCommandRouteReceipt, invocation: u64) -> Self {
        Self {
            command: receipt.command().as_str().to_owned(),
            scope: receipt.scope(),
            losers: receipt
                .losers()
                .iter()
                .take(16)
                .map(|loss| (loss.command().as_str().to_owned(), loss.reason()))
                .collect::<Vec<_>>()
                .into_boxed_slice(),
            invocation,
        }
    }

    pub(crate) fn command(&self) -> &str {
        &self.command
    }
    pub(crate) const fn scope(&self) -> crate::capability::UiCommandRouteScope {
        self.scope
    }
    pub(crate) fn losers(&self) -> &[(String, super::UiCommandRouteLossReason)] {
        &self.losers
    }
    pub(crate) const fn invocation(&self) -> u64 {
        self.invocation
    }
}
