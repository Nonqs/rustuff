use notify::{Event, RecursiveMode, Result, Watcher};
use std::{path::Path, sync::mpsc};

fn main() -> Result<()> {
    let (tx, rx) = mpsc::channel::<Result<Event>>();

    let mut watcher = notify::recommended_watcher(tx)?;

    watcher.watch(Path::new("/home/tomast/Downloads"), RecursiveMode::Recursive)?;

    for res in rx {
        match res {
            Ok(event) => {
                match event.kind{
                    notify::EventKind::Create(_) => println!("File created"),
                    notify::EventKind::Modify(_) => println!("File modified"),
                    notify::EventKind::Remove(_) => println!("File deleted"),
                    _ => (),
                }
                if let Some(path) = event.paths.get(0) {
                    if let Some(nombre) = path.file_name() {
                        println!("File {:?}", nombre);
                    }
                }
                
            },
            Err(e) => println!("watch error: {:?}", e),
        }
    }

    Ok(())
}