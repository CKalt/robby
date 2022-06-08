mod params;
use params::Params;

fn main() {
    let params = Params::new();
    println!("params = {:?}", params);
}
