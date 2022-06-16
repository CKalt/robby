use std::io::Write;
use std::fs::File;
use std::fs;
use std::path::PathBuf;
use std::error::Error;
use crate::params::Params;

pub struct Serial {
    serial_file: PathBuf,
}

impl Serial {
    pub fn new(fname: &str) -> Result<Self, Box<dyn Error>> {
        Ok(
            Self {
                serial_file:  Params::home_path(fname)?,
            }
        )
    }
    pub fn bump(&self) -> Result<i32, Box<dyn Error>> {
        let num = 
            if self.serial_file.exists() { // read file
                let data = fs::read_to_string(&self.serial_file)
                        .expect("Unable to read run_num file");
                println!("serial_num input data: {}", data);
                data.trim().parse::<i32>().unwrap() + 1i32
            } else {
                1i32
            };

        { // write file
            let mut f = File::create(&self.serial_file)
                        .expect("Unable to create run_num file");
            let data = num.to_string();
            f.write_all(data.as_bytes()).expect("Unable to write data");
            println!("serial_num output data: {}", data);
        }

        Ok(num)
    }
}

#[cfg(test)]
mod tests {
    #[test]
    fn test_serial() {


    }
}
