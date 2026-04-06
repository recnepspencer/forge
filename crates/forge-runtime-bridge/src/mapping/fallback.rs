#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMappingFallbackClass {
    Entity,
    Aspect,
    Surface,
    EntityAspect,
    EntitySurface,
    AspectSurface,
    EntityAspectSurface,
}
