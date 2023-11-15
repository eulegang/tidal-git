pub trait Error: std::error::Error {
    fn status(&self) -> i32;
}

pub trait SysErrorHandler<T> {
    fn handle_system_error(self) -> T;
}

impl<T, E> SysErrorHandler<T> for Result<T, E>
where
    E: Error,
{
    fn handle_system_error(self) -> T {
        match self {
            Ok(t) => t,
            Err(e) => {
                eprintln!("{}", e);
                std::process::exit(e.status());
            }
        }
    }
}

#[derive(Debug, Copy, Clone)]
pub struct NotGitRepo;

impl std::fmt::Display for NotGitRepo {
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        write!(f, "current directory is not in a repository")?;
        Ok(())
    }
}

impl std::error::Error for NotGitRepo {}
impl Error for NotGitRepo {
    fn status(&self) -> i32 {
        32
    }
}
