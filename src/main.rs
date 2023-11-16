use driver::Runner;

use crate::{
    driver::detect::find_driver,
    errors::{NotGitRepo, SysErrorHandler},
};

pub mod cred;
mod driver;
mod errors;
mod tidal;

#[tokio::main]
async fn main() {
    pretty_env_logger::init();

    let repo = gix::discover(".")
        .map_err(|_| NotGitRepo)
        .handle_system_error();

    let req = crate::tidal::Req::build(&repo).handle_system_error();
    let driver = find_driver(&repo).handle_system_error();

    driver.run(repo, req).await.handle_system_error();
}
