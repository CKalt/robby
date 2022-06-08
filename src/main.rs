mod params;
mod opt;

use params::Params;
use opt::Opt;

fn main() {
    let params = Params::new();
    println!("params = {:?}", params);

    let opt = Opt::new();
    println!("opt = {:?}", opt);
}
