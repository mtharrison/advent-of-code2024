use anyhow::Result;
use std::process::Command;

#[derive(Debug)]
pub enum TestStatus {
    Missing,
    Passed,
    Failed,
}

impl TestStatus {
    pub fn as_emoji(&self) -> String {
        match self {
            TestStatus::Missing => "👨‍💻".to_string(),
            TestStatus::Passed => "⭐️".to_string(),
            TestStatus::Failed => "❌".to_string(),
        }
    }
}

pub fn get_test_status(day: usize) -> Result<(TestStatus, TestStatus)> {
    let mut statuses = (TestStatus::Missing, TestStatus::Missing);
    let run_output = Command::new("cargo")
        .arg("test")
        .arg("--lib")
        .arg(format!("day{:02}::tests", day))
        .output()?;

    let re = regex::Regex::new(r"test_part(?<part>\d) ... (?<status>\w+)$")?;

    for line in String::from_utf8_lossy(&run_output.stdout).lines() {
        let captures = match re.captures(line) {
            Some(captures) => captures,
            None => continue,
        };

        let part = match captures.name("part") {
            Some(part) => part.as_str().parse::<usize>()?,
            None => continue,
        };

        let status = match captures.name("status") {
            Some(status) => status.as_str(),
            None => continue,
        };

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
            _ => continue,
        }
    }

    Ok(statuses)
}
