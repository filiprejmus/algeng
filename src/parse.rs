use std::io::{stdin, BufRead};

use crate::*;

pub fn parse() -> (Graph, Vec<String>) {
    let mut names_backmap: HashMap<String, usize> = Default::default();
    let mut edges: Vec<Edge> = Vec::new();

    for line in stdin().lock().lines() {
        let line = line.expect("error reading line");
        let line = line.split("#").next().unwrap();
        let line = line.trim();

        if line.is_empty() { continue; }

        if let [x, y] = &line.split_whitespace().collect::<Vec<_>>()[..] {
            let n = names_backmap.len();
            let xid = *names_backmap.entry(x.to_string()).or_insert(n);
            let n = names_backmap.len();
            let yid = *names_backmap.entry(y.to_string()).or_insert(n);

            edges.push((xid, yid));
        } else {
            panic!("invalid line: {}", line);
        }
    }

    let mut names: Vec<(String, usize)> = names_backmap.into_iter().collect();
    names.sort_unstable_by_key(|i| i.1);
    let names: Vec<_> = names.into_iter().map(|x| x.0).collect();
    let g = Graph::new(names.len(), edges.into_iter());

    // print(&g, &names);

    (g, names)
}
