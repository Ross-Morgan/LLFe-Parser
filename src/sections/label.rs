pub trait ValidLabel {
    fn is_label(&self) -> bool;
}

impl ValidLabel for &str {
    fn is_label(&self) -> bool {
        if self.len() == 0 { return false; }

        let mut chars = self.chars();

        let first = chars.next().unwrap();
        let last = chars.next_back().unwrap();

        if !first.is_ascii_lowercase() { return false; }
        if last != ':' { return false; }

        chars.all(|c| c.is_ascii_lowercase() || c.is_ascii_digit() || c == '_')
    }
}
