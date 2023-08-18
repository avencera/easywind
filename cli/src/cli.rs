use crate::{InitArgs, ServerArgs, StartArgs, TailwindArgs};

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

impl From<InitArgs> for easywind::init::InitArgs {
    fn from(args: InitArgs) -> Self {
        Self {
            project_name: args.project_name,
        }
    }
}

pub fn get_styles() -> clap::builder::Styles {
    clap::builder::Styles::styled()
        .usage(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .header(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Yellow))),
        )
        .literal(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .invalid(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .error(
            anstyle::Style::new()
                .bold()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Red))),
        )
        .valid(
            anstyle::Style::new()
                .bold()
                .underline()
                .fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::Green))),
        )
        .placeholder(
            anstyle::Style::new().fg_color(Some(anstyle::Color::Ansi(anstyle::AnsiColor::White))),
        )
}
