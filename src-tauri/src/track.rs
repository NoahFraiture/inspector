use notify::{Config, RecommendedWatcher, RecursiveMode, Watcher};
use std::path::Path;
use std::sync::mpsc::{Receiver, Sender};

pub fn watch(
  path: &Path,
  sender: Sender<Result<notify::Event, notify::Error>>,
  receiver: Receiver<Result<notify::Event, notify::Error>>,
) -> notify::Result<()> {
  // Automatically select the best implementation for your platform.
  // You can also access each implementation directly e.g. INotifyWatcher.
  let mut watcher = RecommendedWatcher::new(sender, Config::default())?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(path, RecursiveMode::Recursive)?;

  std::thread::spawn(|| {
    for res in receiver {
      match res {
        Ok(event) => log::info!("Change: {event:?}"),
        Err(error) => log::error!("Error: {error:?}"),
      }
    }
  });

  Ok(())
}
