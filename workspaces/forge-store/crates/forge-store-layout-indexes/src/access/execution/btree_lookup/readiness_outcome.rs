use super::{BTreeLookupReady, StaleBTreeLookup};

macro_rules! define_btree_lookup_readiness_cases {
    ($( $variant:ident($payload:ty) => $name:literal ),+ $(,)?) => {
        #[derive(Debug, PartialEq, Eq)]
        enum BTreeLookupReadinessCase {
            $( $variant($payload), )+
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
        pub struct BTreeLookupReadinessCaseId(&'static str);

        impl BTreeLookupReadinessCaseId {
            pub const fn name(self) -> &'static str {
                self.0
            }
        }

        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        pub enum BTreeLookupReadinessView<'a> {
            $( $variant(&'a $payload), )+
        }

        impl BTreeLookupReadinessCase {
            const fn id(&self) -> BTreeLookupReadinessCaseId {
                match self {
                    $( Self::$variant(_) => BTreeLookupReadinessCaseId($name), )+
                }
            }

            const fn view(&self) -> BTreeLookupReadinessView<'_> {
                match self {
                    $( Self::$variant(value) => BTreeLookupReadinessView::$variant(value), )+
                }
            }
        }

        pub fn btree_lookup_readiness_cases(
        ) -> impl Iterator<Item = BTreeLookupReadinessCaseId> {
            [$( BTreeLookupReadinessCaseId($name), )+].into_iter()
        }
    };
}

define_btree_lookup_readiness_cases!(
    Ready(BTreeLookupReady) => "layout.btree_lookup.readiness.ready",
    Stale(StaleBTreeLookup) => "layout.btree_lookup.readiness.stale",
);

#[derive(Debug, PartialEq, Eq)]
pub struct BTreeLookupReadinessOutcome {
    case: BTreeLookupReadinessCase,
}

impl BTreeLookupReadinessOutcome {
    pub(super) fn ready(value: BTreeLookupReady) -> Self {
        Self {
            case: BTreeLookupReadinessCase::Ready(value),
        }
    }

    pub(super) fn stale(value: StaleBTreeLookup) -> Self {
        Self {
            case: BTreeLookupReadinessCase::Stale(value),
        }
    }

    pub fn view(&self) -> BTreeLookupReadinessView<'_> {
        self.case.view()
    }

    pub const fn case_id(&self) -> BTreeLookupReadinessCaseId {
        self.case.id()
    }

    pub fn into_ready(self) -> Result<BTreeLookupReady, Self> {
        match self.case {
            BTreeLookupReadinessCase::Ready(ready) => Ok(ready),
            _ => Err(self),
        }
    }
}
