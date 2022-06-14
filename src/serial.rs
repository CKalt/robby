use std::io::prelude::*;
use std::error::Error;
use std::fs::File;
use crate::params::Params;

pub fn serial_num() -> Result<i32, Box<dyn Error>> {
    let serial_file = Params::home_path("run_num")?;
    let mut file = File::open(serial_file)?;

    write!(&mut file, "1")?;
    Ok(1)
}
