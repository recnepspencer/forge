use std::collections::{BTreeMap, BTreeSet};

use super::LayoutOwnerFamily;

#[derive(Debug, Clone, PartialEq, Eq)]
pub struct LayoutOwnerCaseDeclarations {
    families: BTreeMap<LayoutOwnerFamily, BTreeSet<&'static str>>,
}

impl LayoutOwnerCaseDeclarations {
    pub fn from_owner_inventories() -> Self {
        let mut declarations = Self {
            families: BTreeMap::new(),
        };
        super::access::register(&mut declarations);
        super::materialization::register(&mut declarations);
        super::maintenance::register(&mut declarations);
        super::evolution::register(&mut declarations);
        super::integrity::register(&mut declarations);
        super::durable::register(&mut declarations);

        let canonical_families = LayoutOwnerFamily::all();
        let unique_canonical_families = canonical_families.into_iter().collect::<BTreeSet<_>>();
        assert_eq!(
            unique_canonical_families.len(),
            canonical_families.len(),
            "canonical owner family inventory must not contain duplicates"
        );
        assert_eq!(
            declarations
                .families
                .keys()
                .copied()
                .collect::<BTreeSet<_>>(),
            unique_canonical_families,
            "canonical owner families must exactly match registered owner inventories"
        );
        declarations
    }

    pub(super) fn insert(
        &mut self,
        family: LayoutOwnerFamily,
        cases: impl IntoIterator<Item = &'static str>,
    ) {
        let declared_cases = cases.into_iter().collect::<Vec<_>>();
        let cases = declared_cases.iter().copied().collect::<BTreeSet<_>>();
        assert!(
            !cases.is_empty(),
            "owner family must declare at least one case"
        );
        assert_eq!(
            cases.len(),
            declared_cases.len(),
            "owner family inventory must not contain duplicate case identities"
        );
        assert!(
            self.families.insert(family, cases).is_none(),
            "owner family must be aggregated exactly once"
        );
    }

    pub fn cases(&self, family: LayoutOwnerFamily) -> &BTreeSet<&'static str> {
        self.families
            .get(&family)
            .expect("canonical owner family must have declarations")
    }

    pub fn families(&self) -> impl Iterator<Item = LayoutOwnerFamily> + '_ {
        self.families.keys().copied()
    }
}
