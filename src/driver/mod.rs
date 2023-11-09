pub trait Driver {
    type Cli: clap::Parser;

    fn cli() -> Self::Cli;
}
