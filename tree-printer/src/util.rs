pub fn content_dimension(content: &str) -> anyhow::Result<(u32, u32)> {
    let lines = content.lines();
    Ok((
        u32::try_from(
            lines
                .clone()
                .max_by(|s1, s2| s1.len().cmp(&s2.len()))
                .unwrap_or_default()
                .len(),
        )?,
        u32::try_from(lines.count())?,
    ))
}
