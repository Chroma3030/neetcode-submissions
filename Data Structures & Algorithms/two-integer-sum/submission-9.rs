impl Solution {
    pub fn two_sum(nums: Vec<i32>, target: i32) -> Vec<i32> {
        let mut pairs: Vec<(i32, i32)> = nums
        .iter()
        .enumerate()
        .map(|(i, &n)| (n, i as i32))
        .collect();

        pairs.sort_unstable_by_key(|&(n, _)| n);

        let mut l = 0;
        let mut r = nums.len() - 1;

        while l < r {
            let attempt = pairs[l].0 as i64 + pairs[r].0 as i64;

            match attempt.cmp(&(target as i64)) {
                Ordering::Equal => return vec![pairs[l].1.min(pairs[r].1), pairs[l].1.max(pairs[r].1)],
                Ordering::Greater => r -= 1,
                Ordering::Less => l += 1,
            }
        }
        unreachable!()
    }
}
