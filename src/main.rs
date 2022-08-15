use std::path::Path;

use notify::{Watcher, RecursiveMode, Result};

fn main() -> Result<()> {
    let mut watcher = notify::recommended_watcher(|res| {
        match res {
           Ok(event) => println!("event: {:?}", event),
           Err(e) => println!("watch error: {:?}", e),
        }
    })?;

    watcher.watch(Path::new("test_dir"), RecursiveMode::Recursive)?;

    for _ in std::io::stdin().lines() {
        return Ok(())
    }

    Ok(())
}
