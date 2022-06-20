use rand::Rng;
use crate::params::Params;
use crate::srand::SRng;
use crate::app::actions::Action;

fn generate_popluation(prm: &Params, srng: &mut SRng) {
    let mut current_pop: Vec<Vec<Action>> = Vec::new();
    for _i in 0..prm.run.pop_size {
        let mut chroms : Vec<Action> = Vec::new();
        for _j in 0..prm.run.chrom_length {
            let action: Action = srng.gen();
            chroms.push(action);
        }
        current_pop.push(chroms);
    }
}

pub fn runga(prm: &Params, srng: &mut SRng) {
    generate_popluation(prm, srng);

    for _current_generation in 1..= prm.run.num_generations {


    }
}
