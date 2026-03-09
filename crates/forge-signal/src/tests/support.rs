use crate::facade::{Aspect, AspectMask, AspectVersion};
#[allow(unused_imports)]
pub(crate) use crate::tests::legacy_callback_adapter::{
    evaluate, evaluate_on_demand, evaluate_with_resolver, evaluate_with_resolvers,
};

pub const ASPECT_A: Aspect = Aspect::new(0);
pub const ASPECT_B: Aspect = Aspect::new(1);

pub fn mask_a() -> AspectMask {
    AspectMask::from_aspect(ASPECT_A)
}

pub fn mask_b() -> AspectMask {
    AspectMask::from_aspect(ASPECT_B)
}

pub fn version_ab(a: u64, b: u64) -> AspectVersion {
    AspectVersion::from_updates([(ASPECT_A, a), (ASPECT_B, b)])
}
