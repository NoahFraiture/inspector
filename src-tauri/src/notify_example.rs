use notify::{RecursiveMode, Result, Watcher};
use std::collections::HashMap;
use std::io;
use std::path::Path;

fn watch() -> Result<()> {
  // Automatically select the best implementation for your platform.
  let mut watcher = notify::recommended_watcher(|res| match res {
    Ok(event) => println!("event: {:?}", event),
    Err(e) => println!("watch error: {:?}", e),
  })?;

  // Add a path to be watched. All files and directories at that path and
  // below will be monitored for changes.
  watcher.watch(Path::new("./file.txt"), RecursiveMode::Recursive)?;

  let mut buffer = String::new();
  io::stdin().read_line(&mut buffer)?;
  Ok(())
}
