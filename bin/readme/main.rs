use core::panic;
use std::process::Command;

#[derive(Debug)]
enum TestStatus {
    Missing,
    Passed,
    Failed,
}

#[derive(Debug)]
struct DayTest {
    part1: TestStatus,
    part2: TestStatus,
}

fn get_days() -> Vec<DayTest> {
    let mut days = Vec::new();
    for _ in 1..=25 {
        days.push(DayTest {
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
        let day = captures
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

        let day = &mut days[day - 1];
        match part {
            1 => {
                day.part1 = match status {
                    "ok" => TestStatus::Passed,
                    _ => TestStatus::Failed,
                }
            }
            2 => {
                day.part2 = match status {
                    "ok" => TestStatus::Passed,
                    _ => TestStatus::Failed,
                }
            }
            _ => panic!("Invalid part number"),
        }
    }

    days
}

fn build_markdown_table(days: Vec<DayTest>) -> String {
    let mut table = String::new();
    table.push_str("| Day | Part 1 | Part 2 |\n");
    table.push_str("|-----|--------|--------|\n");

    for (i, day) in days.iter().enumerate() {
        table.push_str(&format!(
            "| {:>3} | {:>6} | {:>6} |\n",
            i + 1,
            match day.part1 {
                TestStatus::Missing => "",
                TestStatus::Passed => "⭐️",
                TestStatus::Failed => "❌",
            },
            match day.part2 {
                TestStatus::Missing => "",
                TestStatus::Passed => "⭐️",
                TestStatus::Failed => "❌",
            }
        ));
    }

    table
}
fn main() {
    let days = get_days();
    let table = build_markdown_table(days);

    let readme = std::fs::read_to_string("README.md").unwrap();
    let re = regex::Regex::new(
        r"(?s)(?<pre>.*)<!---Results Table BEGIN-->(.*)<!---Results Table END-->(?<post>.*)$",
    )
    .unwrap();
    let captures = re.captures(&readme).unwrap();
    let pre = captures.name("pre").unwrap().as_str();
    let post = captures.name("post").unwrap().as_str();

    let new_readme = format!(
        "{}<!---Results Table BEGIN-->\n## Status\n{}<!---Results Table END-->{}",
        pre, table, post
    );

    std::fs::write("README.md", new_readme).unwrap();
}
