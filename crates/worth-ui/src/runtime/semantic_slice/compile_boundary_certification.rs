use super::{
    WorthUiAdmittedHotReloadableSemanticSliceSet, WorthUiSemanticChangedSliceSet,
    WorthUiSemanticCompileBoundary, WorthUiSemanticSliceId, WorthUiSemanticSliceInventory,
};

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiCompileBoundaryPosture {
    HotReloadWithinProductMeaning,
    CompileRequiredPlatformMeaning,
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiCompileBoundaryCertification {
    changed_slice_ids: Vec<WorthUiSemanticSliceId>,
    hot_reloadable_slice_ids: Vec<WorthUiSemanticSliceId>,
    compile_required_slice_ids: Vec<WorthUiSemanticSliceId>,
    posture: WorthUiCompileBoundaryPosture,
}

impl WorthUiCompileBoundaryCertification {
    pub fn certify(
        boundary: &WorthUiSemanticCompileBoundary,
        changed_slices: &WorthUiSemanticChangedSliceSet,
    ) -> Self {
        let mut changed_slice_ids = changed_slices
            .rows()
            .iter()
            .map(|row| row.descriptor().id())
            .collect::<Vec<_>>();
        changed_slice_ids.sort();
        changed_slice_ids.dedup();

        let compile_required_slice_ids = changed_slice_ids
            .iter()
            .copied()
            .filter(|id| boundary.is_compile_required_platform_slice(*id))
            .collect::<Vec<_>>();
        let hot_reloadable_slice_ids = admitted_hot_reloadable_slice_ids(&changed_slice_ids);
        let posture = if compile_required_slice_ids.is_empty() {
            WorthUiCompileBoundaryPosture::HotReloadWithinProductMeaning
        } else {
            WorthUiCompileBoundaryPosture::CompileRequiredPlatformMeaning
        };
        Self {
            changed_slice_ids,
            hot_reloadable_slice_ids,
            compile_required_slice_ids,
            posture,
        }
    }

    pub fn changed_slice_ids(&self) -> &[WorthUiSemanticSliceId] {
        &self.changed_slice_ids
    }

    pub fn hot_reloadable_slice_ids(&self) -> &[WorthUiSemanticSliceId] {
        &self.hot_reloadable_slice_ids
    }

    pub fn compile_required_slice_ids(&self) -> &[WorthUiSemanticSliceId] {
        &self.compile_required_slice_ids
    }

    pub fn posture(&self) -> WorthUiCompileBoundaryPosture {
        self.posture
    }

    pub fn hot_reload_stays_within_product_meaning(&self) -> bool {
        self.posture == WorthUiCompileBoundaryPosture::HotReloadWithinProductMeaning
    }

    pub fn stable_digest(&self) -> u64 {
        let mut digest = 0xcbf2_9ce4_8422_2325u64;
        for basis in self.digest_bases() {
            for byte in basis.as_bytes() {
                digest ^= u64::from(*byte);
                digest = digest.wrapping_mul(0x0000_0100_0000_01b3);
            }
        }
        digest
    }
}

fn admitted_hot_reloadable_slice_ids(
    changed_slice_ids: &[WorthUiSemanticSliceId],
) -> Vec<WorthUiSemanticSliceId> {
    let inventory = WorthUiSemanticSliceInventory::current();
    let product_ids = changed_slice_ids
        .iter()
        .copied()
        .filter(|id| {
            inventory
                .slice(*id)
                .is_some_and(|descriptor| descriptor.meaning().is_product_meaning())
        })
        .collect::<Vec<_>>();
    WorthUiAdmittedHotReloadableSemanticSliceSet::admit(&inventory, product_ids)
        .expect("product-meaning semantic slices must admit into the hot-reloadable set")
        .slices()
        .iter()
        .map(|slice| slice.descriptor().id())
        .collect()
}

impl WorthUiCompileBoundaryCertification {
    fn digest_bases(&self) -> Vec<String> {
        let mut bases = self
            .changed_slice_ids
            .iter()
            .map(|id| format!("changed:{id:?}"))
            .chain(
                self.hot_reloadable_slice_ids
                    .iter()
                    .map(|id| format!("hot:{id:?}")),
            )
            .chain(
                self.compile_required_slice_ids
                    .iter()
                    .map(|id| format!("compile:{id:?}")),
            )
            .collect::<Vec<_>>();
        bases.push(format!("posture:{:?}", self.posture));
        bases.sort();
        bases
    }
}

trait WorthUiSemanticMeaningClassExt {
    fn is_product_meaning(self) -> bool;
}

impl WorthUiSemanticMeaningClassExt for super::WorthUiSemanticMeaningClass {
    fn is_product_meaning(self) -> bool {
        matches!(self, super::WorthUiSemanticMeaningClass::ProductMeaning)
    }
}
