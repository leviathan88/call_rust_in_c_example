use std::collections::HashMap;
pub struct Interleave<'h> {
    a: &'h str,
    b: &'h str,
    c: &'h str,
}

// Public methods
impl<'h> Interleave<'h> {
    pub fn new(a: &'h str, b: &'h str, c: &'h str) -> Self {
        Interleave { a, b, c }
    }

    pub fn solve(&self) -> bool {
        let mut memo = HashMap::new();

        if self.a.len() + self.b.len() != self.c.len() {
            return false;
        }

        self.aux(0, 0, 0, &mut memo)
    }
}

// Private methods
impl<'h> Interleave<'h> {
    fn aux(
        &self,
        a_index: usize,
        b_index: usize,
        c_index: usize,
        memo: &mut HashMap<String, bool>,
    ) -> bool {
        let key = Self::generate_key(a_index, b_index, c_index);
        let key_clone = key.clone();

        if memo.contains_key(&key) {
            *memo.get(&key).unwrap()
        } else {
            let a_temp = Self::get_char_or(a_index, self.a, "-1");
            let b_temp = Self::get_char_or(b_index, self.b, "-2");
            let c_temp = Self::get_char_or(c_index, self.c, "-3");

            if a_index >= self.a.len() && b_index >= self.b.len() && c_index >= self.c.len() {
                memo.insert(key_clone, true);
            } else if a_temp == c_temp && b_temp == c_temp {
                let result = self.aux(a_index + 1, b_index, c_index + 1, memo)
                    || self.aux(a_index, b_index + 1, c_index + 1, memo);
                memo.insert(key_clone, result);
            } else if a_temp == c_temp {
                let result = self.aux(a_index + 1, b_index, c_index + 1, memo);
                memo.insert(key_clone, result);
            } else if b_temp == c_temp {
                let result = self.aux(a_index, b_index + 1, c_index + 1, memo);
                memo.insert(key_clone, result);
            } else {
                memo.insert(key_clone, false);
            }
            *memo.get(&key).unwrap()
        }
    }

    fn get_char_or(index: usize, s: &'h str, or: &'h str) -> &'h str {
        if index < s.len() {
            &s[index..index + 1]
        } else {
            or
        }
    }

    fn generate_key(a_index: usize, b_index: usize, c_index: usize) -> String {
        format!("{}-{}-{}", a_index, b_index, c_index)
    }
}

#[cfg(test)]
mod tests {

    use super::*;

    #[test]
    fn test_interleave_long() {
        let s1 = "aabcc";
        let s2 = "dbbca";
        let s3 = "aadbbcbcac";

        let interleave = Interleave::new(s1, s2, s3);
        assert_eq!(true, interleave.solve());
    }

    #[test]
    fn test_interleave_short() {
        let s1 = "a";
        let s2 = "b";
        let s3 = "a";

        let interleave = Interleave::new(s1, s2, s3);
        assert_eq!(false, interleave.solve());
    }
}
