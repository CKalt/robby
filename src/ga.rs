use rand::Rng;
use crate::params::Params;
use crate::srand::SRng;
use crate::app::{actions::Action, fitness::calc_fitness};

#[allow(dead_code)]
pub struct Chromosome {
    pub actions: Vec<Action>,
    pub fitness: f64,
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
pub struct Ga {
    fitness_mean: f64,
    fitness_std_dev: f64,
    population: Option<Population>,
}
impl Ga {
    pub fn new() -> Self {
        Self {
            fitness_mean: 0.0,
            fitness_std_dev: 0.0,
            population: None,
        }
    }

    pub fn run(&mut self, prm: &Params, srng: &mut SRng) {
        let mut fitness_sum: f64;
        let mut fitness_sum_squares: f64;
//        let current_fitness: f64 = 0.0;
//        let weight_sum: f64;

//        let curent_chrom: i32;
//        let i: i32;
//        let j: i32;
//        let k: i32;
//        let parent1_index: i32;
//        let parent2_index: i32;
//        let next_pop_index: i32;
//        let chrom: i32;
//        let fitness: i32;

        self.generate_population(prm, srng);

        println!("Running the \"Robby the Robot\" genetic algorithm");

        for current_generation in 1..= prm.run.num_generations {
            println!("Generation {}", current_generation);
            fitness_sum = 0.0;
            fitness_sum_squares = 0.0;
            self.fitness_std_dev = 0.0;

            println!("Generation {}", current_generation);

            // Compute the fitness of each individual
            for (chrom_num, chrom) in
                self.population.as_mut().unwrap().iter_mut().enumerate() {
                let current_fitness =
                    calc_fitness(prm, chrom, chrom_num, srng);
                fitness_sum += current_fitness;
                fitness_sum_squares += fitness_sum * fitness_sum;
            }

            // compute fitness statistics
            self.fitness_mean = fitness_sum / (prm.run.pop_size as f64);
            self.fitness_std_dev = (
                    (
                        fitness_sum_squares -
                        (fitness_sum * fitness_sum) / (prm.run.pop_size as f64)
                    ) / (prm.run.pop_size as f64)
                ).sqrt();

               self.fitness_std_dev = (
                    (
                       fitness_sum_squares -
                       (fitness_sum * fitness_sum) / (prm.run.pop_size as f64))
                        / (prm.run.pop_size as f64)
                    ).sqrt();


            self.sort_population();
        }
    }

    fn sort_population(&mut self) {
        let pop = self.population.as_mut().unwrap();
        pop.sort_by(|a, b| b.fitness.partial_cmp(&a.fitness).unwrap());
    }

    fn generate_population(&mut self, prm: &Params, srng: &mut SRng) {
        let mut current_pop: Population = Vec::new();
        for _i in 0..prm.run.pop_size {
            let mut chroms = Chromosome::new();
            for _j in 0..prm.run.chrom_length {
                let action: Action = srng.gen();
                chroms.actions.push(action);
            }
            current_pop.push(chroms);
        }
        self.population = Some(current_pop)
    }
}

