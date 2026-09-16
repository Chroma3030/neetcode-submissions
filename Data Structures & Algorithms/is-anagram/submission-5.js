class Solution {
    /**
     * @param {string} s
     * @param {string} t
     * @return {boolean}
     */
    isAnagram(s, t) {
        if (s.length != t.length) return false;
        let alphabet = new Array(26).fill(0);
        for (let i = 0; i < t.length ; i++) {
            alphabet[t.charCodeAt(i) - 97] ++
            alphabet[s.charCodeAt(i) - 97] -- 
        }
        return alphabet.every(x => x === 0);
    }
}
