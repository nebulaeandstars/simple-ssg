use std::env;

fn process_args() -> (String, Vec<String>, Vec<String>) {
    let mut args: Vec<String> = env::args().collect();
    let mut commands: Vec<String> = Vec::new();

    let directory: String = args[0].clone();
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

    (directory, commands, args)
}

fn main() {
    let (directory, commands, flags) = process_args();

    println!("{:?}", directory);
    println!("{:?}", commands);
    println!("{:?}", flags);
}
