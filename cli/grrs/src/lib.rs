pub fn find_matches(content: &str, pattern: &str, mut writer: impl std::io::Write) {
    for line in content.lines() {
        if pattern.is_empty() || line.contains(pattern) {
            writeln!(writer, "{}", line);
        }
    }
}
