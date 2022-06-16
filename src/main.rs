mod params;
mod opt;
mod srand;
mod error;
mod serial;
//mod const;

use serial::Serial;
use error::AppError;
use params::Params;

use opt::Opt;

//use rand::prelude::*;
use crate::srand::RngFactory;

fn main() -> Result<(), AppError> {
    let params = Params::new()?;
    println!("params = {:?}", params);

    let opt = Opt::new();
    // println!("opt = {:?}", opt);
    // println!("num_runs = {}", params.run.get_num_runs());

    let serial = Serial::new("run_num")?;
    for _i in 0..params.run.get_num_runs() {
        let mut _srng = RngFactory::new(opt.seed);
        let run_num = serial.bump()?;
        params.write_header(&opt, run_num)?;
    }

    // println!("CONTROL_M={}", const::CONTROL_M);

    Ok(())
}
