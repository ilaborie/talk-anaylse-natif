use std::collections::HashSet;
use std::hash::Hash;
use crate::solver::SolverError::NoSolutionFound;

#[derive(Debug, Eq, PartialEq)]
pub enum SolverError {
    NoSolutionFound
}

pub trait Problem<S: Clone>: Hash + Clone + Eq {
    fn is_solved(&self) -> bool;

    fn available_steps(&self, previous_steps: &[S]) -> Vec<S>;

    fn next(&self, step: S) -> Self;

    fn solve(&self) -> Result<Vec<S>, SolverError> {
        let mut states = HashSet::new();
        states.insert(self.clone());
        let initial = vec![(self.clone(), vec![])];

        solve_aux(initial, &mut states)
    }
}


fn solve_aux<P, S>(history: Vec<(P, Vec<S>)>, visited: &mut HashSet<P>) -> Result<Vec<S>, SolverError>
    where P: Problem<S> + Sized + Hash, S: Clone {
    // Nowhere to go
    if history.is_empty() { return Err(NoSolutionFound); }

    let mut next = vec![];
    for (state, steps) in history {
        // Try found solution
        if state.is_solved() {
            return Ok(steps);
        }

        // Find next states  FIXME previous
        let available_steps = state.available_steps(&steps);
        for step in available_steps {
            // Apply step
            let new_state = state.clone().next(step.clone());

            if !visited.contains(&new_state) {
                // Found a new state
                visited.insert(new_state.clone());
                let mut next_history = steps.clone();
                next_history.push(step.clone());
                next.push((new_state, next_history));
            }
        }
    }
    // Deeper
    solve_aux(next, visited)
}