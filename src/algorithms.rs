use std::{collections::HashMap, hash::Hash, i64};

#[derive(Clone, PartialEq, Eq, Hash, Debug)]
pub struct ShortestPath<V> {
    pub cost: i64,
    pub sources: Vec<V>,
}

pub fn calculate_shortest_paths<V: Eq + Hash + Clone>(
    edges: HashMap<V, Vec<(V, i64)>>,
    start: V,
) -> HashMap<V, ShortestPath<V>> {
    let mut prices: HashMap<V, ShortestPath<V>> = Default::default();
    let mut pending: HashMap<V, ShortestPath<V>> = Default::default();
    pending.insert(
        start,
        ShortestPath {
            cost: 0,
            sources: vec![],
        },
    );
    while let Some((v, price)) = find_smallest(&pending) {
        let lowest_price = pending.remove(&v).unwrap();
        if let Some(edges) = edges.get(&v) {
            for (dest, cost) in edges {
                // We've already found the cheapest price for this dest vertex
                if prices.contains_key(dest) {
                    continue;
                }
                let candidate = price + cost;
                let current = pending.entry(dest.clone()).or_insert(ShortestPath {
                    cost: i64::MAX,
                    sources: vec![],
                });
                use std::cmp::Ordering;
                match candidate.cmp(&current.cost) {
                    Ordering::Less => {
                        *current = ShortestPath {
                            cost: candidate,
                            sources: vec![v.clone()],
                        }
                    }
                    Ordering::Equal => {
                        current.sources.push(v.clone());
                    }
                    Ordering::Greater => {}
                }
            }
        }
        prices.insert(v, lowest_price);
    }
    prices
}

// TODO: we could use a heap to avoid paying O(n) to find the smallest value.
fn find_smallest<V: Clone>(pending: &HashMap<V, ShortestPath<V>>) -> Option<(V, i64)> {
    let mut v_or = None;
    for (k, v) in pending {
        let smaller = match v_or {
            None => true,
            Some((_, v_other)) => v.cost < v_other,
        };
        if smaller {
            v_or = Some((k, v.cost));
        }
    }
    v_or.map(|(v, c)| (v.clone(), c))
}
