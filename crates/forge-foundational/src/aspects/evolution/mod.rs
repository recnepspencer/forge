mod scalar_evolution;
mod struct_evolution;
mod verdict;

pub use scalar_evolution::{scalar_widens, AspectEvolutionPolicy};
pub use struct_evolution::classify_struct_evolution;
pub use verdict::{AspectEvolutionKind, AspectEvolutionVerdict};
