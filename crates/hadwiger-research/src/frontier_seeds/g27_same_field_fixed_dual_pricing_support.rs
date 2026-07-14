use super::g27_same_field_mwis_exact::exact_mwis;

const WORDS: usize = 10;

pub(super) type BitWords = [u64; WORDS];

pub(super) fn empty_words() -> BitWords {
    [0; WORDS]
}

pub(super) fn set_bit(words: &mut BitWords, index: usize) {
    words[index / 64] |= 1u64 << (index % 64);
}

pub(super) fn has_bit(words: &BitWords, index: usize) -> bool {
    words[index / 64] & (1u64 << (index % 64)) != 0
}

#[derive(Clone, Debug, Eq, PartialEq)]
pub(super) struct ThresholdMwisBracket {
    pub(super) lower_bound: i128,
    pub(super) upper_bound: i128,
    pub(super) certified_exact: bool,
    pub(super) component_count: usize,
    pub(super) largest_component_size: usize,
    pub(super) exact_component_count: usize,
    pub(super) witness_vertices: Vec<usize>,
}

pub(super) fn threshold_mwis_bracket(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    threshold: i128,
) -> ThresholdMwisBracket {
    let components = connected_components(adjacency, candidates);
    let mut lower_bound = 0;
    let mut upper_bound = 0;
    let mut certified_exact = true;
    let mut exact_component_count = 0;
    let mut witness_vertices = Vec::new();
    let mut largest_component_size = 0;
    for component in &components {
        largest_component_size = largest_component_size.max(component.len());
        let bracket = component_mwis_bracket(adjacency, weights, component);
        lower_bound += bracket.lower_bound;
        upper_bound += bracket.upper_bound;
        if !bracket.certified_exact {
            certified_exact = false;
        } else {
            exact_component_count += 1;
        }
        witness_vertices.extend(bracket.witness_vertices);
        if lower_bound >= threshold {
            witness_vertices.sort_unstable();
            return ThresholdMwisBracket {
                lower_bound,
                upper_bound: lower_bound,
                certified_exact: false,
                component_count: components.len(),
                largest_component_size,
                exact_component_count,
                witness_vertices,
            };
        }
    }
    witness_vertices.sort_unstable();
    ThresholdMwisBracket {
        lower_bound,
        upper_bound,
        certified_exact: certified_exact || upper_bound == lower_bound,
        component_count: components.len(),
        largest_component_size,
        exact_component_count,
        witness_vertices,
    }
}

pub(super) fn greedy_independent_witness(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> (i128, Vec<usize>) {
    let mut best_weight = 0;
    let mut best_vertices = Vec::new();
    let mut orders = Vec::new();
    let mut by_weight = candidates.to_vec();
    by_weight.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    orders.push(by_weight);
    let mut by_weight_reverse = candidates.to_vec();
    by_weight_reverse.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| right.cmp(left))
    });
    orders.push(by_weight_reverse);
    let mut by_low_degree = candidates.to_vec();
    by_low_degree.sort_by(|left, right| {
        degree(adjacency, *left, candidates)
            .cmp(&degree(adjacency, *right, candidates))
            .then_with(|| weights[*right].cmp(&weights[*left]))
    });
    orders.push(by_low_degree);
    let mut by_ratio = candidates.to_vec();
    by_ratio.sort_by(|left, right| {
        let left_degree = degree(adjacency, *left, candidates) as i128 + 1;
        let right_degree = degree(adjacency, *right, candidates) as i128 + 1;
        (weights[*right] * left_degree)
            .cmp(&(weights[*left] * right_degree))
            .then_with(|| left.cmp(right))
    });
    orders.push(by_ratio);
    for order in orders {
        let (weight, vertices) = improve_independent_witness(
            adjacency,
            weights,
            candidates,
            greedy_for_order(adjacency, weights, &order).1,
        );
        if weight > best_weight || (weight == best_weight && vertices < best_vertices) {
            best_weight = weight;
            best_vertices = vertices;
        }
    }
    (best_weight, best_vertices)
}

struct ComponentBracket {
    lower_bound: i128,
    upper_bound: i128,
    certified_exact: bool,
    witness_vertices: Vec<usize>,
}

fn component_mwis_bracket(
    adjacency: &[BitWords],
    weights: &[i128],
    component: &[usize],
) -> ComponentBracket {
    let (greedy_weight, greedy_witness) = greedy_independent_witness(adjacency, weights, component);
    let upper_bound = clique_cover_weight_upper_bound(adjacency, weights, component);
    if greedy_weight == upper_bound {
        return ComponentBracket {
            lower_bound: greedy_weight,
            upper_bound,
            certified_exact: true,
            witness_vertices: greedy_witness,
        };
    }
    if component.len() <= 80 {
        let (exact_weight, exact_witness) = exact_mwis(adjacency, weights, component);
        return ComponentBracket {
            lower_bound: exact_weight,
            upper_bound: exact_weight,
            certified_exact: true,
            witness_vertices: exact_witness,
        };
    }
    ComponentBracket {
        lower_bound: greedy_weight,
        upper_bound,
        certified_exact: false,
        witness_vertices: greedy_witness,
    }
}

fn connected_components(adjacency: &[BitWords], candidates: &[usize]) -> Vec<Vec<usize>> {
    let mut remaining = candidates.to_vec();
    remaining.sort_unstable();
    let mut components = Vec::new();
    while let Some(start) = remaining.pop() {
        let mut stack = vec![start];
        let mut component = Vec::new();
        while let Some(vertex) = stack.pop() {
            component.push(vertex);
            let mut index = 0;
            while index < remaining.len() {
                if has_bit(&adjacency[vertex], remaining[index]) {
                    stack.push(remaining.swap_remove(index));
                } else {
                    index += 1;
                }
            }
        }
        component.sort_unstable();
        components.push(component);
    }
    components.sort_by(|left, right| {
        right
            .len()
            .cmp(&left.len())
            .then_with(|| left.first().cmp(&right.first()))
    });
    components
}

