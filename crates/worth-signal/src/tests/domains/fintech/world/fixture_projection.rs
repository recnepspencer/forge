use super::super::regimes::MarketRegime;
use super::FinancialWorldDefinition;

const PRICE_SCALE: i64 = 10_000;
const FX_SCALE: i64 = 10_000;

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FixtureMarketPoint {
    pub(in crate::tests::domains::fintech) price: u64,
    pub(in crate::tests::domains::fintech) vol: u64,
    pub(in crate::tests::domains::fintech) curve: u64,
    pub(in crate::tests::domains::fintech) liquidity: u64,
    pub(in crate::tests::domains::fintech) risk: u64,
    pub(in crate::tests::domains::fintech) alert: u64,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FixtureFxMarket {
    pub(in crate::tests::domains::fintech) eur_usd: u64,
    pub(in crate::tests::domains::fintech) usd_jpy: u64,
    pub(in crate::tests::domains::fintech) eur_jpy: u64,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FixtureScenarioShock {
    pub(in crate::tests::domains::fintech) risk: u64,
    pub(in crate::tests::domains::fintech) vol: u64,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FixtureAggregateState {
    pub(in crate::tests::domains::fintech) risk: u64,
    pub(in crate::tests::domains::fintech) alert: u64,
}

#[derive(Clone, Copy, Debug)]
struct ProbabilityWindow {
    center: i64,
    width: i64,
    weight: u64,
}

#[derive(Clone, Copy, Debug)]
struct RegimeShape {
    price_windows: [ProbabilityWindow; 3],
    vol_windows: [ProbabilityWindow; 3],
    liquidity_windows: [ProbabilityWindow; 3],
    curve_windows: [ProbabilityWindow; 3],
    fx_windows: [ProbabilityWindow; 3],
    alert_bias: i64,
}

#[derive(Clone, Copy, Debug)]
pub(in crate::tests::domains::fintech) struct FinancialFixtureProjection {
    seed: u64,
    shape: RegimeShape,
}

impl FinancialFixtureProjection {
    pub(in crate::tests::domains::fintech) fn from_definition(
        definition: &FinancialWorldDefinition,
    ) -> Self {
        Self {
            seed: definition.seed(),
            shape: regime_shape(definition.fixture_regime()),
        }
    }

    pub(in crate::tests::domains::fintech) fn market_point(
        self,
        instrument_index: usize,
        book_index: usize,
    ) -> FixtureMarketPoint {
        let instrument = instrument_index as u64;
        let book = book_index as u64;
        let global_pressure = signed_jitter(self.seed, 0xA11CE, 140);
        let book_pressure = signed_jitter(self.seed ^ book, 0xB00C, 90);
        let instrument_pressure = signed_jitter(self.seed ^ instrument, 0xC0FFEE, 70);
        let price_shock = sample_window(
            self.seed ^ instrument ^ (book << 8),
            0x1001,
            &self.shape.price_windows,
        );
        let vol_shock = sample_window(
            self.seed ^ instrument.rotate_left(7),
            0x1002,
            &self.shape.vol_windows,
        );
        let liquidity_shock = sample_window(
            self.seed ^ instrument.rotate_left(13),
            0x1003,
            &self.shape.liquidity_windows,
        );
        let curve_shock = sample_window(
            self.seed ^ instrument.rotate_left(17),
            0x1004,
            &self.shape.curve_windows,
        );

        let price = clamp_u64(
            PRICE_SCALE
                + 12 * price_shock
                + 6 * global_pressure
                + 4 * book_pressure
                + instrument_pressure,
        );
        let vol = clamp_u64(1_200 + 10 * vol_shock + 5 * global_pressure + 3 * price_shock);
        let curve = clamp_u64(2_000 + 9 * curve_shock + 4 * global_pressure + 2 * book_pressure);
        let liquidity = clamp_u64(
            350 + 8 * liquidity_shock + 2 * global_pressure + 3 * vol_shock.abs() - price_shock,
        );
        let risk = clamp_u64(
            3_000
                + 14 * price_shock.abs()
                + 11 * vol_shock.abs()
                + 7 * curve_shock.abs()
                + 5 * liquidity_shock.abs()
                + 3 * global_pressure.abs(),
        );
        let alert = u64::from(
            risk as i64 + self.shape.alert_bias + 3 * global_pressure + 2 * vol_shock > 4_100,
        );

        FixtureMarketPoint {
            price,
            vol,
            curve,
            liquidity,
            risk,
            alert,
        }
    }

    pub(in crate::tests::domains::fintech) fn fx_market(self) -> FixtureFxMarket {
        let global_shift = sample_window(self.seed, 0x2001, &self.shape.fx_windows);
        let dislocation = signed_jitter(self.seed, 0x2002, 80);
        let eur_usd = clamp_u64(FX_SCALE + 18 * global_shift + dislocation);
        let usd_jpy = clamp_u64(14_000 - 11 * global_shift + 2 * dislocation);
        let eur_jpy = clamp_u64((eur_usd as i64 * usd_jpy as i64) / FX_SCALE);

        FixtureFxMarket {
            eur_usd,
            usd_jpy,
            eur_jpy,
        }
    }

    pub(in crate::tests::domains::fintech) fn curve_bucket_series(
        self,
        buckets: usize,
    ) -> Vec<u64> {
        bucket_series(
            self.seed,
            0x3001,
            buckets,
            1_500 + sample_window(self.seed, 0x3002, &self.shape.curve_windows) * 9,
            22 + signed_jitter(self.seed, 0x3003, 7),
            6 + signed_jitter(self.seed, 0x3004, 3),
        )
    }

    pub(in crate::tests::domains::fintech) fn vol_surface_series(self, buckets: usize) -> Vec<u64> {
        bucket_series(
            self.seed,
            0x4001,
            buckets,
            900 + sample_window(self.seed, 0x4002, &self.shape.vol_windows) * 8,
            15 + signed_jitter(self.seed, 0x4003, 5),
            4 + signed_jitter(self.seed, 0x4004, 2),
        )
    }

    pub(super) fn regime_window_signature(self) -> (u64, u64, u64) {
        let calm_point = self.market_point(0, 0);
        let fx = self.fx_market();
        (calm_point.price, calm_point.vol, fx.eur_jpy)
    }

    pub(in crate::tests::domains::fintech) fn scenario_shocks(
        self,
        scenarios: usize,
    ) -> Vec<FixtureScenarioShock> {
        let mut shocks = Vec::with_capacity(scenarios);
        for scenario in 0..scenarios {
            let scenario_seed = self.seed ^ ((scenario as u64 + 1) * 0x9e37);
            let risk = clamp_u64(
                700 + 17 * sample_window(scenario_seed, 0x5001, &self.shape.vol_windows).abs()
                    + 11 * sample_window(scenario_seed, 0x5002, &self.shape.fx_windows).abs(),
            );
            let vol = clamp_u64(
                400 + 13 * sample_window(scenario_seed, 0x5003, &self.shape.vol_windows).abs(),
            );
            shocks.push(FixtureScenarioShock { risk, vol });
        }
        shocks
    }

    pub(in crate::tests::domains::fintech) fn book_states(
        self,
        books: usize,
    ) -> Vec<FixtureAggregateState> {
        let mut states = Vec::with_capacity(books);
        for book in 0..books {
            let seed = self.seed ^ ((book as u64 + 11) * 0x45d9);
            let risk = clamp_u64(
                1_400
                    + 15 * sample_window(seed, 0x6001, &self.shape.price_windows).abs()
                    + 9 * sample_window(seed, 0x6002, &self.shape.liquidity_windows).abs(),
            );
            let alert = u64::from(
                risk as i64 + sample_window(seed, 0x6003, &self.shape.vol_windows) * 8 > 1_850,
            );
            states.push(FixtureAggregateState { risk, alert });
        }
        states
    }

    pub(in crate::tests::domains::fintech) fn desk_limits(
        self,
        desks: usize,
    ) -> Vec<FixtureAggregateState> {
        let mut states = Vec::with_capacity(desks);
        for desk in 0..desks {
            let seed = self.seed ^ ((desk as u64 + 23) * 0x27d4);
            let risk = clamp_u64(
                2_100
                    + 12 * sample_window(seed, 0x7001, &self.shape.curve_windows).abs()
                    + 7 * sample_window(seed, 0x7002, &self.shape.fx_windows).abs(),
            );
            let alert = u64::from(
                risk as i64 + sample_window(seed, 0x7003, &self.shape.price_windows) * 10 > 2_450,
            );
            states.push(FixtureAggregateState { risk, alert });
        }
        states
    }
}

fn regime_shape(regime: MarketRegime) -> RegimeShape {
    match regime {
        MarketRegime::Calm => RegimeShape {
            price_windows: windows([(-4, 5, 5), (0, 4, 14), (5, 5, 5)]),
            vol_windows: windows([(-2, 3, 4), (0, 2, 16), (3, 4, 4)]),
            liquidity_windows: windows([(-1, 2, 3), (0, 2, 18), (2, 3, 3)]),
            curve_windows: windows([(-2, 3, 4), (0, 2, 14), (2, 3, 6)]),
            fx_windows: windows([(-3, 4, 4), (0, 3, 14), (3, 4, 4)]),
            alert_bias: -180,
        },
        MarketRegime::HighVol => RegimeShape {
            price_windows: windows([(-7, 6, 5), (0, 5, 10), (8, 7, 9)]),
            vol_windows: windows([(6, 5, 6), (11, 5, 10), (16, 6, 6)]),
            liquidity_windows: windows([(1, 3, 5), (4, 4, 10), (7, 4, 5)]),
            curve_windows: windows([(0, 3, 6), (4, 4, 10), (8, 4, 4)]),
            fx_windows: windows([(-5, 5, 5), (1, 4, 10), (8, 6, 5)]),
            alert_bias: 260,
        },
        MarketRegime::SpreadBlowout => RegimeShape {
            price_windows: windows([(-9, 7, 6), (-2, 5, 8), (4, 5, 6)]),
            vol_windows: windows([(4, 4, 5), (8, 5, 10), (13, 5, 5)]),
            liquidity_windows: windows([(7, 4, 6), (13, 5, 10), (20, 6, 4)]),
            curve_windows: windows([(-1, 3, 6), (2, 4, 10), (6, 4, 4)]),
            fx_windows: windows([(-6, 5, 6), (-1, 4, 8), (5, 5, 6)]),
            alert_bias: 340,
        },
        MarketRegime::CurveShock => RegimeShape {
            price_windows: windows([(-3, 4, 5), (2, 4, 10), (7, 5, 5)]),
            vol_windows: windows([(1, 3, 5), (5, 4, 10), (9, 4, 5)]),
            liquidity_windows: windows([(0, 3, 5), (3, 3, 10), (7, 4, 5)]),
            curve_windows: windows([(10, 5, 5), (18, 6, 10), (28, 7, 5)]),
            fx_windows: windows([(-2, 4, 5), (2, 4, 10), (6, 5, 5)]),
            alert_bias: 220,
        },
        MarketRegime::FxDislocation => RegimeShape {
            price_windows: windows([(-5, 5, 5), (1, 4, 8), (9, 6, 7)]),
            vol_windows: windows([(5, 4, 5), (10, 5, 10), (16, 6, 5)]),
            liquidity_windows: windows([(3, 4, 5), (8, 4, 9), (14, 5, 6)]),
            curve_windows: windows([(1, 3, 5), (4, 4, 10), (8, 4, 5)]),
            fx_windows: windows([(10, 5, 4), (18, 6, 12), (28, 7, 4)]),
            alert_bias: 390,
        },
    }
}

fn windows(raw: [(i64, i64, u64); 3]) -> [ProbabilityWindow; 3] {
    raw.map(|(center, width, weight)| ProbabilityWindow {
        center,
        width,
        weight,
    })
}

fn bucket_series(
    seed: u64,
    salt: u64,
    buckets: usize,
    base: i64,
    slope: i64,
    curvature: i64,
) -> Vec<u64> {
    let center = ((buckets.saturating_sub(1)) as i64) / 2;
    let local = signed_jitter(seed, salt, 3);
    let mut values = Vec::with_capacity(buckets);
    for bucket in 0..buckets {
        let x = bucket as i64 - center;
        let shape = base + slope * x + curvature * x * x + local * x.signum();
        values.push(clamp_u64(shape));
    }
    values
}

fn sample_window(seed: u64, salt: u64, windows: &[ProbabilityWindow; 3]) -> i64 {
    let roll = mix(seed ^ salt) % windows.iter().map(|w| w.weight).sum::<u64>();
    let mut acc = 0_u64;
    let mut selected = windows[0];
    for window in windows {
        acc += window.weight;
        if roll < acc {
            selected = *window;
            break;
        }
    }
    selected.center + signed_jitter(seed, salt ^ 0x55AA, selected.width)
}

fn signed_jitter(seed: u64, salt: u64, width: i64) -> i64 {
    if width <= 0 {
        return 0;
    }
    let span = (width * 2 + 1) as u64;
    mix(seed ^ salt) as i64 % span as i64 - width
}

fn clamp_u64(value: i64) -> u64 {
    value.max(1) as u64
}

fn mix(mut value: u64) -> u64 {
    value ^= value >> 30;
    value = value.wrapping_mul(0xbf58_476d_1ce4_e5b9);
    value ^= value >> 27;
    value = value.wrapping_mul(0x94d0_49bb_1331_11eb);
    value ^ (value >> 31)
}
