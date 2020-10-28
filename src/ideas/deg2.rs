use crate::*;

#[derive(Clone)]
pub struct Deg2(HashSet<Vx>);

#[must_use]
fn apply_deg2(state: &mut State, x: Vx) -> Option<()> {
    let neighbours: Vec<Vx> = state.graph.neighbours(x).collect();
    let neighbours = [neighbours[0], neighbours[1]];

    if state.graph.get_edge(neighbours[0], neighbours[1]) {
        return state.extend_solution(neighbours.iter().cloned());
    }

    // calculating full neighbourhood
    let mut full_neighbourhood: HashSet<_> = Default::default();
    for &neigh in neighbours.iter() {
        full_neighbourhood.extend(state.graph.rm_vertex(neigh).into_iter());
    }
    full_neighbourhood.remove(&x);

    // adding corresponding edges
    for &f in &full_neighbourhood {
        state.graph.add_edge(x, f);
    }

    // dirty bits & clearing constraints
    let mut changed = full_neighbourhood;
    changed.extend(&neighbours);
    changed.insert(x);

    for v in changed {
        state.make_dirty(v);
    }
    state.shallow_clear_constraints(neighbours[0]);
    state.shallow_clear_constraints(neighbours[1]);
    state.shallow_clear_constraints(x);

    state.add_to_solution(neighbours[0])?;

    state.cleanup_funcs.push(Box::new(move |solution: &mut Vec<Vx>| {
        if let Some(i) = solution.iter().position(|v| *v == x) {
            solution.swap_remove(i);
            solution.push(neighbours[1]);
        } else {
            let j = solution.iter().position(|v| *v == neighbours[0]).expect("but, I added neighbours[0] to the solution!");
            solution[j] = x;
        }
    }));
    return Some(());
}

impl Idea for Deg2 {
    fn new(_g: &Graph) -> Self { Deg2(Default::default()) }

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

            if state.graph.get_degree(x) == 2{
                apply_deg2(state, x)?;
            }
        }
    }
}
