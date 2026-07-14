use super::g27_same_field_fixed_dual_pricing_support::{has_bit, BitWords};

pub(super) fn exact_mwis(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> (i128, Vec<usize>) {
    let mut solver = MwisSolver {
        adjacency,
        weights,
        best_weight: 0,
        best_vertices: Vec::new(),
    };
    solver.expand(candidates.to_vec(), 0, Vec::new());
    (solver.best_weight, solver.best_vertices)
}

struct MwisSolver<'a> {
    adjacency: &'a [BitWords],
    weights: &'a [i128],
    best_weight: i128,
    best_vertices: Vec<usize>,
}

impl MwisSolver<'_> {
    fn expand(&mut self, mut candidates: Vec<usize>, current_weight: i128, chosen: Vec<usize>) {
        candidates.sort_by(|left, right| {
            self.weights[*right]
                .cmp(&self.weights[*left])
                .then_with(|| left.cmp(right))
        });
        if current_weight + self.coloring_upper_bound(&candidates) <= self.best_weight {
            return;
        }
        if candidates.is_empty() {
            self.retain_best(current_weight, chosen);
            return;
        }
        let vertex = self.branch_vertex(&candidates);
        let remaining = candidates
            .iter()
            .copied()
            .filter(|candidate| *candidate != vertex)
            .collect::<Vec<_>>();
        let next_candidates = remaining
            .iter()
            .copied()
            .filter(|candidate| !has_bit(&self.adjacency[vertex], *candidate))
            .collect::<Vec<_>>();
        let mut next_chosen = chosen.clone();
        next_chosen.push(vertex);
        self.expand(
            next_candidates,
            current_weight + self.weights[vertex],
            next_chosen,
        );
        if current_weight + self.coloring_upper_bound(&remaining) > self.best_weight {
            self.expand(remaining, current_weight, chosen);
        }
    }

    fn branch_vertex(&self, candidates: &[usize]) -> usize {
        candidates
            .iter()
            .copied()
            .max_by(|left, right| {
                let left_score = self.compatible_degree(*left, candidates) * self.weights[*left];
                let right_score = self.compatible_degree(*right, candidates) * self.weights[*right];
                left_score.cmp(&right_score).then_with(|| right.cmp(left))
            })
            .expect("nonempty candidates")
    }

    fn compatible_degree(&self, vertex: usize, candidates: &[usize]) -> i128 {
        candidates
            .iter()
            .filter(|candidate| {
                **candidate != vertex && !has_bit(&self.adjacency[vertex], **candidate)
            })
            .count() as i128
    }

    fn coloring_upper_bound(&self, candidates: &[usize]) -> i128 {
        let mut color_classes: Vec<Vec<usize>> = Vec::new();
        let mut color_weights: Vec<i128> = Vec::new();
        for vertex in candidates {
            let mut assigned = false;
            for (index, color_class) in color_classes.iter_mut().enumerate() {
                if color_class
                    .iter()
                    .all(|other| has_bit(&self.adjacency[*vertex], *other))
                {
                    color_class.push(*vertex);
                    color_weights[index] = color_weights[index].max(self.weights[*vertex]);
                    assigned = true;
                    break;
                }
            }
            if !assigned {
                color_classes.push(vec![*vertex]);
                color_weights.push(self.weights[*vertex]);
            }
        }
        color_weights.into_iter().sum()
    }

    fn retain_best(&mut self, weight: i128, mut vertices: Vec<usize>) {
        vertices.sort_unstable();
        if weight > self.best_weight
            || (weight == self.best_weight && vertices < self.best_vertices)
        {
            self.best_weight = weight;
            self.best_vertices = vertices;
        }
    }
}
