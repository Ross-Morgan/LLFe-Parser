pub trait Dedent {
    fn dedent(&mut self);
}

impl Dedent for Vec<String> {
    fn dedent(&mut self) {
        let ident_size = self
            .iter()
            .map(|s| s.len() - s.trim_start().len())
            .filter(|&x| x > 0)
            .min()
            .unwrap_or(0);

        self
            .iter_mut()
            .for_each(|x| {
                if x.len() < ident_size {
                    x.clear();
                    return;
                }

                for _ in 0..ident_size {
                    x.remove(0);
                }
            })
    }
}


pub trait TrimEnd {
    fn trim_end(&mut self);
}

impl TrimEnd for Vec<String> {
    fn trim_end(&mut self) {
        loop {
            if self.last().is_none() { return; }

            let last = self.last().unwrap();

            if last.chars().all(|c| c.is_whitespace()) {
                self.remove(self.len() - 1);
            } else {
                return;
            }
        }
    }
}
