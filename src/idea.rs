use crate::*;

pub trait Idea: Clone {
    fn new(g: &Graph) -> Self;
    fn priority(&self, state: &State, dirty: &HashSet<Vx>) -> usize;
    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()>; // None -> surrender this State
}

pub trait InnerIdea {
    fn box_clone(&self) -> Box<dyn InnerIdea>;
    fn priority(&self, state: &State, dirty: &HashSet<Vx>) -> usize;
    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()>;
}

impl<T: Idea + 'static> InnerIdea for T {
    fn box_clone(&self) -> Box<dyn InnerIdea> {
        Box::new(self.clone())
    }
    fn priority(&self, state: &State, dirty: &HashSet<Vx>) -> usize {
        Idea::priority(self, state, dirty)
    }

    fn apply(&mut self, state: &mut State, dirty: &HashSet<Vx>) -> Option<()> {
        Idea::apply(self, state, dirty)
    }
}

impl Clone for Box<dyn InnerIdea> {
    fn clone(&self) -> Box<dyn InnerIdea> {
        self.box_clone()
    }
}
