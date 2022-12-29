use std::env;
use std::env::Args;
use std::process;
use std::error;
use std::io;

fn main() {
    let args = env::args();
     
    let specific_challenge = match parse_arguments(args) {
       Ok(s) => s,
       Err(e) => {
            println!("Failed with error: {e}");
            process::exit(1);
        }
    };

    match advent_of_code::run_challenges(specific_challenge) {
        Ok(()) => process::exit(0),
        Err(e) => {
            println!("Failed with error: {e}");
            process::exit(1);
        }
    };

}


fn parse_arguments(mut args : Args) -> Result<usize, Box<dyn error::Error>> {
    args.next(); // drop first file name argument
    let args : Vec<String> = args.collect();

    if args.len() > 1 {
        let e = io::Error::new(io::ErrorKind::Other, "Unsupported number of arguments (0 or 1).");
        return Err(Box::new(e));
    }

    // If no argument, specific_challenge = 0 as default (which is used by 'run_challenges' to mean 'all')
    // If there is an argument, interpret it as a usize
    if args.len() == 0 {
        Ok(0)
    } else {
        Ok(args[0].parse::<usize>()? - 1)
    }
}