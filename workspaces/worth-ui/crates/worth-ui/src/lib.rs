//! Worth UI public library boundary.

pub mod facade;

/// Builds a typed one- or two-stroke command shortcut without a string parser.
#[macro_export]
macro_rules! shortcut {
    (@stroke [$modifiers:expr] Primary + $($rest:tt)+) => {
        $crate::shortcut!(@stroke [$modifiers.with_primary()] $($rest)+)
    };
    (@stroke [$modifiers:expr] Shift + $($rest:tt)+) => {
        $crate::shortcut!(@stroke [$modifiers.with_shift()] $($rest)+)
    };
    (@stroke [$modifiers:expr] Control + $($rest:tt)+) => {
        $crate::shortcut!(@stroke [$modifiers.with_control()] $($rest)+)
    };
    (@stroke [$modifiers:expr] Alt + $($rest:tt)+) => {
        $crate::shortcut!(@stroke [$modifiers.with_alt()] $($rest)+)
    };
    (@stroke [$modifiers:expr] Meta + $($rest:tt)+) => {
        $crate::shortcut!(@stroke [$modifiers.with_meta()] $($rest)+)
    };
    (@stroke [$modifiers:expr] Physical($key:ident)) => {
        $crate::facade::service::UiCommandShortcutStroke::physical(
            $crate::facade::service::UiCommandKeyCode::$key,
            $modifiers,
        )
    };
    (@stroke [$modifiers:expr] $key:ident) => {
        $crate::facade::service::UiCommandShortcutStroke::logical(
            $crate::facade::service::UiCommandKeyCode::$key,
            $modifiers,
        )
    };
    (($($first:tt)+), ($($second:tt)+)) => {
        $crate::facade::service::UiCommandShortcutSequence::two_stroke(
            $crate::shortcut!(@stroke [$crate::facade::service::UiCommandModifierSet::none()] $($first)+),
            $crate::shortcut!(@stroke [$crate::facade::service::UiCommandModifierSet::none()] $($second)+),
        )
    };
    ($($stroke:tt)+) => {
        $crate::facade::service::UiCommandShortcutSequence::single(
            $crate::shortcut!(@stroke [$crate::facade::service::UiCommandModifierSet::none()] $($stroke)+),
        )
    };
}
