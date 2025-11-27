fn longest_palindrome_ascii_ci(s: &str) -> &str {
    let b = s.as_bytes();
    let n = b.len();
    if n == 0 { return ""; }

    let mut left = 0;
    let mut right = 1;

    let expand = |b: &[u8], mut l: isize, mut r: isize| {
        while l >= 0 && (r as usize) < b.len() {
            let lc = b[l as usize].to_ascii_lowercase();
            let rc = b[r as usize].to_ascii_lowercase();

            if lc != rc {
                break;
            }

            l -= 1;
            r += 1;
        }

        ((l + 1) as usize, (r - 1) as usize + 1)
    };

    for i in 0..n {
        for &(l, r) in &[
            expand(b, i as isize, i as isize),
            expand(b, i as isize, i as isize + 1),
        ] {
            if r - l > right - left {
                left = l;
                right = r;
            }
        }
    }

    &s[left..right]
}

fn main() {
    println!("{}", longest_palindrome_ascii_ci("mantan teman duet maya estianti adalah pinkan mambo"));
    println!("{}", longest_palindrome_ascii_ci("Level Ousmane Dembele sekarang ini sudah setara dengan Rodri"));
}
