#![allow(dead_code)]

use crate::{HashMap, HashSet};

pub type Vx = usize;
pub type Edge = (Vx, Vx);

#[derive(Clone, Debug)]
pub struct Graph {
    n: usize,
    neighbours: HashMap<Vx, HashSet<Vx>>,
    degrees: Vec<u32>,
}

impl Graph {
    pub fn new<E:Iterator<Item=Edge>>(n: usize, edges: E) -> Graph {
        let mut g = Graph {
            n: n,
            degrees: Vec::new(),
            neighbours: Default::default(),
        };

        for (u, v) in edges {
            #[cfg(debug_assertions)]
            assert!(u != v);
            g.neighbours.entry(u).or_insert(Default::default()).insert(v);
            g.neighbours.entry(v).or_insert(Default::default()).insert(u);
        }

        g.degrees = vec![0; n];

        for &v in g.neighbours.keys() {
            g.degrees[v] = g.neighbours[&v].len() as u32;
        }

        g
    }

    pub fn shrink_to_fit(&mut self) {
        self.neighbours.shrink_to_fit();
        for set in self.neighbours.values_mut() {
            set.shrink_to_fit();
        }
    }

    #[inline]
    pub fn n(&self) -> usize { self.n }

    #[inline]
    pub fn vertices(&self) -> impl Iterator<Item=Vx> + '_ {
        self.neighbours.keys().cloned()
    }

    #[inline]
    pub fn get_degree(&self, v: Vx) -> u32 {
        self.degrees[v]
    }

    #[inline]
    pub fn get_edge(&self, u: Vx, v: Vx) -> bool {
        #[cfg(debug_assertions)]
        assert!(u != v);
        self.neighbours.get(&u)
            .map(|set| set.contains(&v))
            .unwrap_or(false)
    }

    #[inline]
    pub fn rm_vertex(&mut self, u: Vx) -> HashSet<Vx> {
        if let Some(set) = self.neighbours.remove(&u) {
            self.degrees[u] = 0;
            for &v in &set {

                self.degrees[v] -= 1;
                if self.degrees[v] == 0 {
                    self.neighbours.remove(&v);
                } else {
                    self.neighbours.get_mut(&v)
                        .expect("neighbour, but not vice versa?")
                        .remove(&u);
                }
            }
            set
        } else {
            Default::default()
        }
    }

    #[inline]
    pub fn neighbours(&self, u: Vx) -> impl Iterator<Item=Vx> + '_ {
        self.neighbours.get(&u)
            .into_iter()
            .map(|set| set.iter())
            .flatten()
            .cloned()
    }

    #[inline]
    pub fn neighbours_iter(&self) -> impl Iterator<Item=(&Vx, &HashSet<Vx>)> + '_ {
        self.neighbours.iter()
    }

    #[inline]
    // does not give both (u, v) and (v, u) but just one of them!
    pub fn edges(&self) -> impl Iterator<Item=Edge> + '_ {
        self.neighbours_iter()
            .map(move |(u, ne)|
                ne.iter()
                    .filter(move |v| u < v)
                    .map(move |v| (*u, *v))
            ).flatten()
    }

    #[inline]
    // yields (u, v) and (v, u) for all edges
    pub fn edges_sym(&self) -> impl Iterator<Item=Edge> + '_ {
        self.neighbours_iter()
            .map(move |(u, ne)|
                ne.iter()
                    .map(move |v| (*u, *v))
            ).flatten()
    }

    #[inline]
    pub fn is_empty(&self) -> bool {
        self.neighbours.is_empty()
    }

    #[inline]
    pub fn neighbours_hashset(&self, u: Vx)-> Option<&HashSet<Vx>> {
        self.neighbours.get(&u)
    }

    #[inline]
    pub fn add_neighbours(&mut self, v: Vx, new_neighbours: &HashSet<Vx>) {
        #[cfg(debug_assertions)]
        assert!(!new_neighbours.contains(&v));

        self.neighbours.entry(v)
            .or_insert(Default::default())
            .extend(new_neighbours);

        for x in new_neighbours {
            self.neighbours.entry(*x)
                .or_insert(Default::default())
                .insert(v);
            self.degrees[*x] = self.neighbours[x].len() as u32;
        }
        self.degrees[v] = self.neighbours[&v].len() as u32;
    }

    #[inline]
    pub fn add_edge(&mut self, u: Vx, v: Vx) {
        if self.get_edge(u, v) { return; }

        self.neighbours.entry(u)
            .or_insert(Default::default())
            .insert(v);

        self.neighbours.entry(v)
            .or_insert(Default::default())
            .insert(u);

        self.degrees[u] = self.neighbours[&u].len() as u32;
        self.degrees[v] = self.neighbours[&v].len() as u32;
    }

    #[inline]
    pub fn next_edge(&self) -> Option<Edge> {
        self.neighbours.iter()
            .next()
            .and_then(|(u, neigh)| neigh.iter().next()
                .map(|v| (*u,*v))
            )
    }
}
