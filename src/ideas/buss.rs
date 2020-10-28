use crate::*;

#[derive(Clone)]
pub struct Buss;

// this can probably be converted into a lower bound too

impl Idea for Buss {
    fn new(_: &Graph) -> Self { Buss }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        2 // needs to be smaller than the prio of highdeg
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        let k = state.upper_bound - state.solution.len();
        let g = &state.graph;
        if g.vertices().count() > k * k + k || g.edges().count() > k * k {
            return None;
        } else {
            Some(())
        }
    }
}
