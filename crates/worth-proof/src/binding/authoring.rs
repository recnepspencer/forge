//! Declaring a binding and its per-axis drift in one place.

/// Declare a binding, its axes, and the denial kind for each.
///
/// ```
/// worth_proof::binding_axes! {
///     pub struct CapabilityBinding {
///         pub runtime_identity: u64 => RuntimeIdentity,
///         pub lane: &'static str => Lane,
///     }
///     drift pub enum CapabilityBindingDrift;
/// }
///
/// use worth_proof::Binding;
///
/// let issued = Binding::new(CapabilityBinding { runtime_identity: 1, lane: "recovery" });
/// let presented = Binding::new(CapabilityBinding { runtime_identity: 2, lane: "recovery" });
///
/// let drift = issued.ensure_matches(&presented).expect_err("the runtime drifted");
/// assert_eq!(drift, CapabilityBindingDrift::RuntimeIdentity);
/// assert_eq!(drift.axis_name(), "runtime_identity");
/// ```
///
/// **A forgotten axis is unrepresentable.** The field, its drift variant, its
/// comparison, and its diagnostic name are one macro entry. There is no second
/// list from which an axis can be omitted. This preserves the crate's certified
/// zero-normal-dependency contract while providing the same single-declaration
/// guarantee a derive macro would otherwise require a proc-macro crate to own.
///
/// The struct derives `Debug`, `Clone`, `PartialEq`, and `Eq` — do not repeat
/// them in `#[derive(...)]`. Fields are compared with `!=`, so every axis type
/// must be `PartialEq`.
///
/// Field visibility is the caller's choice, but `pub` is usually right: a
/// binding is a bag of facts whose safety comes from *comparison*, not from
/// sealed construction, and `pub` fields let tests build a one-axis twin with
/// struct-update syntax.
#[macro_export]
macro_rules! binding_axes {
    (
        $(#[$meta:meta])*
        $vis:vis struct $name:ident {
            $(
                $(#[$field_meta:meta])*
                $field_vis:vis $field:ident : $field_type:ty => $variant:ident
            ),+ $(,)?
        }
        drift $drift_vis:vis enum $drift:ident $(;)?
    ) => {
        $(#[$meta])*
        #[derive(Debug, Clone, PartialEq, Eq)]
        $vis struct $name {
            $( $(#[$field_meta])* $field_vis $field: $field_type, )+
        }

        /// Which axis of the binding drifted.
        #[derive(Debug, Clone, Copy, PartialEq, Eq)]
        $drift_vis enum $drift {
            $( $variant, )+
        }

        impl $drift {
            /// The declared field name of the axis that drifted.
            pub const fn axis_name(&self) -> &'static str {
                match self {
                    $( Self::$variant => stringify!($field), )+
                }
            }
        }

        impl $crate::BindingAxes for $name {
            type Drift = $drift;

            const AXIS_NAMES: &'static [&'static str] = &[ $( stringify!($field), )+ ];

            fn compare_axes(&self, other: &Self) -> ::core::result::Result<(), Self::Drift> {
                $(
                    if self.$field != other.$field {
                        return ::core::result::Result::Err($drift::$variant);
                    }
                )+
                ::core::result::Result::Ok(())
            }
        }

    };
}

/// Generate the per-axis drift tests for a binding declared by
/// [`binding_axes!`].
///
/// Every axis gets a twin that differs on exactly that axis, and the test
/// asserts the comparison denies it *by that axis*. A twin whose value equals
/// the base fails loudly rather than passing vacuously.
///
/// ```
/// # worth_proof::binding_axes! {
/// #     pub struct CapabilityBinding {
/// #         pub runtime_identity: u64 => RuntimeIdentity,
/// #         pub lane: &'static str => Lane,
/// #     }
/// #     drift pub enum CapabilityBindingDrift;
/// # }
/// # fn main() {}
/// mod certification {
///     use super::{CapabilityBinding, CapabilityBindingDrift};
///
///     worth_proof::binding_axis_drift_certification! {
///         binding: CapabilityBinding,
///         drift: CapabilityBindingDrift,
///         base: CapabilityBinding { runtime_identity: 1, lane: "recovery" },
///         twins: {
///             runtime_identity => RuntimeIdentity = 2,
///             lane => Lane = "settlement",
///         }
///     }
/// }
/// ```
///
/// [`binding_axes!`] already makes a *forgotten* axis a compile error. This
/// macro covers the other half: an axis that is declared and compared but
/// whose comparison does not actually distinguish anything.
#[macro_export]
macro_rules! binding_axis_drift_certification {
    (
        binding: $name:ident,
        drift: $drift:ident,
        base: $base:expr,
        twins: {
            $( $axis:ident => $variant:ident = $twin:expr ),+ $(,)?
        } $(,)?
    ) => {
        #[test]
        fn every_declared_axis_has_a_drift_twin() {
            let covered: &[&'static str] = &[ $( stringify!($axis), )+ ];
            let declared = <$name as $crate::BindingAxes>::AXIS_NAMES;
            let missing: ::std::vec::Vec<&&'static str> = declared
                .iter()
                .filter(|axis| !covered.contains(axis))
                .collect();

            assert!(
                missing.is_empty(),
                "axes declared on {} with no drift twin: {:?}",
                stringify!($name),
                missing,
            );
        }

        #[test]
        fn identical_bindings_agree_on_every_axis() {
            let held = $crate::Binding::new($base);
            let presented = $crate::Binding::new($base);

            assert!(
                held.ensure_matches(&presented).is_ok(),
                "{} does not match itself; the comparison denies something that did not drift",
                stringify!($name),
            );
        }

        #[test]
        fn drifting_one_axis_is_denied_by_that_axis() {
            $(
                {
                    let held = $crate::Binding::new($base);
                    let presented = $crate::Binding::new($name { $axis: $twin, ..$base });

                    match held.ensure_matches(&presented) {
                        ::core::result::Result::Ok(_) => panic!(
                            "twin for axis `{}` did not drift; its value equals the base, so \
                             this axis is untested",
                            stringify!($axis),
                        ),
                        ::core::result::Result::Err(drift) => assert_eq!(
                            drift,
                            $drift::$variant,
                            "axis `{}` drifted but was denied as `{}`",
                            stringify!($axis),
                            drift.axis_name(),
                        ),
                    }
                }
            )+
        }
    };
}
