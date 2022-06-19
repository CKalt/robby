mod params;
mod opt;
mod srand;
mod error;
mod serial;
mod runga;
mod fitness;
mod app;

use serial::Serial;
use error::AppError;
use params::Params;
use app::actions::Action;

use opt::Opt;

use rand::Rng;
use crate::srand::RngFactory;

fn main() -> Result<(), AppError> {
    let params = Params::new()?;
    println!("params = {:?}", params);

    let opt = Opt::new();
    // println!("opt = {:?}", opt);
    // println!("num_runs = {}", params.run.get_num_runs());

    let serial = Serial::new("run_num")?;
    for _i in 0..params.run.get_num_runs() {
        let mut srng = RngFactory::new(opt.seed);
        let action: Action = srng.gen();
        println!("Random Action = {:?}", action);

        let run_num = serial.bump()?;
        params.write_header(&opt, run_num)?;
        fitness::init_fitness_function();
        runga::runga(&params);
    }

    Ok(())
}
