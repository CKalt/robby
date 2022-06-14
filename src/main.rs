mod params;
mod opt;
mod srand;
mod error;
mod serial;

use error::AppError;
use params::Params;

use opt::Opt;

use rand::prelude::*;
use crate::srand::RngFactory;

fn main() -> Result<(), AppError> {
    let params = Params::new()?;
    println!("params = {:?}", params);

    let opt = Opt::new();
    println!("opt = {:?}", opt);
    println!("num_runs = {}", params.run.get_num_runs());

    for i in 0..params.run.get_num_runs() {
        println!("i={}", i);
    }

    let seed = Some(737221);
    let mut srng = RngFactory::new(seed);
    let n_u8: u8 = srng.gen();
    println!("ChaChaRng Seeded Random Test Passed u8: {}", n_u8);

    let run_num = serial::serial_num()?;
    println!("run_num = {}", run_num);

    Ok(())
}
