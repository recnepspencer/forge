use crate::data::aspect::AspectMask;
use crate::data::error::SignalError;
use crate::data::graph::SignalGraph;
use crate::data::handle::NodeId;
use crate::data::node::NodeState;

pub(super) struct UnsettledDependencyPaths {
    required: Vec<bool>,
}

impl UnsettledDependencyPaths {
    pub(super) fn contains(&self, node: NodeId) -> bool {
        self.required
            .get(node.index() as usize)
            .copied()
            .unwrap_or(false)
    }
}

pub(super) fn discover_unsettled_dependency_paths(
    graph: &mut SignalGraph,
    targets: &[NodeId],
) -> Result<UnsettledDependencyPaths, SignalError> {
    #[derive(Clone, Copy)]
    enum Frame {
        Enter(NodeId),
        Exit(NodeId),
    }

    let capacity = graph.arena_capacity();
    let mut visit_state = vec![0_u8; capacity];
    let mut required = vec![false; capacity];
    for &target in targets {
        let mut stack = vec![Frame::Enter(target)];
        while let Some(frame) = stack.pop() {
            let node = match frame {
                Frame::Enter(node) | Frame::Exit(node) => node,
            };
            let index = node.index() as usize;
            match frame {
                Frame::Enter(_) => match visit_state.get(index).copied() {
                    Some(2) => continue,
                    Some(1) => {
                        return Err(SignalError::invalid_input(format!(
                            "cycle detected while discovering unsettled dependency paths at {node}"
                        )))
                    }
                    Some(0) => {}
                    _ => {
                        return Err(SignalError::invalid_input(format!(
                            "dependency path references unavailable node {node}"
                        )))
                    }
                },
                Frame::Exit(_) => {
                    let state = graph.get_state(node)?;
                    let contract = graph.get_contract(node)?.clone();
                    let dependency_requires_work =
                        graph.runtime_dependencies_of(node)?.iter().any(|edge| {
                            let scopes = edge.scope_ref().map(std::slice::from_ref).unwrap_or(&[]);
                            required[edge.source().index() as usize]
                                && contract.cares_about_change(
                                    AspectMask::from_aspect(edge.aspect()),
                                    scopes,
                                )
                        });
                    required[index] =
                        !matches!(state, NodeState::Clean) || dependency_requires_work;
                    visit_state[index] = 2;
                    continue;
                }
            }
            graph.get_state(node)?;
            visit_state[index] = 1;
            stack.push(Frame::Exit(node));
            for edge in graph.runtime_dependencies_of(node)?.iter().rev() {
                stack.push(Frame::Enter(edge.source()));
            }
        }
    }
    Ok(UnsettledDependencyPaths { required })
}
