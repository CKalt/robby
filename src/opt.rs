use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "robby", about = "An example of GA.")]
#[allow(dead_code)]
pub struct Opt {
    #[structopt(short = "r", long = "file-name")]
    file_name: Option<String>,

    #[structopt(short = "s", long = "seed")]
    seed: Option<u64>,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }
}
