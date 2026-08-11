mod authoritative_truth;
mod model;
mod world;

use self::model::observe;
use self::world::mixed_effect_world;
use super::prepare_provider_attempt;

#[test]
fn mixed_effects_lower_to_the_exact_independent_semantic_model() {
    let world = mixed_effect_world();
    let prepared = prepare_provider_attempt(
        world.facts,
        world.effects,
        world.retained_bytes,
        world.retained_bytes,
        None,
    )
    .expect("complete mixed effect basis should lower");

    assert_eq!(observe(prepared), world.expected);
}

#[test]
fn alternate_effect_insertion_preserves_each_exact_association_and_order() {
    let world = mixed_effect_world();
    let prepared = prepare_provider_attempt(
        world.facts,
        world.alternate_effects,
        world.retained_bytes,
        world.retained_bytes,
        None,
    )
    .expect("complete mixed effect basis should lower");
    assert_eq!(observe(prepared), world.alternate_expected);
}
