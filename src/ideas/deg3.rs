use crate::*;

#[derive(Clone)]
pub struct Deg3(HashSet<Vx>);

#[must_use]
fn apply_deg3(state: &mut State, v: Vx) -> Option<()> {
    let neighbours: Vec<Vx> = state.graph.neighbours(v).collect();
    let (a, b, c) = (neighbours[0], neighbours[1], neighbours[2]);

    // if they are no indep-set: nothing todo
    if state.graph.get_edge(a, b) { return Some(()); }
    if state.graph.get_edge(b, c) { return Some(()); }
    if state.graph.get_edge(a, c) { return Some(()); }

    let mut new_edges = Vec::new();

    let nei: Vec<Vx> = state.graph.neighbours(b).collect();
    for n in nei {
        if n == a { continue; }
        new_edges.push((a, n));
    }

    let nei: Vec<Vx> = state.graph.neighbours(c).collect();
    for n in nei {
        if n == b { continue; }
        new_edges.push((b, n));
    }

    let nei: Vec<Vx> = state.graph.neighbours(a).collect();
    for n in nei {
        if n == c { continue; }
        new_edges.push((c, n));
    }

    new_edges.push((a, b));
    new_edges.push((b, c));

    state.shallow_clear_constraints(a);
    state.shallow_clear_constraints(b);
    state.shallow_clear_constraints(c);
    state.shallow_clear_constraints(v);

    for (v1, v2) in new_edges {
        state.add_edge(v1, v2);
    }

    state.remove_vertex(v);

    state.cleanup_funcs.push(Box::new(move |solution: &mut Vec<Vx>| {
        let a_ = solution.contains(&a);
        let b_ = solution.contains(&b);
        let c_ = solution.contains(&c);

        match (a_, b_, c_) {
            (true,  true,  true ) => return,
            (true,  true,  false) => solution.retain(|vx| *vx != a),
            (false, true,  true ) => solution.retain(|vx| *vx != b),
            (true,  false, true ) => solution.retain(|vx| *vx != c),
            (false, true,  false) => solution.retain(|vx| *vx != b),
            _ => { panic!("impossible!") }
        };
        solution.push(v); // this is always executed, except if a_ && b_ && c_
    }));

    Some(())
}

impl Idea for Deg3 {
    fn new(_g: &Graph) -> Self { Deg3(Default::default()) }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() && self.0.is_empty() { return 0; }

        4
    }

    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()> {
        self.0.extend(dirty.iter().cloned());
        loop {
            let x: Vx = if let Some(&x) = self.0.iter().next() { x }
                        else { return Some(()); };
            self.0.remove(&x);

            if state.graph.get_degree(x) == 3 {
                apply_deg3(state, x)?;
            }
        }
    }
}
