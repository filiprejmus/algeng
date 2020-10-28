use std::mem;

use crate::*;

#[cfg(feature = "count-steps")]
use crate::REC_COUNTER;

#[must_use]
pub fn solve(mut ex: ExState) -> Option<ExState> {
    reduce(&mut ex)?;

    if ex.state.graph.is_empty() { return Some(ex); }

    if !cfg!(feature = "exclude-comps") {
        let comps = ex.state.graph.get_components();
        if comps.len() > 1 {
            return solve_comps(ex,comps);
        }
    }

    let mut sol: Option<ExState> = None;
    let mut ub = ex.state.upper_bound;

    for mut branch in ex_branch(ex) {
        #[cfg(feature = "count-steps")]
        REC_COUNTER.fetch_add(1, Ordering::SeqCst);

        branch.state.upper_bound = ub;

        if let Some(new_sol) = solve(branch){
            #[cfg(debug_assertions)]
            assert!(ub>=new_sol.state.solution.len());
            ub = new_sol.state.solution.len();
            sol = Some(new_sol);
        }
    }
    sol
}

#[must_use]
fn prio(ex: &ExState) -> Option<usize> {
    ex.ideas.iter()
        .enumerate()
        .map(|(i, idea)| {
            let p: usize = idea.priority(&ex.state, &ex.state.dirty[i]);
            (i, idea, p)
        }).max_by_key(|(_, _, p)| *p)
        .filter(|(_, _, p)| *p > 0)
        .map(|(i, _, _)| i)
}

#[must_use]
fn reduce(ex: &mut ExState) -> Option<()> {
    if ex.state.solution.len() > ex.state.upper_bound { return None; }
    if ex.state.graph.is_empty() { return Some(()); }
    if ex.state.solution.len() == ex.state.upper_bound { return None; }

    while let Some(i) = prio(ex) {
        let idea: &mut dyn InnerIdea = &mut *ex.ideas[i];
        let mut new_dirty = Default::default();
        mem::swap(&mut new_dirty, &mut ex.state.dirty[i]);
        idea.apply(&mut ex.state, &new_dirty)?;

        if ex.state.solution.len() > ex.state.upper_bound { return None; }
        if ex.state.graph.is_empty() { return Some(()); }
        if ex.state.solution.len() == ex.state.upper_bound { return None; }
    }
    Some(())
}

#[must_use]
fn ex_branch(ex: ExState) -> Vec<ExState> {
    let ideas = ex.ideas;
    let branches = branch::branch(ex.state);
    let ex_branches: Vec<ExState> = branches.into_iter().map(|s| ExState{
        ideas: ideas.clone(),
        state: s,
    }).collect();
    ex_branches
}

#[must_use]
fn solve_comps(mut ex_state: ExState, comps: Vec<HashSet<Vx>>) -> Option<ExState>{
    let mut k = ex_state.state.solution.len();
    for c in comps {
        //Build substate for component
        let subgraph = Graph::new(ex_state.state.graph.n(),ex_state.state.graph.edges().filter(|(u, v)| c.contains(u) && c.contains(v))); //TODO Smaller n ~~~> Note that vertices().max() <= g.n() is important, so n = .vertices().count() is a bug
        let ex_sub = ex_state.substate(subgraph, k);
        let sub_sol = solve(ex_sub)?;

        //Merge sub_sol into current ex_state
        ex_state.state.solution.extend(sub_sol.state.solution.iter());
        ex_state.state.cleanup_funcs.extend(sub_sol.state.cleanup_funcs);
        k += sub_sol.state.solution.len();
    }
    Some(ex_state)
}
