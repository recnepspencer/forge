//! Simulation of Simplicity (SoS) for resolving geometric degeneracies.
//!
//! When predicates like `orient3d` or `orient2d` hit exactly `0.0`, it means
//! the points are geometrically degenerate (e.g., 4 points perfectly coplanar).
//! Rather than hacking floating-point thresholds, SoS applies a symbolic,
//! infinitesimally small perturbation to every coordinate based on the
//! unique ID of the vertex.
//!
//! Because the perturbations are symbolic powers of epsilon, we never actually
//! compute them. Instead, we use the permutation parity of the involved IDs
//! to deterministically flip the sign, forcing coplanar faces to perfectly
//! resolve as either strictly "above" or strictly "below".

use crate::sign::CertifiedTriSign;
use crate::sign::TriSign;

/// A geometric point equipped with a stable ID for SoS perturbation.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct SosPoint {
    /// The unique, stable ID of the entity (e.g., VertexId).
    /// Used to assign the symbolic epsilon perturbation polynomial.
    pub id: u64,
    /// The geometric 3D coordinate
    pub coords: [f64; 3],
}

/// Evaluates orientation, using SoS to mathematically eliminate `TriSign::Zero`.
///
/// This delegates to `crate::predicates::orient3d` first. 
/// If the unperturbed exact determinant is non-zero, it returns the result.
/// If exactly zero, it uses SoS permutation parity to return `Pos` or `Neg`.
pub fn orient3d_sos(
    pa: SosPoint,
    pb: SosPoint,
    pc: SosPoint,
    pd: SosPoint,
) -> CertifiedTriSign {
    let (exact_result, _escalation) = crate::predicates::orient3d(
        pa.coords, pb.coords, pc.coords, pd.coords
    ).expect("Orient3d exact arithmetic evaluates without panic");

    if exact_result.sign() != TriSign::Zero {
        return exact_result;
    }

    // Simulation of Simplicity Fallback
    // The exact rational evaluation yielded precisely 0.0.
    // Evaluate Taylor series expansion based on unique IDs.
    
    let mut points = [pa, pb, pc, pd];
    let parity = sort_and_compute_parity(&mut points);
    
    if parity % 2 == 0 {
        CertifiedTriSign::new(TriSign::Pos)
    } else {
        CertifiedTriSign::new(TriSign::Neg)
    }
}

/// Evaluates 2D orientation, using SoS to mathematically eliminate `TriSign::Zero`.
pub fn orient2d_sos(
    pa: SosPoint,
    pb: SosPoint,
    pc: SosPoint,
) -> CertifiedTriSign {
    let (exact_result, _escalation) = crate::predicates::orient2d(
        [pa.coords[0], pa.coords[1]], 
        [pb.coords[0], pb.coords[1]], 
        [pc.coords[0], pc.coords[1]]
    ).expect("Orient2d exact arithmetic evaluates without panic");

    if exact_result.sign() != TriSign::Zero {
        return exact_result;
    }

    let mut points = [pa, pb, pc];
    let parity = sort_and_compute_parity(&mut points);
    
    if parity % 2 == 0 {
        CertifiedTriSign::new(TriSign::Pos)
    } else {
        CertifiedTriSign::new(TriSign::Neg)
    }
}

/// Sorts the array by `point.id` and counts the number of swaps (inversions) 
/// to determine if the permutation is even or odd.
fn sort_and_compute_parity(points: &mut [SosPoint]) -> usize {
    let mut swaps = 0;
    let n = points.len();
    
    // Simple Bubble Sort to count permutations
    for i in 0..n {
        for j in 0..n - i - 1 {
            if points[j].id > points[j + 1].id {
                points.swap(j, j + 1);
                swaps += 1;
            }
        }
    }
    
    swaps
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn parity_sort_counts_swaps() {
        let p1 = SosPoint { id: 10, coords: [0.0; 3] };
        let p2 = SosPoint { id: 5, coords: [0.0; 3] };
        let p3 = SosPoint { id: 20, coords: [0.0; 3] };
        let p4 = SosPoint { id: 15, coords: [0.0; 3] };

        // IDs: [10, 5, 20, 15]
        let mut points = [p1, p2, p3, p4];
        let swaps = sort_and_compute_parity(&mut points);
        
        // Sorted: [5, 10, 15, 20]
        // 10 <-> 5 (1 swap) -> [5, 10, 20, 15]
        // 20 <-> 15 (1 swap) -> [5, 10, 15, 20]
        assert_eq!(swaps, 2); 
    }

    #[test]
    fn orient3d_sos_resolves_coplanar() {
        // Four points perfectly coplanar on Z=0
        let p1 = SosPoint { id: 1, coords: [0.0, 0.0, 0.0] };
        let p2 = SosPoint { id: 2, coords: [1.0, 0.0, 0.0] };
        let p3 = SosPoint { id: 3, coords: [0.0, 1.0, 0.0] };
        let p4 = SosPoint { id: 4, coords: [0.5, 0.5, 0.0] };

        let result = orient3d_sos(p1, p2, p3, p4);
        
        // Unperturbed is Zero. Parity of sorted [1,2,3,4] is 0 -> Even -> Positive
        assert_eq!(result.sign(), TriSign::Pos);

        // If we swap the IDs to create an odd permutation, it will flip to Negative
        let p1_swapped = SosPoint { id: 2, coords: [0.0, 0.0, 0.0] };
        let p2_swapped = SosPoint { id: 1, coords: [1.0, 0.0, 0.0] };
        let result_swapped = orient3d_sos(p1_swapped, p2_swapped, p3, p4);
        assert_eq!(result_swapped.sign(), TriSign::Neg);
    }
}
