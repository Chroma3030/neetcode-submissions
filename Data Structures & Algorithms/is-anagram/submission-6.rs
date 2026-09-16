impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if s.len() != t.len() {
            return false;
        };

        let mut a: Vec<char> = t.chars().collect();
        let mut b: Vec<char> = s.chars().collect();

        a.sort_unstable();
        b.sort_unstable();

        a == b
    }
}
