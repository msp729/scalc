pub fn progress(p: usize) {
    print!("{}{}\r", "=".repeat(p), "-".repeat(80 - p))
}
