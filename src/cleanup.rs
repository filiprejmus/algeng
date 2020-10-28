use crate::*;

pub trait CleanupFn: for<'r> Fn(&'r mut Vec<Vx>) {
    fn box_clone(&self) -> Box<dyn CleanupFn>;
}

impl<F> CleanupFn for F
        where F: for<'r> Fn(&'r mut Vec<Vx>) +
                 Clone + 'static {

    fn box_clone(&self) -> Box<dyn CleanupFn> {
        Box::new(self.clone())
    }
}

impl Clone for Box<dyn CleanupFn> {
    fn clone(&self) -> Box<dyn CleanupFn> {
        (**self).box_clone()
    }
}

impl ExState {
    pub fn final_solution(mut self) -> Vec<Vx> {
        while let Some(c) = self.state.cleanup_funcs.pop() {
            (*c)(&mut self.state.solution);
        }
        self.state.solution
    }
}
