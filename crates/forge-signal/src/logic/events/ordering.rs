use std::collections::{BTreeSet, VecDeque};

use crate::data::event_subscriber::SubscriberId;

use super::errors::SubscriberRegistryError;
use super::runtime::SubscriberEntry;

pub(super) fn resolve_order<E, D, C>(
    entries: &[SubscriberEntry<E, D, C>],
) -> Result<Vec<usize>, SubscriberRegistryError<D>>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    let mut provider_for = std::collections::BTreeMap::<D, usize>::new();
    for (idx, entry) in entries.iter().enumerate() {
        for &data in entry.provides {
            if let Some(prev_idx) = provider_for.get(&data).copied() {
                return Err(SubscriberRegistryError::DuplicateProvider {
                    data_id: data,
                    first: entries[prev_idx].name,
                    second: entry.name,
                });
            }
            provider_for.insert(data, idx);
        }
    }

    let n = entries.len();
    let mut indegree = vec![0usize; n];
    let mut edges: Vec<Vec<usize>> = vec![Vec::new(); n];
    for (consumer_idx, entry) in entries.iter().enumerate() {
        for &required in entry.requires {
            let Some(provider_idx) = provider_for.get(&required).copied() else {
                return Err(SubscriberRegistryError::MissingProvider {
                    subscriber: entry.name,
                    data_id: required,
                });
            };
            if provider_idx != consumer_idx {
                edges[provider_idx].push(consumer_idx);
                indegree[consumer_idx] += 1;
            }
        }
    }

    let mut ready: BTreeSet<(SubscriberId, usize)> = BTreeSet::new();
    for (idx, entry) in entries.iter().enumerate() {
        if indegree[idx] == 0 {
            ready.insert((entry.id, idx));
        }
    }

    let mut resolved = Vec::with_capacity(n);
    while let Some((_, idx)) = ready.pop_first() {
        resolved.push(idx);
        for &dst in &edges[idx] {
            indegree[dst] -= 1;
            if indegree[dst] == 0 {
                ready.insert((entries[dst].id, dst));
            }
        }
    }

    if resolved.len() != n {
        let cycle = build_cycle_chain(&edges, entries);
        return Err(SubscriberRegistryError::CycleDetected { cycle_chain: cycle });
    }

    Ok(resolved)
}

fn build_cycle_chain<E, D, C>(
    edges: &[Vec<usize>],
    entries: &[SubscriberEntry<E, D, C>],
) -> Vec<&'static str>
where
    D: Copy + Ord + std::fmt::Debug + 'static,
{
    let n = edges.len();
    let mut color = vec![0u8; n]; // 0=white,1=gray,2=black
    let mut parent = vec![usize::MAX; n];

    fn dfs<E, D, C>(
        u: usize,
        edges: &[Vec<usize>],
        color: &mut [u8],
        parent: &mut [usize],
    ) -> Option<(usize, usize)>
    where
        D: Copy + Ord + std::fmt::Debug + 'static,
    {
        color[u] = 1;
        for &v in &edges[u] {
            if color[v] == 0 {
                parent[v] = u;
                if let Some(c) = dfs::<E, D, C>(v, edges, color, parent) {
                    return Some(c);
                }
            } else if color[v] == 1 {
                return Some((u, v));
            }
        }
        color[u] = 2;
        None
    }

    for i in 0..n {
        if color[i] != 0 {
            continue;
        }
        if let Some((from, to)) = dfs::<E, D, C>(i, edges, &mut color, &mut parent) {
            let mut chain_idx = VecDeque::new();
            chain_idx.push_front(to);
            let mut cur = from;
            chain_idx.push_front(cur);
            while cur != to {
                let p = parent[cur];
                if p == usize::MAX {
                    break;
                }
                cur = p;
                chain_idx.push_front(cur);
            }
            return chain_idx.into_iter().map(|idx| entries[idx].name).collect();
        }
    }

    vec!["<unknown-cycle>"]
}
