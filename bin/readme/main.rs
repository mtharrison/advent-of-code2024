use core::panic;
use crossbeam_channel::{unbounded, Receiver};
use openai_api_rust::chat::*;
use openai_api_rust::*;
use reqwest;
use std::{os::unix::process, process::Command, thread};

#[derive(Debug)]
enum TestStatus {
    Missing,
    Passed,
    Failed,
}

fn get_day_title(day: usize) -> String {
    let result =
        reqwest::blocking::get(&format!("https://adventofcode.com/2024/day/{}?/input", day))
            .unwrap();

    if result.status().as_u16() == 404 {
        return "".to_string();
    }

    let html = result.text().unwrap();

    let re = regex::Regex::new(r"<h2>-+ Day \d+: (?<title>.*)\ -+<\/h2>").unwrap();
    let captures = re.captures(&html).unwrap_or_else(|| {
        panic!("Failed to parse title from: {}", html);
    });
    captures.name("title").unwrap().as_str().to_string()
}

fn get_day_description(day: usize) -> String {
    // first get the html and parse out the description of the day
    // then load my code for that day
    // call the openai api to get a summary of the code
    // return the summary

    let html = reqwest::blocking::get(&format!("https://adventofcode.com/2024/day/{}?/input", day))
        .unwrap()
        .text()
        .unwrap();

    if html.contains("Please don't repeatedly request this endpoint") {
        return "".to_string();
    }

    let re = regex::Regex::new(r"(?s)<main>(?<main>.*)<\/main>").unwrap();
    let captures = re.captures(&html).unwrap_or_else(|| {
        panic!("Failed to parse main from: {}", html);
    });
    let aoc_description = captures.name("main").unwrap().as_str().to_string();

    let code = match std::fs::read_to_string(format!("src/day{:02}/mod.rs", day)) {
        Ok(code) => code,
        Err(_) => return "".to_string(),
    };

    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");
    let body = ChatBody {
        model: "gpt-4o-mini".to_string(),
        max_tokens: None,
        temperature: None,
        top_p: None,
        n: Some(1),
        stream: Some(false),
        stop: None,
        presence_penalty: None,
        frequency_penalty: None,
        logit_bias: None,
        user: None,
        messages: vec![
          Message { role: Role::System, content: "You will be given an advent of code puzzle description and my code to solve this puzzle in Rust. You must write a very short (just 1 parageph) but informative explanation of the approach that I took to solve the puzzle written in first person. Don't write any filler text or introduction such as stating the name of the puzzle or that it is in Rust. Just launch into the description of the approach. It should be brief and technical and should focus on the choice of datastructures and algorithms. The parsing stage is not to be mentioned.".to_string() },
          Message { role: Role::User, content: format!("Puzzle description: {}\n\nMy code:\n{}", aoc_description, code) },
        ],
    };
    let rs = openai.chat_completion_create(&body);
    let choice = rs.unwrap().choices;
    let message = &choice[0].message.as_ref().unwrap();
    message.content.to_string()
}

fn get_test_status(day: usize) -> (TestStatus, TestStatus) {
    let mut statuses = (TestStatus::Missing, TestStatus::Missing);
    let run_output = Command::new("cargo")
        .arg("test")
        .arg("--lib")
        .arg(format!("day{:02}::tests", day))
        .output()
        .expect("Failed to execute tests");

    let re = regex::Regex::new(r"test_part(?<part>\d) ... (?<status>\w+)$").unwrap();

    for line in String::from_utf8_lossy(&run_output.stdout).lines() {
        let captures = match re.captures(line) {
            Some(captures) => captures,
            None => continue,
        };

        let part = captures
            .name("part")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();

        let status = captures.name("status").unwrap().as_str();

        match part {
            1 => {
                statuses.0 = match status {
                    "ok" => TestStatus::Passed,
                    _ => TestStatus::Failed,
                }
            }
            2 => {
                statuses.1 = match status {
                    "ok" => TestStatus::Passed,
                    _ => TestStatus::Failed,
                }
            }
            _ => panic!("Invalid part number"),
        }
    }

    statuses
}

