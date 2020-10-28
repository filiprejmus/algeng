#![allow(unused_imports)]

use std::sync::atomic::{AtomicU32, Ordering};

#[cfg(feature = "count-steps")]
pub static REC_COUNTER: AtomicU32 = AtomicU32::new(0);

pub type HashMap<K, V> = std::collections::HashMap<K, V, fnv::FnvBuildHasher>;
pub type HashSet<V> = std::collections::HashSet<V, fnv::FnvBuildHasher>;

mod parse;
mod state;
mod solve;
mod idea;
mod ideas;
mod cleanup;
mod api;
pub mod graph;
pub mod branch;
pub mod connected;

pub use parse::*;
pub use state::*;
pub use solve::*;
pub use ideas::*;
pub use idea::*;
pub use cleanup::*;
pub use api::*;
pub use graph::{Vx, Edge};


use graph::Graph;

pub fn ideas(g: &Graph) -> Vec<Box<dyn InnerIdea>> {
    vec![
        Box::new(ideas::NeighbourClique::new(g)),
        Box::new(ideas::Unconfined::new(g)),
        Box::new(ideas::CycleLB::new(g)),
        //Box::new(ideas::ILPLB::new(g)),
        Box::new(ideas::CliqueLB::new(g)),
        Box::new(ideas::Crown::new(g)),
        Box::new(ideas::Deg1::new(g)),
        Box::new(ideas::Deg2::new(g)),
        Box::new(ideas::Deg3::new(g)),
        Box::new(ideas::HighDeg::new(g)),
        Box::new(ideas::Buss::new(g)),
        Box::new(ideas::TwoClique::new(g)),
        Box::new(ideas::ConstraintReductions::new(g)),
        Box::new(ideas::Flow::new(g)),
    ]
}

fn get_vertex_cover(g: Graph) -> Vec<Vx> {
    solve(ExState::new(g))
        .expect("no solution at all?")
        .final_solution()
}

fn main() {
    let (g, vertex_names) = parse();
    let vertex_cover = get_vertex_cover(g);
    for v in vertex_cover {
        println!("{}", vertex_names[v]);
    }

    #[cfg(feature = "count-steps")]
    println!("#recursive steps: {}", REC_COUNTER.load(Ordering::SeqCst));
}
