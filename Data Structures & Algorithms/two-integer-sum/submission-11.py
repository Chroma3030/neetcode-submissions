class Solution:
    def twoSum(self, nums: List[int], target: int) -> List[int]:
        pairs = [(n, i) for i, n in enumerate(nums)];
        pairs.sort(key=lambda p: p[0]);
        l, r = 0, len(pairs) - 1;
        while l < r:
            attempt = pairs[l][0] + pairs[r][0];
            if target == attempt:
                a = pairs[l][1]
                b = pairs[r][1]
                return [min(a, b), max(a, b)];
            elif target < attempt:
                r -= 1;
            else:
                l += 1;
        return []

