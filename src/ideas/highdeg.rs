use crate::*;

pub const HIGH_DEG_PRIO: usize = 3;

#[derive(Clone)]
pub struct HighDeg;

impl Idea for HighDeg {
    fn new(_g: &Graph) -> Self { HighDeg }

    fn priority(&self, _state: &State, dirty: &HashSet<Vx>) -> usize {
        if dirty.is_empty() { return 0; }

        HIGH_DEG_PRIO
    }

    fn apply(&mut self, state: &mut State, _dirty: &HashSet<Vx>) -> Option<()> {
        let k = (state.upper_bound - state.solution.len()) as u32;
        let tmp: Vec<Vx> = state.graph
            .vertices()
            .filter(|&v| state.graph.get_degree(v) > k)
            .collect();
        state.extend_solution(tmp.into_iter())
    }
}
