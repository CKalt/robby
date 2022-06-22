use crate::ga::Chromosome;
use crate::params::Params;
use crate::app::constants;
use crate::srand::SRng;
use rand::Rng;

pub fn init_fitness_function() {
    // for robby nothing in here.
}

pub fn calc_fitness(prm: &Params, chrom: &mut Chromosome, chrom_num: usize,
                    srng: &mut SRng) -> f64 {
    let mut total_score = 0i64;
    for _environment_number in 1..=prm.fitness.num_environments_for_fitness {
        total_score += calc_fitness_one_environment(prm, chrom, chrom_num, srng);
    }

    chrom.fitness = total_score as f64 /
        prm.fitness.num_environments_for_fitness as f64;

    chrom.fitness
}

fn calc_fitness_one_environment(_prm: &Params, _chrom: &mut Chromosome,
        _chrom_num: usize, srng: &mut SRng) -> i64 {
    let _environment = Environment::new(srng);

    /*
    for move_number in 1..=prm.fitness.num_moves {
        let mut state: u8 = 0;

        let n



    }
    */










    0i64
}

#[allow(dead_code)]
struct EnvCell {
    c_row: usize,
    c_column: usize,
    n_row: usize,
    n_column: usize,
    s_row: usize,
    s_column: usize,
    e_row: usize,
    e_column: usize,
    w_row: usize,
    w_column: usize,
    contains_soda_can: bool,
}
impl EnvCell {
    fn new(row: usize, column: usize, srng: &mut SRng) -> Self {
        Self {
            c_row: row,
            c_column: column,
            n_row: row-1,
            n_column: column,
            s_row: row+1,
            s_column: column,
            e_row: row,
            e_column: column+1,
            w_row: row,
            w_column: column-1,
            contains_soda_can: srng.gen(),
        }
    }
}

#[allow(dead_code)]
struct  Environment {
    grid: Vec<Vec<EnvCell>>,
    robby_row: usize,
    robby_column: usize,
}
impl Environment {
    fn new(srng: &mut SRng) -> Self {
        let mut grid: Vec<Vec<EnvCell>> = Vec::new();
        for row in 0..constants::NUM_ENVIRONMENT_ROWS {
            let mut columns: Vec<EnvCell> = Vec::new();
            for column in 0..constants::NUM_ENVIRONMENT_COLUMNS {
                columns.push(EnvCell::new(row, column, srng));
            }
            grid.push(columns);
        }
        Self {
            grid,
            robby_row: 0,
            robby_column: 0,
        }
    }
}
