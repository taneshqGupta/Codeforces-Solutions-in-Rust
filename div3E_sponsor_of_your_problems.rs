struct Scanner(Vec<String>);
impl Scanner {
    fn new() -> Self {
        let input = std::io::read_to_string(std::io::stdin()).unwrap();
        Scanner(input.split_whitespace().map(String::from).rev().collect())
    }
    fn next<T: std::str::FromStr>(&mut self) -> T {
        self.0.pop().unwrap().parse().ok().unwrap()
    }
}

fn main() {
    let mut cin = Scanner::new();
    let t: usize = cin.next();
    for _ in 0..t {
        let (l, r): (String, String) = (cin.next(), cin.next());
        let l: Vec<u8> = l.as_str().chars().map(|c| c as u8 - b'0').collect();
        let r: Vec<u8> = r.as_str().chars().map(|c| c as u8 - b'0').collect();
        
        // dbg!(l, r);
        let mut prefix_end = false;
        let mut prefix_len = 0;
        for i in 0..l.len() { // or r.len()
            if (l[i] == r[i]) && !prefix_end { 
                prefix_len += 1;    
                continue;
            }
            if (l[i] != r[i]) && !prefix_end {
                prefix_end = true;
                continue;
            }
        }
        if prefix_len == l.len() {
            drop(l);
            drop(r);
            println!("{}", 2 * prefix_len);
            continue;
        }

        if l[prefix_len].abs_diff(r[prefix_len]) > 1 {
            drop(l);
            drop(r);
            println!("{}", 2 * prefix_len);
            continue;
        }
        let mut ans = 2 * prefix_len + 1;
        for i in (prefix_len+1)..l.len() {
            if l[i] == 9 && r[i] == 0 {
                ans += 1;
            }
            else {
                break;
            }
        }
        drop(l);
        drop(r);
        println!("{}", ans);
    }
}
