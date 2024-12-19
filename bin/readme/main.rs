use core::panic;
use std::process::Command;
use openai_api_rust::*;
use openai_api_rust::chat::*;
use openai_api_rust::completions::*;
use reqwest;

mod llm;
use llm::get_day_description;

#[derive(Debug)]
enum TestStatus {
    Missing,
    Passed(f32),
    Failed,
}

impl std::fmt::Display for TestStatus {
    fn fmt(&self, f: &mut std::fmt::Formatter) -> std::fmt::Result {
        match self {
            TestStatus::Missing => write!(f, ""),
            TestStatus::Passed(t) => match t {
                t if t < &0.01 => write!(f, "⭐️ (<1ms)"),
                t => write!(f, "⭐️ {:.2}s", t),
            },
            TestStatus::Failed => write!(f, "❌"),
        }
    }
}

fn get_day_title(day: usize) -> String {
    let html = reqwest::blocking::get(&format!("https://adventofcode.com/2024/day/{}?/input", day))
        .unwrap()
        .text()
        .unwrap();

    if html.contains("Please don't repeatedly request this endpoint") {
       return "🔒".to_string();
    }

    let re = regex::Regex::new(r"<h2>-+ Day \d+: (?<title>.*)\ -+<\/h2>").unwrap();
    let captures = re.captures(&html).unwrap_or_else(|| {
        panic!("Failed to parse title from: {}", html);
    });
    captures.name("title").unwrap().as_str().to_string()
}

#[derive(Debug)]
struct Day {
    id: usize,
    name: String,
    day_title: String,
    description: String,
    part1: TestStatus,
    part2: TestStatus,
}

fn get_statuses(openai: &OpenAI) -> Vec<Day> {
    let mut days = Vec::new();
    for i in 1..=25 {
        days.push(Day {
            id: i,
            name: format!("Day {}", i),
            day_title: get_day_title(i),
            description: get_day_description(i),
            part1: TestStatus::Missing,
            part2: TestStatus::Missing,
        });
    }

    let output = Command::new("cargo")
        .arg("test")
        .arg("--no-run")
        .output()
        .expect("Failed to execute command");

    if output.status.success() {
        println!("Tests compiled successfully.");
    } else {
        panic!("Tests failed to compile.");
    }

    let run_output = Command::new("cargo")
        .arg("test")
        .output()
        .expect("Failed to execute tests");

    let re =
        regex::Regex::new(r"^test day(?<day>\d+)::tests::test_part(?<part>\d) ... (?<status>\w+)$")
            .unwrap();

    for line in String::from_utf8_lossy(&run_output.stdout).lines() {
        let captures = match re.captures(line) {
            Some(captures) => captures,
            None => continue,
        };
        let dayi = captures
            .name("day")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let part = captures
            .name("part")
            .unwrap()
            .as_str()
            .parse::<usize>()
            .unwrap();
        let status = captures.name("status").unwrap().as_str();

        let day = &mut days[dayi - 1];
        match part {
            1 => {
                day.part1 = match status {
                    "ok" => TestStatus::Passed(get_time_for_test(dayi, part)),
                    _ => TestStatus::Failed,
                }
            }
            2 => {
                day.part2 = match status {
                    "ok" => TestStatus::Passed(get_time_for_test(dayi, part)),
                    _ => TestStatus::Failed,
                }
            }
            _ => panic!("Invalid part number"),
        }
    }

    days
}

fn get_time_for_test(day: usize, part: usize) -> f32 {
    let run_output = Command::new("cargo")
        .arg("test")
        .arg("--lib")
        .arg(format!("day{:02}::tests::test_part{}", day, part))
        .output()
        .expect("Failed to execute tests");

    let re = regex::Regex::new(r"finished in (?P<time>\d+\.\d+)s").unwrap();

    let lines = String::from_utf8_lossy(&run_output.stdout);
    let captures = re.captures(&lines).unwrap_or_else(|| {
        panic!("Failed to parse time from: {}", lines);
    });
    captures
        .name("time")
        .unwrap()
        .as_str()
        .parse::<f32>()
        .unwrap()
}

fn day_to_string(day: &Day) -> String {
    format!(
        "## [{} - {}](https://github.com/mtharrison/advent-of-code2024/blob/main/src/day{}/mod.rs)\n{}\n- Part 1: {}\n- Part 2: {}\n\n{}",
        day.name, 
        day.day_title,
        format!("{:02}", day.id),
        day.day_title,
        day.part1, day.part2, day.description,
    )
}

fn main() {
    let auth = Auth::from_env().unwrap();
    let openai = OpenAI::new(auth, "https://api.openai.com/v1/");


    let days = get_statuses(&openai);
    let blocks = days
        .iter()
        .map(day_to_string)
        .collect::<Vec<String>>()
        .join("\n");

    let readme = std::fs::read_to_string("README.md").unwrap();
    let re = regex::Regex::new(r"(?s)(?<pre>.*)<!---BEGIN-->(.*)<!---END-->(?<post>.*)$").unwrap();
    let captures = re.captures(&readme).unwrap();
    let pre = captures.name("pre").unwrap().as_str();
    let post = captures.name("post").unwrap().as_str();

    let new_readme = format!("{}<!---BEGIN-->\n{}<!---END-->{}", pre, blocks, post);

    std::fs::write("README.md", new_readme).unwrap();
}
