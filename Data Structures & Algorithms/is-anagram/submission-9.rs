impl Solution {
    pub fn is_anagram(s: String, t: String) -> bool {
        if t.len() != s.len() {
            return false
        }
        let mut char_map: HashMap<char, i32> = HashMap::new();
        for ch in t.chars() {
            *char_map.entry(ch).or_insert(0) += 1
        }
        for ch in s.chars() {
            match char_map.get_mut(&ch) {
                Some(n) if *n > 0 => *n -= 1,
                _ => return false
                
            }
        }
        true

        
    }
}
