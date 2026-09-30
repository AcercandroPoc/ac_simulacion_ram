pub mod kinetics;
pub mod mutation;

pub use kinetics::{compute_death_probability, hill_kill_rate, DORMANT_DENSITY_THRESHOLD, LYSIS_RECYCLE_RESOURCE};
pub use mutation::{eval_sos_mutation_prob, resolve_mutation_event, BASAL_MUTATION_RATE};