#[derive(Debug)]
struct DayInfo {
    id: usize,
    name: String,
    title: String,
    description: String,
    part1: TestStatus,
    part2: TestStatus,
}

fn format_dayinfo(day: &DayInfo) -> String {
    let p1 = match day.part1 {
        TestStatus::Missing => "👨‍💻".to_string(),
        TestStatus::Passed => "⭐️".to_string(),
        TestStatus::Failed => "❌".to_string(),
    };

    let p2 = match day.part2 {
        TestStatus::Missing => "👨‍💻".to_string(),
        TestStatus::Passed => "⭐️".to_string(),
        TestStatus::Failed => "❌".to_string(),
    };

    format!(
        "## [{}{} {} - {}](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day{}/mod.rs)\n\n{}\n\n",
        p1, p2, day.name, day.title, format!("{:02}", day.id), day.description
    )
}

// check if a day exists in the README
fn day_exists(readme: &str, day_id: usize) -> bool {
    let re = regex::Regex::new(&format!(
        r"(?s)(?<pre>.*)<!---DAY{}_BEGIN-->(?<content>.*)<!---DAY{}_END-->(?<post>.*)$",
        day_id, day_id
    ))
    .unwrap();
    let captures = re.captures(&readme).unwrap();
    let content = captures.name("content").unwrap().as_str();
    !content.is_empty()
}

// worker thread reads from the channel and updates the README
fn update_readme(rx: Receiver<DayInfo>) {
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
            format_dayinfo(&day),
            day.id,
            post
        );
        std::fs::write("README.md", new_readme).unwrap();
        println!("Updated day {}", day.id);
    }
}

// worker thread processes a day and sends the result to the channel
fn process_day(day_id: usize, _exists: bool, tx: crossbeam_channel::Sender<DayInfo>) {
    let title = get_day_title(day_id);
    if title.is_empty() {
        println!("Day {} does not exist yet", day_id);
        return;
    }

    let description = get_day_description(day_id);
    let (part1, part2) = get_test_status(day_id);

    let day = DayInfo {
        id: day_id,
        name: format!("Day {}", day_id),
        title: title.to_string(),
        description: description.to_string(),
        part1,
        part2,
    };

    tx.send(day).unwrap();
}

fn main() {
    let force = std::env::args().any(|arg| arg == "--force");
    let (tx, rx) = unbounded::<DayInfo>();
    let rx_handle = thread::spawn(move || update_readme(rx));

    let mut tx_handles = Vec::new();
    let readme = std::fs::read_to_string("README.md").unwrap();
    for i in 1..=25 {
        let day_exists = day_exists(&readme, i);
        if day_exists && !force {
            println!("Day {} exists, skipping...", i);
            continue;
        }
        let tx = tx.clone();
        tx_handles.push(thread::spawn(move || process_day(i, day_exists, tx)));
    }

    for handle in tx_handles {
        handle.join().unwrap();
    }

    drop(tx);

    rx_handle.join().unwrap();

    // let auth = Auth::from_env().unwrap();
    // let openai = OpenAI::new(auth, "https://api.openai.com/v1/");

    // let days = get_statuses(&openai);
    // let blocks = days
    //     .iter()
    //     .map(day_to_string)
    //     .collect::<Vec<String>>()
    //     .join("\n");

    // let readme = std::fs::read_to_string("README.md").unwrap();
    // let re = regex::Regex::new(r"(?s)(?<pre>.*)<!---BEGIN-->(.*)<!---END-->(?<post>.*)$").unwrap();
    // let captures = re.captures(&readme).unwrap();
    // let pre = captures.name("pre").unwrap().as_str();
    // let post = captures.name("post").unwrap().as_str();

    // let new_readme = format!("{}<!---BEGIN-->\n{}<!---END-->{}", pre, blocks, post);

    // std::fs::write("README.md", new_readme).unwrap();
}
