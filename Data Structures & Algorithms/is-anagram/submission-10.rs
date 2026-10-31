impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if t.len() != s.len() {
            return false
        }
        let mut char_list: Vec<char> = t.chars().collect();

        for ch in s.chars() {
            match char_list.iter().position(|x| *x == ch) {
                Some(i) => {
                    char_list.remove(i);
                },
                _ => return false
            }
        }
        true
    }
}
