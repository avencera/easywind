use crate::{ServerArgs, StartArgs, TailwindArgs};

impl From<ServerArgs> for easywind::server::Args {
    fn from(args: ServerArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            port: args.port,
            open: args.open,
        }
    }
}

impl From<TailwindArgs> for easywind::tailwind::Args {
    fn from(_args: TailwindArgs) -> Self {
        Self {}
    }
}

impl From<StartArgs> for easywind::start::Args {
    fn from(_args: StartArgs) -> Self {
        Self {}
    }
}
