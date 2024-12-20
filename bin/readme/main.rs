mod aoc;
mod cargo;
mod llm;
mod readme;

use aoc::AOCClient;
use cargo::TestStatus;
use crossbeam_channel::unbounded;
use std::sync::Arc;
use std::thread;

#[derive(Debug)]
struct DayInfo {
    id: usize,
    name: String,
    title: String,
    description: String,
    part1: TestStatus,
    part2: TestStatus,
}

impl DayInfo {
    fn as_markdown(&self) -> String {
        format!(
            r"
### [{}{} {} - {}](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day{}/mod.rs)
            
<details>
<summary>See explanation</summary>

{}
</details>
            ",
            self.part1.as_emoji(),
            self.part2.as_emoji(),
            self.name,
            self.title,
            format!("{:02}", self.id),
            self.description
        )
    }
}

// worker thread processes a day and sends the result to the channel
fn process_day(
    aoc_client: Arc<AOCClient>,
    day_id: usize,
    _exists: bool,
    tx: crossbeam_channel::Sender<DayInfo>,
) {
    let title = aoc_client.get_day_title(day_id).unwrap_or_default();
    if title.is_empty() {
        println!("Day {} does not exist yet", day_id);
        drop(tx);
        return;
    }

    let description = llm::get_day_description(aoc_client, day_id).unwrap_or_default();
    let (part1, part2) =
        cargo::get_test_status(day_id).unwrap_or((TestStatus::Missing, TestStatus::Missing));

    let day = DayInfo {
        id: day_id,
        name: format!("Day {}", day_id),
        title: title.to_string(),
        description: description.to_string(),
        part1,
        part2,
    };

    tx.send(day).expect("failed to send day info");
}

fn main() {
    let force = std::env::args().any(|arg| arg == "--force");

    let readme = readme::Readme::new();
    let (tx, rx) = unbounded::<DayInfo>();
    let worker_handle = readme.worker(rx);

    let aoc_client = Arc::new(aoc::AOCClient::new());

    (move || {
        for i in 1..=25 {
            let day_exists = readme.day_exists(i);
            if day_exists && !force {
                println!("Day {} exists and not called with --force, skipping...", i);
                continue;
            }
            let tx = tx.clone();
            let aoc_client = aoc_client.clone();
            thread::spawn(move || process_day(aoc_client, i, day_exists, tx));
        }
    })();

    worker_handle.join().expect("readme worker thread panicked");

    println!("All done!");
}
