use crate::runga::Chromosome;
use crate::params::Params;

pub fn init_fitness_function() {
    // for robby nothing in here.
}

pub fn calc_fitness(prm: &Params, _chromo: &mut Chromosome, _index: usize) -> f64 {
    for _environment_number in 1..=prm.fitness.num_environments_for_fitness {


    }
    0.0f64
}



