use tokens::{Attr, Token};

fn is_ident(s: &str) -> bool {
    s
        .chars()
        .all(|c| c.is_ascii_lowercase())
}

fn is_literal(s: &str) -> bool {
    if s.starts_with('"') && s.ends_with('"') && {
        // If char has only 2 quotes
        s.chars().fold(0, |mut acc, c| { if c == '"' { acc += 1; } acc }) == 2
    } {
        return true;
    }

    s.chars().all(|c| c.is_numeric())
}


pub fn parse_attr(attr: &str) -> Attr {
    let mut name = String::new();
    let mut value = Option::<Token>::None;

    let mut chars = attr.chars();
    chars.next();
    chars.next();
    chars.next_back();

    let attr_data = chars.as_str().to_string();

    if attr_data.contains("=") {
        let sides = attr_data.split("=")
            .map(str::trim)
            .map(str::to_string)
            .take(2)
            .collect::<Vec<_>>();

            let lhs = sides.get(0).unwrap().to_string();
            let rhs = sides.get(1).unwrap().to_string();

            value = Some(Token::ATTR(lhs, Some(Box::new(Token::STR(rhs)))));
    }

    Attr { name, value }
}



pub trait ValidAttr {
    fn is_attr(&self) -> bool;
}

impl ValidAttr for &str {
    fn is_attr(&self) -> bool {
        if !self.starts_with("#[") { return false; }
        if !self.ends_with("]") { return false; }
        if self.len() < 4 { return false; }

        let attr_data: String = self.chars().skip(2).take(self.len() - 3).collect();

        if attr_data.contains("=") {
            let sides = attr_data.split("=")
                .map(str::trim)
                .map(str::to_string)
                .take(2)
                .collect::<Vec<String>>();

            let lhs = sides.get(0).unwrap().to_string();
            let rhs = sides.get(1).unwrap().to_string();

            is_ident(lhs.as_str()) && is_literal(rhs.as_str())
        } else {
            is_ident(attr_data.as_str())
        }
    }
}
