pub fn get_day_description(day: usize) -> String {
    let path = format!("bin/day{}/description.md", day);
    path
}
