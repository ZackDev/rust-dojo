use clap::{self, arg, Parser};

#[derive(Parser)]

struct Args {
    /// the duration of the ride in minutes
    #[arg(short, long, exclusive=true, required=false)]
    time: u32,

    #[arg(short, long, exclusive=true, required=false)]
    value: u32,
}
fn main() {
    Args::parse();
}



/*
    this behaviour is interesting. leaving one argument out, for example:
    
    multiinput -t 20

    requires value to be passed too.


    multiinput -t 20 -v 5

    says that both arguments cannot be passed together
*/