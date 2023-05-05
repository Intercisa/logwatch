use notify::{RecommendedWatcher, RecursiveMode, Watcher, Config};
use std::fs::{File};
use std::io::{BufRead, BufReader};
use std::path::Path;

fn main() {
    let path = std::env::args()
        .nth(1)
        .expect("Argument 1 needs to be a path");
    let search = std::env::args()
        .nth(2)
        .expect("Argument 2 needs to be a path");
    println!("search for {}", search);
    println!("watching {}", path);
    read_iter(&path, &search);
    if let Err(e) = watch(path, &search) {
        println!("error: {:?}", e)
    }

    fn watch<P: AsRef<Path>>(path: P, search: &str) -> notify::Result<()> {
        let (tx, rx) = std::sync::mpsc::channel();

        // Automatically select the best implementation for your platform.
        // You can also access each implementation directly e.g. INotifyWatcher.
        let mut watcher = RecommendedWatcher::new(tx, Config::default())?;

        // Add a path to be watched. All files and directories at that path and
        // below will be monitored for changes.
        watcher.watch(path.as_ref(), RecursiveMode::Recursive)?;

        for res in rx {
            match res {
                Ok(event) => {
                    if event.kind.is_modify() {
                        read_iter(&path.as_ref().display().to_string(), search);
                    }
                },
                Err(e) => println!("watch error: {:?}", e),
            }
        }

        Ok(())
    }

    fn read_iter(file_name: &str, search: &str) {
        let file = File::open(file_name).expect("file not found!");
        let reader = BufReader::new(file);
        print!("{}[2J", 27 as char);
        for line in reader.lines() {
            let l = line.unwrap().to_string();
            if l.contains( search) {
                println!("{:?}", l);
                println!("{}", "\n");

            }
        }
    }
}
