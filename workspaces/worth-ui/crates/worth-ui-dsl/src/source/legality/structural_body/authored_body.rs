#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiAuthoredStructuralBody {
    root_regions: Vec<WorthUiAuthoredRegion>,
    projection_contents: Vec<WorthUiAuthoredProjectionContent>,
    interaction_routes: Vec<crate::WorthUiIntentInteractionRoute>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiAuthoredProjectionContent {
    projection_identity_text: String,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiAuthoredRegion {
    region_id_text: String,
    sizing_contract_id_text: Option<String>,
    state_slot_id_text: Option<String>,
    child_regions: Vec<WorthUiAuthoredRegion>,
    mounts: Vec<WorthUiAuthoredMount>,
}

#[derive(Clone, Debug, PartialEq, Eq)]
pub struct WorthUiAuthoredMount {
    surface_id_text: String,
    placement_policy_id_text: Option<String>,
    state_slot_id_text: Option<String>,
}

impl WorthUiAuthoredStructuralBody {
    pub(super) fn new(
        root_regions: Vec<WorthUiAuthoredRegion>,
        projection_contents: Vec<WorthUiAuthoredProjectionContent>,
        interaction_routes: Vec<crate::WorthUiIntentInteractionRoute>,
    ) -> Self {
        Self {
            root_regions,
            projection_contents,
            interaction_routes,
        }
    }

    pub fn root_regions(&self) -> &[WorthUiAuthoredRegion] {
        &self.root_regions
    }

    pub fn projection_contents(&self) -> &[WorthUiAuthoredProjectionContent] {
        &self.projection_contents
    }

    pub fn interaction_routes(&self) -> &[crate::WorthUiIntentInteractionRoute] {
        &self.interaction_routes
    }
}

impl WorthUiAuthoredProjectionContent {
    pub(super) fn new(projection_identity_text: String) -> Self {
        Self {
            projection_identity_text,
        }
    }

    pub fn projection_identity_text(&self) -> &str {
        &self.projection_identity_text
    }
}

impl WorthUiAuthoredRegion {
    pub(super) fn new(
        region_id_text: String,
        sizing_contract_id_text: Option<String>,
        state_slot_id_text: Option<String>,
        child_regions: Vec<Self>,
        mounts: Vec<WorthUiAuthoredMount>,
    ) -> Self {
        Self {
            region_id_text,
            sizing_contract_id_text,
            state_slot_id_text,
            child_regions,
            mounts,
        }
    }

    pub fn region_id_text(&self) -> &str {
        &self.region_id_text
    }

    pub fn sizing_contract_id_text(&self) -> Option<&str> {
        self.sizing_contract_id_text.as_deref()
    }

    pub fn state_slot_id_text(&self) -> Option<&str> {
        self.state_slot_id_text.as_deref()
    }

    pub fn child_regions(&self) -> &[WorthUiAuthoredRegion] {
        &self.child_regions
    }

    pub fn mounts(&self) -> &[WorthUiAuthoredMount] {
        &self.mounts
    }
}

impl WorthUiAuthoredMount {
    pub(super) fn new(
        surface_id_text: String,
        placement_policy_id_text: Option<String>,
        state_slot_id_text: Option<String>,
    ) -> Self {
        Self {
            surface_id_text,
            placement_policy_id_text,
            state_slot_id_text,
        }
    }

    pub fn surface_id_text(&self) -> &str {
        &self.surface_id_text
    }

    pub fn placement_policy_id_text(&self) -> Option<&str> {
        self.placement_policy_id_text.as_deref()
    }

    pub fn state_slot_id_text(&self) -> Option<&str> {
        self.state_slot_id_text.as_deref()
    }
}
