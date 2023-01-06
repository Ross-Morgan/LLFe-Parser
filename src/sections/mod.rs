mod attr;
mod contents;
mod label;

use indexmap::IndexMap;

pub use attr::{parse_attr, ValidAttr};
pub use contents::{Dedent, TrimEnd};
pub use label::ValidLabel;


fn update_buffers(res: &mut IndexMap<String, Vec<String>>, attrs: &mut Vec<String>, name: &mut String, contents: &mut Vec<String>) {
    let name_with_attrs = format!("{}\n{}", attrs.join("\n"), name);

    contents.trim_end();
    contents.dedent();

    res.insert(name_with_attrs.trim_start().to_string(), contents.clone());

    attrs.clear();
    name.clear();
    contents.clear();
}


pub trait SplitSections {
    fn sections(&self) -> IndexMap<String, Vec<String>>;
}

impl SplitSections for String {
    fn sections(&self) -> IndexMap<String, Vec<String>> {
        let mut res = IndexMap::<String, Vec<String>>::new();

        let mut attrs = Vec::<String>::new();
        let mut name = String::new();

        let mut contents = Vec::<String>::new();

        for line in self.split("\n") {
            // Name exists, attr declared
            if !name.is_empty() && (line.is_attr() || line.is_label()) {
                update_buffers(&mut res, &mut attrs, &mut name, &mut contents);
            }

            if line.is_empty() {
                if !name.is_empty() {
                    contents.push("".into());
                }
            }
            else
            if line.is_attr() {
                attrs.push(line.into());
            }
            else
            if line.is_label() {
                name = line.replace(":", "").into();
            }
            else {
                contents.push(line.into());
            }
        }

        if !name.is_empty() {
            update_buffers(&mut res, &mut attrs, &mut name, &mut contents);
        }

        res
    }
}
