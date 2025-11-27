fn expand_from_center(chars: &[char], left: isize, right: isize) -> usize {
    let mut l = left;
    let mut r = right;
    let len = chars.len() as isize;

    while l >= 0 && r < len && chars[l as usize] == chars[r as usize] {
        l -= 1;
        r += 1;
    }

    (r - l - 1) as usize // panjang palindrome
}

fn longest_palindrome(s: &str) -> String {
    let chars: Vec<char> = s.chars().collect();
    let n = chars.len();
    if n == 0 { return "".to_string(); }

    let mut start = 0usize;
    let mut end = 0usize;

    for i in 0..n {
        let len1 = expand_from_center(&chars, i as isize, i as isize);       // odd
        let len2 = expand_from_center(&chars, i as isize, i as isize + 1);   // even
        let length = len1.max(len2);

        if length > end - start {
            start = i - (length - 1) / 2;
            end   = i + length / 2;
        }
    }

    chars[start..=end].iter().collect()
}

fn main() {
    println!("{}", longest_palindrome("babad"));                         // bab atau aba
    println!("{}", longest_palindrome("A man, a plan, a canal: Panama")); // amanakanama
}
