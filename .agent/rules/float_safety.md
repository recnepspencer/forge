# Float Safety Rules

## Doctrine

All geometric comparisons MUST route through `forge_core::ToleranceProvider` methods or the comparison predicates. **No hardcoded float literals** (`1e-10`, `1e-20`, etc.) in production code.

## Banned Patterns

```rust
// ❌ BANNED — hardcoded tolerance
if area < 1e-10 { ... }
if (a - b).abs() < 1e-12 { ... }
let eps = 1e-20;

// ❌ BANNED — direct float equality (clippy::float_cmp will reject)
if a == b { ... }
```

## Sanctioned Alternatives

```rust
use forge_core::{ToleranceProvider, approximately_equal, positions_coincident,
                 is_effectively_zero, is_degenerate_magnitude_sq};

// ✅ Point identity
if positions_coincident(&pos_a, &pos_b, tolerance_provider) { ... }

// ✅ Scalar near-zero
if is_effectively_zero(area, tolerance_provider) { ... }

// ✅ Normal degeneracy (squared magnitude)
if is_degenerate_magnitude_sq(normal_mag_sq, tolerance_provider) { ... }

// ✅ Approximate equality
if approximately_equal(a, b, tolerance_provider) { ... }

// ✅ Direct use of ToleranceProvider
let eps = tolerance_provider.geometry_epsilon();
let vtol = tolerance_provider.vertex_tolerance(idx, gen);
```

## Where to get ToleranceProvider

- **Spatial validators**: Passed via `GeometryContext.tolerance_provider`
- **Kernel operations**: From `ModelingContext`
- **Tests**: Use `FlatToleranceProvider::new(1e-10)` (test code is exempt from the literal ban)

## Exceptions

- **Test code** (`#[cfg(test)]`, `*_tests.rs`): Float literals in assertions are fine
- **Named algorithm constants**: Finite difference steps, grid scales — use named constants with doc comments explaining the value
- **`worth-math`**: Pure math library, no tolerance concerns
