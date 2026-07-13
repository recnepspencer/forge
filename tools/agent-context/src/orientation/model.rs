pub(crate) struct CrateOrientation {
    pub(crate) crate_name: String,
    pub(crate) relative_path: String,
    pub(crate) constitutional_class: String,
    pub(crate) domain: String,
    pub(crate) exemplar_role: String,
    pub(crate) deferred_routes: Vec<String>,
    pub(crate) allowed_target_bands: Vec<String>,
    pub(crate) facade_exports: Vec<String>,
    pub(crate) owned_modules: Vec<String>,
    pub(crate) machine_fences: Vec<String>,
    pub(crate) skeleton_fence: String,
    pub(crate) machine_constitution: String,
}
