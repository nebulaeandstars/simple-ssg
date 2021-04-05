use std::env;
use std::fs;
use std::path;
use std::process::exit;

fn new(path: path::PathBuf) {
    // Try to move into the given directory.
    match env::set_current_dir(&path) {
        Ok(_) => (),
        // If it doesn't exist, create it.
        Err(_) => {
            let s = path.to_str().unwrap();

            fs::create_dir_all(s).unwrap();
            env::set_current_dir(s).unwrap();
            let path = env::current_dir().unwrap();
            println!("creating new project at {}/", path.to_str().unwrap());
        }
    };

    let path = env::current_dir().expect("Could not read working directory!");
    let path_string = path.to_str().unwrap();

    let paths = fs::read_dir(path_string).unwrap();

    // If the directory isn't empty, exit with an error.
    for _ in paths {
        eprintln!("{} is not an empty directory!", path_string);
        exit(1);
    }

    // If in an empty directory, create all template directories.
    fs::create_dir(format!("{}/snippets", path_string)).unwrap();
    fs::create_dir(format!("{}/layouts", path_string)).unwrap();
    fs::create_dir(format!("{}/pages", path_string)).unwrap();
    fs::create_dir(format!("{}/css", path_string)).unwrap();
    fs::create_dir(format!("{}/scripts", path_string)).unwrap();
}

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
            new(path);
        } else if commands[0] == "new" {
            let path = path::PathBuf::from(commands.get(1).expect("No directory given!"));
            new(path)
        }
    }
}
