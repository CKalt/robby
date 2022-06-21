use rand::Rng;
use crate::params::Params;
use crate::srand::SRng;
use crate::app::{actions::Action, fitness::calc_fitness};

#[allow(dead_code)]
pub struct Chromosome {
    pub actions: Vec<Action>,
    fitness: f64,
    weight: f64,
    generation: i32,
    id: i32,
}
impl Chromosome {
    fn new() -> Self {
        Self {
            actions: Vec::new(),
            fitness: 0.0,
            weight: 0.0,
            generation: 0,
            id: 0,
        }
    }
}

type Population = Vec<Chromosome>;

fn generate_popluation(prm: &Params, srng: &mut SRng) -> Population {
    let mut current_pop: Population = Vec::new();
    for _i in 0..prm.run.pop_size {
        let mut chroms = Chromosome::new();
        for _j in 0..prm.run.chrom_length {
            let action: Action = srng.gen();
            chroms.actions.push(action);
        }
        current_pop.push(chroms);
    }
    current_pop
}

pub fn runga(prm: &Params, srng: &mut SRng) {
    let mut _fitness_sum: f64;
    let mut _fitness_sum_squares: f64;
    let mut _fitness_std_dev: f64;
    let mut population = generate_popluation(prm, srng);

    println!("Running the \"Robby the Robot\" genetic algorithm");

    for current_generation in 1..= prm.run.num_generations {
        _fitness_sum = 0.0;
        _fitness_sum_squares = 0.0;
        _fitness_std_dev = 0.0;

        println!("Generation {}", current_generation);

        for (chrom_num, current_chrom) in population.iter_mut().enumerate() {
            calc_fitness(prm, current_chrom, chrom_num, srng);
        }
    }
}
