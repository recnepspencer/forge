use crate::runtime::{WorthUiPageHostPlan, WorthUiRuntimeFactId, WorthUiRuntimeHost};

#[derive(Clone, Debug, Eq, PartialEq)]
pub struct WorthUiMosaicPlacementLegalityReceipt {
    page_name: String,
    posture: WorthUiMosaicPlacementLegalityPosture,
    consumed_facts: Vec<WorthUiRuntimeFactId>,
    receipt_digest: u64,
}

#[derive(Clone, Copy, Debug, Eq, PartialEq)]
pub enum WorthUiMosaicPlacementLegalityPosture {
    Admitted,
    Denied,
}

impl WorthUiRuntimeHost {
    pub fn admit_mosaic_placement_for_page(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
    ) -> WorthUiMosaicPlacementLegalityReceipt {
        WorthUiMosaicPlacementLegalityReceipt::new(
            page_host_plan.page_name(),
            WorthUiMosaicPlacementLegalityPosture::Admitted,
            mosaic_legality_facts(page_host_plan),
        )
    }

    pub fn deny_mosaic_placement_for_page(
        &self,
        page_host_plan: &WorthUiPageHostPlan,
    ) -> WorthUiMosaicPlacementLegalityReceipt {
        WorthUiMosaicPlacementLegalityReceipt::new(
            page_host_plan.page_name(),
            WorthUiMosaicPlacementLegalityPosture::Denied,
            mosaic_legality_facts(page_host_plan),
        )
    }
}

impl WorthUiMosaicPlacementLegalityReceipt {
    fn new(
        page_name: impl Into<String>,
        posture: WorthUiMosaicPlacementLegalityPosture,
        consumed_facts: Vec<WorthUiRuntimeFactId>,
    ) -> Self {
        let page_name = page_name.into();
        let mut consumed_facts = consumed_facts;
        consumed_facts.sort();
        consumed_facts.dedup();
        let receipt_digest = digest_parts(
            [
                "mosaic_placement_legality".to_owned(),
                page_name.clone(),
                posture.token().to_owned(),
            ]
            .into_iter()
            .chain(consumed_facts.iter().map(|fact| fact.identity().to_owned())),
        );
        Self {
            page_name,
            posture,
            consumed_facts,
            receipt_digest,
        }
    }

    pub fn page_name(&self) -> &str {
        &self.page_name
    }

    pub fn posture(&self) -> WorthUiMosaicPlacementLegalityPosture {
        self.posture
    }

    pub fn admitted(&self) -> bool {
        self.posture == WorthUiMosaicPlacementLegalityPosture::Admitted
    }

    pub fn consumed_facts(&self) -> &[WorthUiRuntimeFactId] {
        &self.consumed_facts
    }

    pub fn receipt_digest(&self) -> u64 {
        self.receipt_digest
    }
}

impl WorthUiMosaicPlacementLegalityPosture {
    pub const fn token(self) -> &'static str {
        match self {
            Self::Admitted => "admitted",
            Self::Denied => "denied",
        }
    }
}

fn mosaic_legality_facts(page_host_plan: &WorthUiPageHostPlan) -> Vec<WorthUiRuntimeFactId> {
    vec![
        WorthUiRuntimeFactId::mosaic_placement_legality(page_host_plan.page_name()),
        WorthUiRuntimeFactId::layout_topology(page_host_plan.page_name()),
    ]
}

fn digest_parts(parts: impl IntoIterator<Item = String>) -> u64 {
    let mut hash = 0xcbf29ce484222325u64;
    for part in parts {
        for byte in part.as_bytes() {
            hash ^= u64::from(*byte);
            hash = hash.wrapping_mul(0x100000001b3);
        }
        hash ^= 0xff;
        hash = hash.wrapping_mul(0x100000001b3);
    }
    hash
}
