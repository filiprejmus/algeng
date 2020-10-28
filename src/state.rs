use std::iter::repeat;

use crate::*;
use crate::graph::Graph;

#[derive(Clone)]
pub struct ExState {
    pub ideas: Vec<Box<dyn InnerIdea>>,
    pub state: State,
}

// Let G be the graph given to ideas[i] on the last .apply() call,
// and let G' be the current graph.
// Then dirty[i].contains(v) <-> G[N(v, G) + v] != G'[N(v, G') + v] (plain equality - not up to isomorphism)

#[derive(Clone)]
pub struct State {
    pub cleanup_funcs: Vec<Box<dyn CleanupFn>>,
    pub dirty: Vec<HashSet<Vx>>,
    pub diff_vec: Vec<usize>,
    pub constraints_vec: Vec<HashSet<Vx>>,
    pub fingertable: HashMap<Vx, HashSet<usize>>,
    pub solution: Vec<Vx>,
    pub upper_bound: usize,
    pub graph: Graph,
}

impl ExState {
    pub fn new(g: Graph) -> ExState {
        let ideas = ideas(&g);
        let dirty = std::iter::repeat(Default::default()) // initially, there are no dirty-bits
            .take(ideas.len()).collect();

        let constraints = constraints(&g);

        // setup fingertable
        let mut fingertable: HashMap<Vx, HashSet<usize>> = Default::default();
        for (i, c) in constraints.0.iter().enumerate() {
            for &elem in c {
                fingertable.entry(elem)
                    .or_insert(Default::default())
                    .insert(i);
            }
        }

        ExState {
            ideas,
            state: State {
                cleanup_funcs: Vec::new(),
                dirty,
                diff_vec: constraints.1,
                constraints_vec: constraints.0,
                fingertable,
                solution: Vec::new(),
                upper_bound: upper_bound(&g),
                graph: g,
            },
        }
    }
    pub fn substate(&self, subgraph:Graph, k:usize)-> ExState{
        //Returns "substate" of self for subgraph and sets dirty bits for everything
        // not in subgraph so ideas recomp. stuff
        //k is the current sol_size in G-subgraph
        #[cfg(debug_assertions)]
        assert!(self.state.upper_bound>=k);
        let mut ex_sub = ExState{
            ideas: self.ideas.clone(),
            state: State{
                cleanup_funcs: Vec::new(),
                dirty: self.state.dirty.clone(),
                diff_vec: Vec::new(),
                constraints_vec: Vec::new(),
                fingertable: Default::default(),
                solution: Vec::new(),
                upper_bound: self.state.upper_bound-k,
                graph: subgraph,
            }
        };
        //copy constraints that include v for all v \in subgraph
        for v in ex_sub.state.graph.vertices(){
            if let Some(indices) = self.state.fingertable.get(&v){
                for i in indices{
                    let index_new = ex_sub.state.constraints_vec.len();
                    ex_sub.state.constraints_vec.push(self.state.constraints_vec[*i].clone());
                    ex_sub.state.diff_vec.push(self.state.diff_vec[*i]);
                    ex_sub.state.fingertable.entry(v).or_insert(Default::default()).insert(index_new);
                }
            }

        }
        //Set dirty for all vertices outside of component (are removed)
        for v in self.state.graph.vertices(){
            if ex_sub.state.graph.get_degree(v)==0 {
                ex_sub.state.make_dirty(v);
            }
        }
        ex_sub

    }
}

fn constraints(_g: &Graph) -> (Vec<HashSet<Vx>>, Vec<usize>) { // TODO
    //unimplemented!()
    (vec![], vec![])
}

fn upper_bound(g: &Graph) -> usize {
    //initialize upper bound using greedy max remaining degree heuristic
    let mut graph = g.clone();
    let mut ub: usize = 0;
    while !graph.is_empty(){
        let v = graph.vertices()
            .max_by_key(|&v| graph.get_degree(v))
            .expect("no vertices?");
        graph.rm_vertex(v);
        ub += 1;

    }
    ub
}
