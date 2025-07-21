#[derive(Default, Debug)]
pub struct Scanner(Vec<String>);
macro_rules! methods {
    ($(($get:ident, $get_n:ident): $ty:ty),*) => {
        $(
            pub fn $get(&mut self) -> $ty {
                self.next()
            }
            pub fn $get_n<const N: usize>(&mut self) -> [$ty; N] {
                std::array::from_fn(|_| self.next())
            }
        )*
    }
}
impl Scanner {
    fn next<T: std::str::FromStr>(&mut self) -> T {
        loop {
            if let Some(c) = self.0.pop() {
                return c.parse().ok().unwrap();
            }
            let mut s = String::new();
            std::io::stdin().read_line(&mut s).unwrap();
            self.0 = s.split_whitespace().rev().map(String::from).collect();
        }
    }
    methods!(
        (num, nums): usize,
        (int, ints): isize,
        (string, strings): String,
        (char, chars): char,
        (float, floats): f64
    );
}

fn main() {
    let mut cin = Scanner::default();
    let t = cin.num();
    for _ in 0..t {
        let n = cin.num();
        let mut ans = 0;
        for _ in 0..n {
            let [a, b, c, d] = cin.nums();
            if a > c && b > d {
                ans += a - c;
                ans += b - d;
                ans += c;
            } else if a > c {
                ans += a - c;
            } else if b > d {
                ans += a;
                ans += b - d;
            } else {
                continue;
            }
        }
        println!("{}", ans);
    }
}
