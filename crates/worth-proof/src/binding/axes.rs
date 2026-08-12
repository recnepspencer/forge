/// A set of axes that two bindings can be compared across.
///
/// Implement it by hand only if [`crate::binding_axes!`] cannot express the
/// shape; the macro exists because writing the comparison by hand is how an
/// axis goes missing.
pub trait BindingAxes: Sized {
    /// Which axis drifted. One variant per axis so the caller receives the
    /// exact part of the binding that changed.
    type Drift: 'static;

    /// Declared axis names, in comparison order. Used by
    /// [`crate::binding_axis_drift_certification!`] to prove every axis has a
    /// test, so this is not merely documentation.
    const AXIS_NAMES: &'static [&'static str];

    /// Compare every axis, reporting the first that drifted.
    ///
    /// First rather than all, because the order is the owner's declaration
    /// order and collecting the rest would require an allocation this crate
    /// does not make.
    fn compare_axes(&self, other: &Self) -> Result<(), Self::Drift>;
}

/// The facts a capability was issued against.
#[derive(Debug, Clone, PartialEq, Eq)]
pub struct Binding<Axes>
where
    Axes: BindingAxes,
{
    axes: Axes,
}

impl<Axes> Binding<Axes>
where
    Axes: BindingAxes,
{
    pub fn new(axes: Axes) -> Self {
        Self { axes }
    }

    pub fn axes(&self) -> &Axes {
        &self.axes
    }

    pub fn into_axes(self) -> Axes {
        self.axes
    }

    /// Compare every held axis against one presented binding.
    ///
    /// Success is deliberately not a transferable authority token: the axes
    /// type cannot encode which dynamic values agreed. Owners that use this
    /// check keep it inseparable from the continuation it guards.
    pub fn ensure_matches(&self, presented: &Self) -> Result<(), Axes::Drift> {
        self.axes.compare_axes(&presented.axes)
    }
}

#[cfg(test)]
mod tests {
    use super::{Binding, BindingAxes};

    crate::binding_axes! {
        /// Two axes is enough to show the shape; the certification macro is
        /// exercised against a wider one in `tests/`.
        pub struct LaneBinding {
            pub runtime_identity: u64 => RuntimeIdentity,
            pub lane: &'static str => Lane,
        }
        drift pub enum LaneBindingDrift;
    }

    fn base() -> LaneBinding {
        LaneBinding {
            runtime_identity: 1,
            lane: "recovery",
        }
    }

    #[test]
    fn identical_bindings_match_without_minting_reusable_authority() {
        let held = Binding::new(base());
        let presented = Binding::new(base());

        let matched: () = held
            .ensure_matches(&presented)
            .expect("identical bindings agree");

        assert_eq!(matched, ());
        assert_eq!(LaneBinding::AXIS_NAMES.len(), 2);
    }

    #[test]
    fn a_drifted_axis_is_named_rather_than_reported_as_a_bare_mismatch() {
        let held = Binding::new(base());
        let presented = Binding::new(LaneBinding {
            lane: "settlement",
            ..base()
        });

        let drift = held
            .ensure_matches(&presented)
            .expect_err("the lane drifted");

        assert_eq!(drift, LaneBindingDrift::Lane);
        assert_eq!(drift.axis_name(), "lane");
    }
}
