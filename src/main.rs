mod init;

use std::env;
use std::path;

fn process_args() -> (Vec<String>, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();
    let mut commands: Vec<String> = Vec::new();

    // throw away how this binary was called
    args.remove(0);

    let mut i: usize = 0;
    loop {
        if i == args.len() {
            break;
        } else {
            i += 1;

            let arg = args[i - 1].clone();
            if &arg[0..=1] != "--" {
                commands.push(arg);
                args.remove(i - 1);
                i -= 1;
            }
        }
    }

    (commands, args)
}

fn main() {
    let (commands, flags) = process_args();

    if !commands.is_empty() {
        if commands[0] == "init" {
            let path = env::current_dir().expect("Could not read working directory!");
            init::new(path);
        } else if commands[0] == "new" {
            let path = path::PathBuf::from(commands.get(1).expect("No directory given!"));
            init::new(path)
        }
    }
}
