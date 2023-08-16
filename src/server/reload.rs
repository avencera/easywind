use log::{error, info};
use notify_debouncer_mini::DebounceEventResult;
use tower_livereload::Reloader;

static FILE_TYPES: &[&str] = &[
    "html", "css", "js", "jinja", "md", "toml", "js", "ts", "tsx", "j2", "heex", "sface", "eex",
    "leex",
];

pub(crate) fn handle_reload(event: DebounceEventResult, reloader: &Reloader) -> eyre::Result<()> {
    match event {
        Ok(events) => events.iter().for_each(|event| {
            let path = &event.path;

            if path.is_dir() {
                return;
            }

            let Some(extention) = path.extension() else {
                return
            };

            // only reload files that are in watcher file types
            if FILE_TYPES.contains(&extention.to_str().unwrap_or_default()) {
                info!("Reloading {} ...", event.path.to_string_lossy());
                reloader.reload()
            }
        }),

        Err(errors) => errors
            .iter()
            .for_each(|error| error!("Watcher Error {error:?}")),
    }

    Ok(())
}
