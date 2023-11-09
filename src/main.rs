use std::process::exit;

mod driver;

#[tokio::main]
async fn main() {
    let Ok(repo) = gix::discover(".") else {
        eprintln!("current directory is not in a repository");
        exit(1);
    };

    let config = repo.config_snapshot();

    println!("Hello, world!");
}
