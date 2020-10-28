use crate::*;
use crate::{HashMap, HashSet};

#[must_use]
pub fn branch(s: State) -> Vec<State> {
    let v = max_degree(&s.graph);
    let mut branches:Vec<State> = vec![];
    let s_clone = s.clone();
    if let Some(s1) = branch_left(s,v) {branches.push(s1);}
    if let Some(s2) = branch_right(s_clone,v) {branches.push(s2);}

    branches

}

fn max_degree(g: &Graph)->Vx{
    //Returns vertex v with max degree
    let v = g.vertices()
        .max_by_key(|&v| g.get_degree(v))
        .expect("no vertices?");
    v
}
fn find_mirrors(g: &Graph, v: Vx)->HashSet<Vx>{
    //Returns M[v]
    let mut candidates:HashSet<Vx> = Default::default();
    let neighbours: HashSet<Vx> = g.neighbours(v).collect();
    for u in &neighbours{ candidates.extend(g.neighbours(*u));}

    candidates.retain(|&u| is_mirror(g,u,&neighbours));
    candidates.insert(v);
    candidates


}
fn is_mirror(g: &Graph, u:Vx, n_v: &HashSet<Vx>)->bool{
    let induced: HashSet<Vx> = n_v.difference(g.neighbours_hashset(u).unwrap()).cloned().collect();
    //Check if induced is a (possibly empty) clique
    for w in &induced {
        if !induced.iter().all(|&v| (v==*w || g.get_edge(v, *w))){return false;} //TODO Possible factor 2 speedup by removing w
    }
    true

}
fn add_constraint(state: &mut State,s: HashSet<Vx>, diff: usize){
    if cfg!(feature = "exclude-constraints") { return; }

    let index = state.constraints_vec.len();
    for v in &s{
        state.fingertable.entry(*v).or_insert(Default::default()).insert(index);
    }
    state.constraints_vec.push(s);
    state.diff_vec.push(diff);

}

#[must_use]
fn branch_left(mut s:State, v:Vx)->Option<State>{
    //Returns State corresponding to case where M[v] is included or None if this Branch violates constr.
    let mirrors: HashSet<Vx>= find_mirrors(&s.graph,v);

    if !cfg!(feature = "exclude-constraints") {
        //Introduce new constraint
        let neighbours: HashSet<Vx> = s.graph.neighbours(v).collect();
        let mut diff:usize = neighbours.len() - 1;
        for u in &s.solution{
            if neighbours.contains(u) {
                diff = diff.checked_sub(1)?;
            }
        }
        add_constraint(&mut s, neighbours,diff);
    }

    if cfg!(feature = "exclude-mirror") {
        s.add_to_solution(v)?;
    } else {
        s.extend_solution(mirrors.into_iter())?;
    }

    return Some(s);

}

#[must_use]
fn branch_right(mut s:State, v:Vx)->Option<State>{
    //Returns State corresponding to case where N(v) is included or None if this Branch violates constr.
    let neighbours: HashSet<Vx> = s.graph.neighbours(v).collect();

    //Introduce new constraints
    if !cfg!(feature = "exclude-constraints") {
        for w in &neighbours{
            let mut neigh_private:HashSet<Vx> = s.graph.neighbours(*w).collect();
            neigh_private.remove(&v);
            for u in &neighbours{neigh_private.remove(u);}
            if neigh_private.is_empty(){
                println!("# should have been handled by unconfined!");
                return None;
            }
            let mut diff:usize = neigh_private.len() - 1;
            for u in &s.solution{
                if neigh_private.contains(u) {
                    diff = diff.checked_sub(1)?;
                }
            }
            add_constraint(&mut s, neigh_private,diff);
        }
    }
    //Update solution, constrainst and graph
    s.extend_solution(neighbours.into_iter())?;

    return Some(s);

}