pub(super) fn clique_cover_weight_upper_bound(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
) -> i128 {
    let mut ordered = candidates.to_vec();
    ordered.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    coloring_upper_bound(adjacency, weights, &ordered)
}

fn greedy_for_order(
    adjacency: &[BitWords],
    weights: &[i128],
    order: &[usize],
) -> (i128, Vec<usize>) {
    let mut chosen = Vec::new();
    let mut weight = 0;
    for vertex in order {
        if chosen
            .iter()
            .all(|chosen_vertex| !has_bit(&adjacency[*vertex], *chosen_vertex))
        {
            chosen.push(*vertex);
            weight += weights[*vertex];
        }
    }
    chosen.sort_unstable();
    (weight, chosen)
}

fn improve_independent_witness(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    mut chosen: Vec<usize>,
) -> (i128, Vec<usize>) {
    chosen.sort_unstable();
    loop {
        let mut changed = false;
        let chosen_mask = chosen_mask(&chosen);
        let mut outside = candidates
            .iter()
            .copied()
            .filter(|candidate| !has_bit(&chosen_mask, *candidate))
            .collect::<Vec<_>>();
        outside.sort_by(|left, right| {
            weights[*right]
                .cmp(&weights[*left])
                .then_with(|| left.cmp(right))
        });
        for vertex in outside {
            let conflicts = chosen
                .iter()
                .copied()
                .filter(|chosen_vertex| has_bit(&adjacency[vertex], *chosen_vertex))
                .collect::<Vec<_>>();
            let conflict_weight = conflicts
                .iter()
                .map(|conflict| weights[*conflict])
                .sum::<i128>();
            if conflicts.is_empty() || weights[vertex] > conflict_weight {
                chosen.retain(|chosen_vertex| !conflicts.contains(chosen_vertex));
                chosen.push(vertex);
                chosen.sort_unstable();
                fill_witness_greedily(adjacency, weights, candidates, &mut chosen);
                changed = true;
                break;
            }
        }
        if !changed {
            changed = improve_by_pair_swap(adjacency, weights, candidates, &mut chosen);
        }
        if !changed {
            break;
        }
    }
    let weight = chosen.iter().map(|vertex| weights[*vertex]).sum();
    (weight, chosen)
}

fn improve_by_pair_swap(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    chosen: &mut Vec<usize>,
) -> bool {
    let chosen_bits = chosen_mask(chosen);
    let mut outside = candidates
        .iter()
        .copied()
        .filter(|candidate| !has_bit(&chosen_bits, *candidate))
        .collect::<Vec<_>>();
    outside.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    for left_index in 0..outside.len() {
        let left = outside[left_index];
        for right in outside.iter().skip(left_index + 1) {
            if has_bit(&adjacency[left], *right) {
                continue;
            }
            let conflicts = chosen
                .iter()
                .copied()
                .filter(|chosen_vertex| {
                    has_bit(&adjacency[left], *chosen_vertex)
                        || has_bit(&adjacency[*right], *chosen_vertex)
                })
                .collect::<Vec<_>>();
            let conflict_weight = conflicts
                .iter()
                .map(|conflict| weights[*conflict])
                .sum::<i128>();
            if weights[left] + weights[*right] > conflict_weight {
                chosen.retain(|chosen_vertex| !conflicts.contains(chosen_vertex));
                chosen.push(left);
                chosen.push(*right);
                chosen.sort_unstable();
                fill_witness_greedily(adjacency, weights, candidates, chosen);
                return true;
            }
        }
    }
    false
}

fn fill_witness_greedily(
    adjacency: &[BitWords],
    weights: &[i128],
    candidates: &[usize],
    chosen: &mut Vec<usize>,
) {
    let mut outside = candidates
        .iter()
        .copied()
        .filter(|candidate| !chosen.contains(candidate))
        .collect::<Vec<_>>();
    outside.sort_by(|left, right| {
        weights[*right]
            .cmp(&weights[*left])
            .then_with(|| left.cmp(right))
    });
    for vertex in outside {
        if chosen
            .iter()
            .all(|chosen_vertex| !has_bit(&adjacency[vertex], *chosen_vertex))
        {
            chosen.push(vertex);
        }
    }
    chosen.sort_unstable();
}

fn chosen_mask(chosen: &[usize]) -> BitWords {
    let mut mask = empty_words();
    for vertex in chosen {
        set_bit(&mut mask, *vertex);
    }
    mask
}

fn degree(adjacency: &[BitWords], vertex: usize, candidates: &[usize]) -> usize {
    candidates
        .iter()
        .filter(|candidate| **candidate != vertex && has_bit(&adjacency[vertex], **candidate))
        .count()
}

fn coloring_upper_bound(adjacency: &[BitWords], weights: &[i128], candidates: &[usize]) -> i128 {
    let mut color_classes: Vec<Vec<usize>> = Vec::new();
    let mut color_weights: Vec<i128> = Vec::new();
    for vertex in candidates {
        let mut assigned = false;
        for (index, color_class) in color_classes.iter_mut().enumerate() {
            if color_class
                .iter()
                .all(|other| has_bit(&adjacency[*vertex], *other))
            {
                color_class.push(*vertex);
                color_weights[index] = color_weights[index].max(weights[*vertex]);
                assigned = true;
                break;
            }
        }
        if !assigned {
            color_classes.push(vec![*vertex]);
            color_weights.push(weights[*vertex]);
        }
    }
    color_weights.into_iter().sum()
}
