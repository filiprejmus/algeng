use std::iter::once;

use crate::*;

mod hopcroftkarp;
use self::hopcroftkarp::max_matching_bipartite;

#[derive(Clone)]
pub struct Crown;

fn vertex_set_of_matching(m: &HashSet<Edge>) -> HashSet<Vx> {
    m.iter().map(|(u,v)|
            once(*u).chain(once(*v))
        ).flatten()
        .collect()
}

fn get_maximal_matching(g: &Graph) -> HashSet<Edge> {
    let mut g = g.clone();
    let mut m: HashSet<Edge> = Default::default();
    loop {
        let opt = g.next_edge();
        if let Some(e) = opt {
            m.insert(e);
            g.rm_vertex(e.0);
            g.rm_vertex(e.1);
        } else { break; }
    }
    m
}

// https://en.wikipedia.org/wiki/K%C5%91nig's_theorem_(graph_theory)#Statement_of_the_theorem
fn minimum_vertex_cover_of_bip(g: &Graph, l: &HashSet<Vx>, r: &HashSet<Vx>) -> HashSet<Vx> {
    // note: there exist degree == 0 vertices
    // for &x in l.union(&r) {
    //    assert!(g.get_degree(x) > 0);
    // }

    let l: HashSet<Vx> = l.iter().cloned().filter(|&v| g.get_degree(v) > 0).collect();
    let r: HashSet<Vx> = r.iter().cloned().filter(|&v| g.get_degree(v) > 0).collect();

    // note: m is not symmetrically!
    // for (u,v) in &m {
    //    assert!(m.contains(&(*v,*u)));
    // }

    let m = max_matching_bipartite(g, &l);
    let m_set = vertex_set_of_matching(&m);

    let u = l.difference(&m_set).cloned();
    let z = enrich_altpath(g, u, &m);

    let k: HashSet<Vx> = l.difference(&z).cloned().collect::<HashSet<Vx>>()
            .union(&r.intersection(&z).cloned().collect::<HashSet<Vx>>())
            .cloned().collect();
    k
}

fn enrich_altpath<Z: Iterator<Item=Vx>>(g: &Graph, z: Z, m: &HashSet<Edge>) -> HashSet<Vx> {
    let mut done_reachable_matched: HashSet<Vx> = Default::default();
    let mut done_reachable_unmatched: HashSet<Vx> = Default::default();

    let mut reachable_unmatched: HashSet<Vx> = Default::default();
    let mut reachable_matched: HashSet<Vx> = z.collect();

    loop {
        let opt1: Option<Vx> = reachable_matched.iter().cloned().next();
        if let Some(x) = opt1 {
            reachable_matched.remove(&x);
            done_reachable_matched.insert(x);
            for y in g.neighbours(x) {
                if !(m.contains(&(x,y)) || m.contains(&(y,x))) && !done_reachable_unmatched.contains(&y) {
                    reachable_unmatched.insert(y);
                }
            }
        }

        let opt2: Option<Vx> = reachable_unmatched.iter().cloned().next();
        if let Some(x) = opt2 {
            reachable_unmatched.remove(&x);
            done_reachable_unmatched.insert(x);
            for y in g.neighbours(x) {
                if (m.contains(&(x,y)) || m.contains(&(y,x))) && !done_reachable_matched.contains(&y) {
                    reachable_matched.insert(y);
                }
            }
        }

        if opt1.is_none() && opt2.is_none() { break; }
    }

    done_reachable_matched.extend(done_reachable_unmatched);
    done_reachable_matched
}


impl Idea for Crown {
    fn new(_: &Graph) -> Self { Crown }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        2 // TODO this priority collides with the lbs
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        let matching = get_maximal_matching(&state.graph);
        let a = vertex_set_of_matching(&matching);
        let b: HashSet<Vx> = state.graph.vertices().filter(|v| !a.contains(v)).collect();

        let iter = state.graph
            .edges()
            .filter(|(u, v)| a.contains(u) != a.contains(v));
        let g = Graph::new(state.graph.n(), iter);
        let x = minimum_vertex_cover_of_bip(&g, &a, &b);
        let ax: HashSet<Vx> = a.intersection(&x).cloned().collect();
        let b0: HashSet<Vx> = b.difference(&x).cloned().collect();
        let crown = (ax, b0);
        let (h, i) = crown;

        state.extend_solution(h.into_iter())?;
        state.remove_vertices(i.into_iter());

        Some(())
    }
}
