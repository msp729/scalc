pub fn progress(p: usize) {
    // reports progress, takes an argument from 0 to 80
    print!("{}{}\r", "=".repeat(p), "-".repeat(80 - p))
}
