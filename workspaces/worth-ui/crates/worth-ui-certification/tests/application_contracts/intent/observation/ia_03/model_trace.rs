use worth_ui::facade::interaction::{
    UiHostInteractionIngressOutcome, UI_ACTIVE_POINTER_GESTURE_LIMIT,
};
use worth_ui::facade::observation_report::UiHostPointerButtonTransition;

use super::super::super::interaction_world::InteractionWorld;
use super::super::assertions::actual_verdict;
use super::super::model::{IndependentGestureModel, ModelEvent, ModelTarget};

#[test]
fn ordered_pointer_trace_agrees_with_independent_model() {
    let mut world = InteractionWorld::canonical();
    let mut model = IndependentGestureModel::new(UI_ACTIVE_POINTER_GESTURE_LIMIT);
    for event in hostile_pointer_trace() {
        let outcome = dispatch(&mut world, event);
        assert_eq!(actual_verdict(&outcome), model.step(event), "{event:?}");
    }
    assert_eq!(
        world
            .session
            .interaction_state()
            .counters()
            .semantic_interactions(),
        2
    );
    let shutdown = world.session.shutdown();
    assert_eq!(
        shutdown
            .interaction()
            .final_state()
            .unwrap()
            .active_gestures(),
        0
    );
}

fn hostile_pointer_trace() -> [ModelEvent; 10] {
    [
        ModelEvent::Press {
            pointer: 1,
            capture: 1,
            target: ModelTarget::Front,
        },
        ModelEvent::Motion {
            pointer: 1,
            capture: 1,
        },
        ModelEvent::Motion {
            pointer: 1,
            capture: 1,
        },
        ModelEvent::Release {
            pointer: 1,
            capture: 1,
            target: ModelTarget::Front,
        },
        ModelEvent::Press {
            pointer: 2,
            capture: 1,
            target: ModelTarget::Front,
        },
        ModelEvent::Motion {
            pointer: 2,
            capture: 2,
        },
        ModelEvent::Press {
            pointer: 3,
            capture: 1,
            target: ModelTarget::Front,
        },
        ModelEvent::Release {
            pointer: 3,
            capture: 1,
            target: ModelTarget::Outer,
        },
        ModelEvent::Press {
            pointer: 4,
            capture: 1,
            target: ModelTarget::Front,
        },
        ModelEvent::Release {
            pointer: 4,
            capture: 1,
            target: ModelTarget::Front,
        },
    ]
}

#[test]
fn pointer_capacity_minus_at_plus_matches_independent_model_and_settles() {
    let mut world = InteractionWorld::canonical();
    let mut model = IndependentGestureModel::new(UI_ACTIVE_POINTER_GESTURE_LIMIT);
    for pointer in 1..=UI_ACTIVE_POINTER_GESTURE_LIMIT + 1 {
        let event = ModelEvent::Press {
            pointer: pointer as u64,
            capture: 1,
            target: ModelTarget::Front,
        };
        let outcome = dispatch(&mut world, event);
        assert_eq!(actual_verdict(&outcome), model.step(event));
    }

    let expected_shutdown = model.settle_all();
    let shutdown = world.session.shutdown();
    let settlement = shutdown.interaction().settlement().unwrap();
    assert_eq!(settlement.stops().len(), expected_shutdown.stops);
    assert_eq!(settlement.final_state().active_gestures(), 0);
}

fn dispatch(world: &mut InteractionWorld, event: ModelEvent) -> UiHostInteractionIngressOutcome {
    match event {
        ModelEvent::Press {
            pointer,
            capture,
            target,
        } => world.button(
            pointer,
            capture,
            UiHostPointerButtonTransition::Pressed,
            point(target),
        ),
        ModelEvent::Release {
            pointer,
            capture,
            target,
        } => world.button(
            pointer,
            capture,
            UiHostPointerButtonTransition::Released,
            point(target),
        ),
        ModelEvent::Motion { pointer, capture } => world.motion(pointer, capture, [159, 95]),
        ModelEvent::FocusLoss => world.focus_loss(),
    }
}

fn point(target: ModelTarget) -> [i64; 2] {
    match target {
        ModelTarget::Front => [20, 20],
        ModelTarget::Outer => [10, 20],
        ModelTarget::None => [159, 95],
    }
}
