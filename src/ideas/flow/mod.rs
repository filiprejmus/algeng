use std::iter;

mod hopcroftkarp;
use hopcroftkarp::max_matching_bipartite;

mod digraph;
pub use digraph::Digraph;

mod residual;

use crate::*;

#[derive(Clone)]
pub struct Flow;

impl Idea for Flow {
    fn new(_g: &Graph) -> Self { Flow }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        1
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        let n = state.graph.n();

        let s = 2 * n;
        let t = 2 * n + 1;

        let (undir, l) = undir_graph(&state.graph);
        let m = max_matching_bipartite(&undir, &l);

        #[cfg(debug_assertions)]
        for &(u,v) in &m { assert!(u < v); }

        let d = build_digraph(&state.graph);
        let f = matching_to_flow(state.graph.n(), &d, &m);


        #[cfg(debug_assertions)]
        assert!(f.len() == 3 * m.len());

        let r = d.residual(&f, |(u, v)| {
            if u < s && v < s { std::usize::MAX }
            else { 1 }
        });

        #[cfg(debug_assertions)]
        assert_flow(&d, &f);

        let reach = r.reachable(s);
        #[cfg(debug_assertions)]
        assert!(!reach.contains(&t)); // otherwise flow is non-maximal

        let mut to_remove = Vec::new();
        let mut to_solution = Vec::new();

        let mut lb:f64 = 0.0;

        for v in 0..n {
            if state.graph.get_degree(v) == 0 { continue; }

            let reach_l = reach.contains(&v);
            let reach_r = reach.contains(&(v + n));

            if !reach_l && reach_r {
                lb += 1.0;
                to_solution.push(v);
            } else if reach_l && !reach_r {
                to_remove.push(v);
            }
            else {
                lb += 0.5;
            }
        }
        state.lb(lb as usize)?;

        for x in to_solution {
            state.add_to_solution(x)?;
        }

        for x in to_remove {
            state.remove_vertex(x);
        }

        Some(())
    }
}

fn undir_graph(g: &Graph) -> (Graph, HashSet<Vx>) {
    let n = g.n();
    let l = (0..n).collect();
    let edges = g.edges()
            .map(|(i, j)| {
                iter::once((i, j+n)).chain(
                iter::once((j, i+n))
                )
            }).flatten();
    let out_g = Graph::new(2*n, edges);
    (out_g, l)
}

fn build_digraph(g: &Graph) -> Digraph {
    let n = g.n();

    let s = n * 2;
    let t = n * 2 + 1;

    let middle_edges = g.edges()
            .map(|(i, j)| {
                iter::once((i, j+n)).chain(
                iter::once((j, i+n))
                )
            }).flatten();
    let st_edges = g.vertices()
            .map(|i| {
                iter::once((s, i)).chain(
                iter::once((i+n, t))
                )
            }).flatten();
    let edges = middle_edges.chain(st_edges);
    Digraph::new(n * 2 + 2, edges)
}

fn matching_to_flow(n: usize, d: &Digraph, m: &HashSet<Edge>) -> HashSet<Edge> {
    let s = n * 2;
    let t = n * 2 + 1;

    let is_matched = |x| -> bool {
        m.iter().any(|&(u, v)| x == u || x == v)
    };

    let fc = |u, v| -> bool {
        if u == s {
            #[cfg(debug_assertions)]
            assert!(v < n);
            is_matched(v)
        } else if v == t {
            #[cfg(debug_assertions)]
            assert!(u >= n && u <= 2*n);
            is_matched(u)
        } else {
            let min = std::cmp::min(u, v);
            let max = std::cmp::max(u, v);

            m.contains(&(min, max))
        }
    };

    d.edges()
        .filter(|&(u,v)| fc(u, v)) // hm...
        .collect()
}

fn assert_flow(d: &Digraph, f: &HashSet<Edge>) {
    let s = d.n - 2;

    for v in d.vertices() {
        if v < s {

            let in_ = d.pred(v).map(|p| (p, v))
                .filter(|e| f.contains(e)).count();

            let out_ = d.succ(v).map(|p| (v, p))
                .filter(|e| f.contains(e)).count();

            assert_eq!(in_, out_);
        }
    }
}
