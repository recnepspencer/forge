use worth_math::arithmetic::Rational;

#[derive(Clone, Debug, Eq, PartialEq)]
pub(crate) struct LocalScaleAreaThresholds {
    zero_area: Rational,
    sliver_area: Rational,
    tiny_hole_area: Rational,
}

impl LocalScaleAreaThresholds {
    pub(crate) fn from_normalization_scale(scale: f64) -> Result<Self, ()> {
        if !scale.is_finite() || scale <= 0.0 {
            return Err(());
        }
        let unit = Rational::try_from_f64(scale).map_err(|_| ())?;
        let unit_area = &unit * &unit;
        Ok(Self {
            zero_area: Rational::zero(),
            sliver_area: unit_area.clone(),
            tiny_hole_area: &unit_area * &Rational::try_from_fraction(100, 1).map_err(|_| ())?,
        })
    }

    pub(crate) fn zero_area(&self) -> &Rational {
        &self.zero_area
    }

    pub(crate) fn sliver_area(&self) -> &Rational {
        &self.sliver_area
    }

    pub(crate) fn tiny_hole_area(&self) -> &Rational {
        &self.tiny_hole_area
    }
}
