pub fn normalize_text(input: &str) -> String {
    let mut text = input.replace("\r\n", "\n").replace('\r', "\n");
    text = text.replace('\t', " ");
    let mut lines = Vec::new();
    for line in text.lines() {
        let trimmed = line.trim_end();
        let replaced = trimmed.replace('\u{2022}', "-").replace("* ", "- ");
        lines.push(replaced);
    }
    lines.join("\n")
}
