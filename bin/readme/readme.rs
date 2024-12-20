use std::thread;

use crate::DayInfo;
use crossbeam_channel::Receiver;

pub struct Readme {
    readme: String,
}

impl Readme {
    pub fn new() -> Self {
        Self {
            readme: std::fs::read_to_string("README.md").unwrap(),
        }
    }

    pub fn day_exists(&self, day_id: usize) -> bool {
        let re = regex::Regex::new(&format!(
            r"(?s)(?<pre>.*)<!---DAY{}_BEGIN-->(?<content>.*)<!---DAY{}_END-->(?<post>.*)$",
            day_id, day_id
        ))
        .unwrap();
        let captures = re.captures(&self.readme).unwrap();
        let content = captures.name("content").unwrap().as_str();
        !content.is_empty()
    }

    // worker thread reads from the channel and updates the README
    pub fn worker(&self, rx: Receiver<DayInfo>) -> thread::JoinHandle<()> {
        thread::spawn(move || {
            while let Ok(day) = rx.recv() {
                println!("Updating day {}", day.id);
                let readme = std::fs::read_to_string("README.md").unwrap();
                let re = regex::Regex::new(&format!(
                    r"(?s)(?<pre>.*)<!---DAY{}_BEGIN-->(.*)<!---DAY{}_END-->(?<post>.*)$",
                    day.id, day.id
                ))
                .unwrap();
                let captures = re.captures(&readme).unwrap();
                let pre = captures.name("pre").unwrap().as_str();
                let post = captures.name("post").unwrap().as_str();
                let new_readme = format!(
                    "{}<!---DAY{}_BEGIN-->\n{}\n<!---DAY{}_END-->{}",
                    pre,
                    day.id,
                    day.as_markdown(),
                    day.id,
                    post
                );
                std::fs::write("README.md", new_readme).unwrap();
                println!("Updated day {}", day.id);
            }
        })
    }
}
