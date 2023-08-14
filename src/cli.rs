use crate::ServerArgs;

impl From<ServerArgs> for easywind::server::Args {
    fn from(args: ServerArgs) -> Self {
        Self {
            root_dir: args.root_dir,
            port: args.port,
            open: args.open,
        }
    }
}
