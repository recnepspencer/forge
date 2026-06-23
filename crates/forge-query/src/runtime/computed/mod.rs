use std::collections::{BTreeMap, BTreeSet};
use std::marker::PhantomData;

use crate::memory_workspace::{
    ForgeQueryMutationDelta, ForgeQueryMutationKind, ForgeQueryMutationReceipt,
};
use crate::program::ForgeQueryDerivedView;

use super::ForgeQueryAuthorityLane;

mod refresh_context;
mod routing;
mod state;
mod surface;

pub use refresh_context::{ForgeQueryRetainedRefreshContext, ForgeQueryRetainedRefreshOrigin};
pub(in crate::runtime) use routing::{
    admit_derived_view_declaration, insert_derived_runtime,
    retained_live_view_names_for_candidates, route_derived_view_patches,
};
pub(in crate::runtime) use state::{
    ForgeQueryComputedAdmissionError, ForgeQueryComputedDependencyIndex,
    ForgeQueryDerivedViewRuntime,
};
pub use surface::{
    ForgeQueryComputedInspectionEvidence, ForgeQueryDerivedPatch, ForgeQueryDerivedPatchFamily,
    ForgeQueryDerivedPatchPayload, ForgeQueryDerivedViewHandle, ForgeQueryDerivedViewMaintainer,
    ForgeQueryDerivedViewMaterialization, ForgeQueryRetainedUpstreamInputs,
};
