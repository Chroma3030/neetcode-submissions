class Solution:
    def isAnagram(self, s: str, t: str) -> bool:
        if len(s) != len(t): return False
        slots = [0] * 26;
        for a, b in zip(t, s):
            slots[ord(a) - 97] += 1;
            slots[ord(b) - 97] -= 1;
        return all(x == 0 for x in slots);
        