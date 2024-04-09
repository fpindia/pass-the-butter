use std::{path::Path, time::Duration};

use notify_debouncer_mini::{new_debouncer, notify::RecursiveMode, DebounceEventHandler};

// Setup debouncer
pub fn start_watch<F>(path: &str, f: F)
where
    F: DebounceEventHandler,
{
    // Select recommended watcher for debouncer.
    // Using a callback here, could also be a channel.
    let mut debouncer = new_debouncer(Duration::from_secs(2), None, f).unwrap();

    // Add a path to be watched. All files and directories at that path and
    // below will be monitored for changes.
    debouncer
        .watcher()
        .watch(Path::new(&path), RecursiveMode::Recursive)
        .unwrap();
}
