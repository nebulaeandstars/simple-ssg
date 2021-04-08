use std::env;
use std::fs;
use std::io::Write;
use std::path;
use std::process::Command;

fn create_project_directories() {
    // Get the path to the current working directory.
    let path = env::current_dir().expect("Could not read working directory!");
    let path_string = path.to_str().unwrap();

    // If it isn't empty, exit with an error.
    if fs::read_dir(path_string).unwrap().count() != 0 {
        panic!("Target directory is not empty!")
    }

    // If in an empty directory, create all template directories.
    fs::create_dir(format!("{}/snippets", path_string)).unwrap();
    fs::create_dir(format!("{}/layouts", path_string)).unwrap();
    fs::create_dir(format!("{}/pages", path_string)).unwrap();
    fs::create_dir(format!("{}/css", path_string)).unwrap();
    fs::create_dir(format!("{}/scripts", path_string)).unwrap();
}

fn create_gitignore() {
    // Get the path to the current working directory.
    let path = env::current_dir().expect("Could not read working directory!");
    let path_string = path.to_str().unwrap();

    // Create the .gitignore template content as a string.
    static GITIGNORE_CONTENT: &str = "target\n";

    // open the .gitignore file in write-only mode.
    let mut file = match fs::File::create(format!("{}/.gitignore", path_string)) {
        Err(why) => panic!("couldn't create file: {}", why),
        Ok(file) => file,
    };

    // then write the template content it.
    file.write_all(GITIGNORE_CONTENT.as_bytes()).unwrap();
}

pub fn new(path: path::PathBuf) {
    // Move into the given directory, or create it if it doesn't exist.
    match env::set_current_dir(&path) {
        Ok(_) => (),
        Err(_) => {
            let s = path.to_str().unwrap();

            fs::create_dir_all(s).unwrap();
            env::set_current_dir(s).unwrap();
            let path = env::current_dir().unwrap();
            println!("creating new project at {}/", path.to_str().unwrap());
        }
    };

    // Then, try to create project directories,
    create_project_directories();

    // create the .gitignore file.
    create_gitignore();

    // and initialise git.
    Command::new("git")
        .arg("init")
        .output()
        .expect("Failed to initialise git repository.");
}
