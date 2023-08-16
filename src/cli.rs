use crate::{ServerArgs, StartArgs, TailwindArgs};

impl From<ServerArgs> for easywind::server::ServerArgs {
    fn from(args: ServerArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            port: args.port,
            open: args.open,
        }
    }
}

impl From<TailwindArgs> for easywind::tailwind::TailwindArgs {
    fn from(args: TailwindArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            input: args.input,
            output: args.output,
            watch: args.watch,
        }
    }
}

impl From<StartArgs> for easywind::start::StartArgs {
    fn from(args: StartArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            port: args.port,
            open: args.open,
            input: args.input,
            output: args.output,
        }
    }
}
