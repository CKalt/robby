use structopt::StructOpt;

#[derive(Debug, StructOpt)]
#[structopt(name = "robby", about = "An example of GA.")]
#[allow(dead_code)]
pub struct Opt {
    #[structopt(short = "r", long = "file-name")]
    pub file_name: Option<String>,

    #[structopt(short = "s", long = "seed")]
    pub seed: Option<u64>,
}

impl Opt {
    pub fn new() -> Self {
        Opt::from_args()
    }
}
