extern crate getopts;

extern crate coreutils;

use std::os;


fn main() {
    let args = os::args();
    let program = args[0].clone();

    let opts = &[
        getopts::optflag("h", "help", "display this help and exit"),
        getopts::optflag("V", "version", "output version information and exit"),
    ];

    let matches = match getopts::getopts(args.tail(), opts) {
        Ok(m) => { m },
        Err(f) => {
            println!("{}", f.to_string());
            println!("-------------------------------");
            println!("Usage: {}", getopts::usage(program.as_slice(), opts));
            return;
        },
    };

    if matches.opt_present("help") {
        println!("Usage: {}", getopts::usage(program.as_slice(), opts));
        return;
    }

    if matches.opt_present("version") {
        println!("{}: {}", program, coreutils::version());
        return;
    }

    os::set_exit_status(1);
}
