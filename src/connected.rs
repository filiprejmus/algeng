use crate::*;
use crate::{HashMap, HashSet};


impl Graph {
    pub fn get_components(&self) -> Vec<HashSet<Vx>> {

        let mut g = self.clone();
        let mut v = Vec::new();

        while !g.is_empty(){
            let x = g.vertices().next().expect("empty?");
            let mut current: HashSet<Vx> = Default::default();
            let mut current_unchecked: HashSet<Vx> = Default::default();

            current_unchecked.insert(x);
            while let Some(&y) = current_unchecked.iter().next() {
                current_unchecked.extend(g.rm_vertex(y));
                current_unchecked.remove(&y);
                current.insert(y);
            }
            v.push(current);
        }

        v
    }
}
