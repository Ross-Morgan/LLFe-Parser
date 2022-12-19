use std::collections::HashMap;

use crate::prelude::Parser;

static ALLOWED_HEADER_CHARS: &'static str = "abcdefghijklmnopqrstuvwxyz_1234567890";


fn header_is_valid(header: &str) -> bool {
    if header.len() < 2 { return false; }

    let mut chars = header.trim_end().chars();

    // Get first and last char
    // Guaranteed to be Ok due to length check
    let first_char = chars.next().unwrap();
    let last_char = chars.last().unwrap();

    if !first_char.is_ascii_lowercase() || last_char != ':' {
        return false;
    }

    if !(last_char == ':') {
        return false;
    }

    let mut all_except_last = header.chars();
    all_except_last.next_back();

    for c in all_except_last {
        if !ALLOWED_HEADER_CHARS.contains(c) {

            return false;
        }
    }

    true
}


pub trait SplitSections {
    fn sections(&self) -> HashMap<String, Vec<String>>;
}


impl SplitSections for String {
    fn sections(&self) -> HashMap<String, Vec<String>> {
        let lines = self
            .split("\n")
            .collect::<Vec<&str>>();

        let label_indicies: Vec<(&str, usize)> = self
            .split("\n")
            .enumerate()
            .filter_map(|(idx, val)| {
                if !header_is_valid(val) {
                    None
                } else {
                    Some((val, idx))
                }
            })
            .collect();

        let (names, label_indicies): (Vec<&str>, Vec<usize>) = label_indicies
            .into_iter()
            .unzip();

        let all_indicies = label_indicies.clone();
        let mut all_shifted_one = label_indicies.clone();

        // Remove first function identificatio
        all_shifted_one.remove(0);
        // Add dummy 'next' function that will be ignored
        all_shifted_one.push(lines.len());

        names.into_iter().zip(all_indicies.into_iter().zip(all_shifted_one.into_iter()))
            .fold(HashMap::<String, Vec<String>>::new(), |mut sections, (name, (start, end))| {
                sections.insert(name.to_string(), lines[start+1..end].join("\n").split("\n").map(|s| s.trim_start().to_owned()).collect());
                sections
            })
    }
}


impl<'a> SplitSections for &Parser<'a> {
    fn sections(&self) -> HashMap<String, Vec<String>> {
        self.source.sections()
    }
}
