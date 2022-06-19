//***************************************************************************/
//*                                                                         */
//*                           PARAMS                                        */
//*                                                                         */
//*                    Parameter file for GA                                */
//*                                                                         */
//***************************************************************************/

use serde::Deserialize;
use std::io::Write;
use std::io::Error;
use std::path::PathBuf;
use std::fs::File;
use crate::opt::Opt;
use crate::app::constants;

#[derive(Debug, PartialEq)]
pub enum SelectionMethod {
    FitnessProportionate, 
    LinearRank(f64),
    PureRank,
    SigmaScaling(f64),
    Elite(i64),
}
use SelectionMethod::*;

use std::convert::From;

impl From<&RunParams> for SelectionMethod {
    fn from(run: &RunParams) -> Self {
        match &run.selection_method[..] {
            "elite" => SelectionMethod::FitnessProportionate,
            "linear_rank" =>
                SelectionMethod::LinearRank(run.rank_max_weight),
            "pure_rank" => SelectionMethod::PureRank,
            "fitness_proportionate" =>
                SelectionMethod::SigmaScaling(run.sigma_scaling_max_weight),
            "sigma_scaling" => SelectionMethod::Elite(run.num_elite),
            _ => SelectionMethod::FitnessProportionate,
        }
    }
}

#[derive(Deserialize, Debug)]
#[allow(dead_code)]
pub struct RunParams { 
    // Experiment parameters
    num_runs:   i64,

    // GA parameters
    pub num_generations:    i64,

    // Options for selection_method are "fitness proportionate", "linear rank", "pure rank"
    // "sigma scaling", and "elite"
    selection_method:   String,
    pop_size:           usize,
    chrom_length:       i64,
    mutation_rate:      f64, // probability of mutation at each locus in a chromosome
    crossover_sites:    i64,
    crossover_rate:     f64, // probability of two parents crossing over
    rank_max_weight:    f64, // Maximum weight for rank scaling 
    sigma_scaling_max_weight: f64, // Maximum weight for sigma scaling 
    num_elite:          i64, // fill in if selection method is "elite" 
}
impl RunParams {
    pub fn get_num_runs(&self) -> i64 { 
        self.num_runs 
    }
    pub fn get_selection_method(&self) -> SelectionMethod {
        SelectionMethod::from(self)
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
    pub fn write_header(&self, opt: &Opt, run_num: i32) -> Result<(), Error> {
        let mut header = String::new();

        header += &format!(r#"
# RUN NUMBER: {}
# RANDOM SEED: {:?}
# FITNESS FUNCTION: {}
# NUM ENVIRONMENT ROWS: {}
# NUM ENVIRONMENT COLUMNS: {}
# WALL_PENALTY: {}
# CAN_REWARD: {}
# CAN_PENALTY: {}
# NUM_MOVES: {}
# SELECTION METHOD: {}
# CHROM LENGTH: {}
# POPULATION SIZE: {}
# NUM GENERATIONS: {}
# CROSSOVER RATE: {}
# MUTATION RATE: {}"#,
        run_num,
        opt.seed,
        self.fitness.fitness_function_name,
        constants::NUM_ENVIRONMENT_ROWS,
        constants::NUM_ENVIRONMENT_COLUMNS,
        constants::WALL_PENALTY,
        constants::CAN_REWARD,
        constants::CAN_PENALTY,
        constants::NUM_MOVES,
        self.run.selection_method,
        self.run.chrom_length,
        self.run.pop_size,
        self.run.num_generations,
        self.run.crossover_rate,
        self.run.mutation_rate);

        if let Elite(num) = self.run.get_selection_method() {
            header += &format!("# NUM ELITE: {}\n", num)
        } else if let LinearRank(max_weight) = self.run.get_selection_method() {
            header += &format!("# RANK MAX WEIGHT: {}\n", max_weight)
        } else if let SigmaScaling(max_weight) = self.run.get_selection_method() {
            header += &format!("# SIGMA SCALING MAX WEIGHT: {}", max_weight)
        }

        header += &format!(r#"
# CROSSOVER RATE: {}
# MUTATION RATE: {}"#, 
            self.run.crossover_rate,
            self.run.mutation_rate);

        let header_file = Params::home_path(&format!("{}.header", run_num))?;
        let mut f = File::create(&header_file)
                        .expect(
                            &format!(
                                "Unable to create header file ({})",
                                header_file.display()));
             // note: use of ugly unwrap_or_else & panic! rather than
             // expect!() above avoids the cost of format unless it's needed.

        f.write_all(header.as_bytes()).expect("Unable to write data");

        Ok(())
    }
    pub fn new() -> Result<Self, Error> {
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

    /// fn home_path returns abs path to file_name located in Cargo project 
    /// home directory, assuming Cargo type project such that
    /// executable file (i.e. std::env::current_exe() )
    /// resides in grandchild of home path.
    pub fn home_path(file_name: &str) -> Result<PathBuf, Error> {
        let exe = std::env::current_exe()?;
        let dir = exe.parent().expect(
            "Executable must be in some directory");
        let mut file_path = dir.join("");
        file_path.pop();    // remove exe dir (i.e. debug or release)
        file_path.pop();   // remove target dir
        file_path.push(file_name); // add file name
        Ok(file_path)
    }
}
