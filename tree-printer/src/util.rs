pub fn content_dimension(content: &str) -> (i32, i32) {
    let lines = content.lines();
    (
        lines.clone().max().unwrap_or_else(|| "").len() as i32,
        lines.count() as i32,
    )
}
