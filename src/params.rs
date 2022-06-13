//***************************************************************************/
//*                                                                         */
//*                           PARAMS                                        */
//*                                                                         */
//*                    Parameter file for GA                                */
//*                                                                         */
//***************************************************************************/

use serde::Deserialize;

/*
enum SelectionMethod {
    FitnessProportionate, 
    LinearRank, 
    PureRank,
    SigmaScaling,
    Elite,
}
*/

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RunParams { 
    // Experiment parameters
    num_runs:   i64,

    // GA parameters
    num_generations:    i64,

    // Options for selection_method are "fitness proportionate", "linear rank", "pure rank"
    // "sigma scaling", and "elite"
    selection_method:   String,
    pop_size:           i64,
    chrom_length:       i64,
    mutation_rate:      f64, // probability of mutation at each locus in a chromosome
    crossover_sites:    i64,
    crossover_rate:     f64, // probability of two parents crossing over
    rank_max_weight:    f64, // Maximum weight for rank scaling 
    sigma_scaling_max_weight: f64, // Maximum weight for sigma scaling 
    num_elite:          f64, // fill in if selection method is "elite" 
}
impl RunParams {
    pub fn get_num_runs(&self) -> i64 { 
        self.num_runs 
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct IOParams { 
    run_num_dir: String,
    output_dir: String,
    snapshot_interval: i64,
    short_print:    bool,
    long_print:    bool,
    best_print:    bool,
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct FitnessParams { 
    fitness_function_name: String,
    wall_penalty: i64, // Points lost for crashing into a wall
    can_reward: i64, // Points gained for picking up a can
    can_penalty: i64, // Points lost for trying to pick up a can in an empty cell 
    num_moves: i64, // Number of moves a robot makes per session 
    num_environments_for_fitness: i64, // Number of environments each 
				  // individual is tested on for 
				  //# calculating fitness
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct Params {
    pub run: RunParams,
    pub io: IOParams,
    pub fitness: FitnessParams,
}
impl Params {
    pub fn new() -> Result<Self, std::io::Error> {
        // location of params.toml is always parent
        // to target directory where executable lives
        let exe = std::env::current_exe()?;
        let dir = exe.parent().expect(
            "Executable must be in some directory");
        let mut file_path = dir.join("");
        file_path.pop();    // remove exe file
        file_path.pop();    // remove target path
        file_path.push("params.toml"); // add file name

        let params_text = std::fs::read_to_string(file_path).unwrap();
        Ok(toml::from_str(&params_text)?)
    }
}
