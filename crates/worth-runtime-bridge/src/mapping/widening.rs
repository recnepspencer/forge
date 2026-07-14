#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub enum BridgeMappingWideningClass {
    Entity,
    Aspect,
    Surface,
    EntityAspect,
    EntitySurface,
    AspectSurface,
    EntityAspectSurface,
}
