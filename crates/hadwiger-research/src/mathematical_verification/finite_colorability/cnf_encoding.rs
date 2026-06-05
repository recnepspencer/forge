use std::collections::BTreeMap;

use crate::domain_artifacts::GraphVersion;

pub(super) fn encode_graph_coloring(
    graph_version: &GraphVersion,
    color_count: u32,
) -> (Vec<(String, u32, i32)>, Vec<Vec<i32>>) {
    let mut variable_map = Vec::new();
    let mut variable_by_vertex_color = BTreeMap::new();
    let mut next_variable = 1;
    for vertex in graph_version.vertices() {
        for color in 0..color_count {
            variable_map.push((vertex.vertex_label().to_string(), color, next_variable));
            variable_by_vertex_color
                .insert((vertex.vertex_label().to_string(), color), next_variable);
            next_variable += 1;
        }
    }
    let mut clauses = Vec::new();
    for vertex in graph_version.vertices() {
        encode_vertex_color_constraints(
            vertex.vertex_label(),
            color_count,
            &variable_by_vertex_color,
            &mut clauses,
        );
    }
    for edge in graph_version.edges() {
        let (left, right) = edge.endpoints();
        encode_edge_color_constraints(
            left,
            right,
            color_count,
            &variable_by_vertex_color,
            &mut clauses,
        );
    }
    (variable_map, clauses)
}

fn encode_vertex_color_constraints(
    vertex_label: &str,
    color_count: u32,
    variable_by_vertex_color: &BTreeMap<(String, u32), i32>,
    clauses: &mut Vec<Vec<i32>>,
) {
    clauses.push(
        (0..color_count)
            .map(|color| variable_by_vertex_color[&(vertex_label.to_string(), color)])
            .collect(),
    );
    for left_color in 0..color_count {
        for right_color in (left_color + 1)..color_count {
            clauses.push(vec![
                -variable_by_vertex_color[&(vertex_label.to_string(), left_color)],
                -variable_by_vertex_color[&(vertex_label.to_string(), right_color)],
            ]);
        }
    }
}

fn encode_edge_color_constraints(
    left: &str,
    right: &str,
    color_count: u32,
    variable_by_vertex_color: &BTreeMap<(String, u32), i32>,
    clauses: &mut Vec<Vec<i32>>,
) {
    for color in 0..color_count {
        clauses.push(vec![
            -variable_by_vertex_color[&(left.to_string(), color)],
            -variable_by_vertex_color[&(right.to_string(), color)],
        ]);
    }
}
