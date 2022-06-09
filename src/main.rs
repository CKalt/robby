mod params;
mod opt;

use params::Params;
use opt::Opt;

fn main() {
    let params = Params::new();
    println!("params = {:?}", params);

    let opt = Opt::new();
    println!("opt = {:?}", opt);
    println!("num_runs = {}", params.run.get_num_runs());

    for i in 0..params.run.get_num_runs() {
        println!("i={}", i);
    }
}
