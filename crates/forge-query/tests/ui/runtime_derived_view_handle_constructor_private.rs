use forge_query::facade::{
    ForgeQueryAuthorityLane, ForgeQueryDerivedViewHandle, ForgeQueryNativeRow,
};

fn main() {
    let _forged = ForgeQueryDerivedViewHandle::<ForgeQueryNativeRow> {
        name: "forged.computed".to_string(),
        authority_lane: ForgeQueryAuthorityLane::DerivedRuntimeState,
        marker: std::marker::PhantomData,
    };
}
