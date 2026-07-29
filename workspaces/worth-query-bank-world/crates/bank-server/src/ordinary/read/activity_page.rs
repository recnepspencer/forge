use bank_domain::model::AccountId;
use bank_domain::reads::AccountActivityItem;
use worth_query_host::facade::primary_graph::WorthQueryOrdinaryReadVersion;

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
/// Descriptive continuation for one exact account and provider version.
///
/// Callers cannot manufacture a cursor from offsets or version numbers:
///
/// ```compile_fail
/// use bank_server::BankActivityCursor;
///
/// let _ = BankActivityCursor {
///     account: todo!(),
///     version: todo!(),
///     offset: 1,
/// };
/// ```
pub struct BankActivityCursor {
    account: AccountId,
    version: WorthQueryOrdinaryReadVersion,
    offset: usize,
}

impl BankActivityCursor {
    pub(super) const fn new(
        account: AccountId,
        version: WorthQueryOrdinaryReadVersion,
        offset: usize,
    ) -> Self {
        Self {
            account,
            version,
            offset,
        }
    }

    pub const fn account(self) -> AccountId {
        self.account
    }

    pub const fn version(self) -> WorthQueryOrdinaryReadVersion {
        self.version
    }

    pub const fn offset(self) -> usize {
        self.offset
    }
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum BankActivityCursorDenial {
    ForeignAccount,
    StaleVersion {
        expected: WorthQueryOrdinaryReadVersion,
        actual: WorthQueryOrdinaryReadVersion,
    },
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct BankActivityPage {
    entries: Vec<AccountActivityItem>,
    next: Option<BankActivityCursor>,
}

impl BankActivityPage {
    pub(super) fn new(entries: Vec<AccountActivityItem>, next: Option<BankActivityCursor>) -> Self {
        Self { entries, next }
    }

    pub fn entries(&self) -> &[AccountActivityItem] {
        &self.entries
    }

    pub const fn next(&self) -> Option<BankActivityCursor> {
        self.next
    }

    pub fn into_entries(self) -> Vec<AccountActivityItem> {
        self.entries
    }
}

pub(crate) struct BankProjectedActivityPage {
    pub(crate) entries: Vec<AccountActivityItem>,
    pub(crate) next_offset: Option<usize>,
}
