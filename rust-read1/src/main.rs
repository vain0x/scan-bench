use std::io::stdin;

#[allow(unused_macros)]
macro_rules! read {
    ([$t:ty] ; $n:expr) =>
        ((0..$n).map(|_| read!([$t])).collect::<Vec<_>>());
    ($($t:ty),+ ; $n:expr) =>
        ((0..$n).map(|_| read!($($t),+)).collect::<Vec<_>>());
    ([$t:ty]) =>
        (rl().split_whitespace().map(|w| w.parse().unwrap()).collect::<Vec<$t>>());
    ($t:ty) =>
        (rl().parse::<$t>().unwrap());
    ($($t:ty),*) => {{
        let buf = rl();
        let mut w = buf.split_whitespace();
        ($(w.next().unwrap().parse::<$t>().unwrap()),*)
    }};
}

#[allow(dead_code)]
fn rl() -> String {
    let mut buf = String::new();
    stdin().read_line(&mut buf).unwrap();
    buf.trim_right().to_owned()
}

fn main() {
    let a = read!(i32);
    let (b, c) = read!(i32, i64);
    let s = rl();
    let k = read!(usize);
    let x = read![[i64]];
    let (h, w) = read!(usize, usize);
    let board = read![String; h];
    let (n, m) = read!(usize, usize);
    let edges = read![usize, usize, i64; m];

    let check = ((a ^ b) as i64
        ^ s.as_bytes()[0] as i64
        ^ x[k - 1]
        ^ board[h - 1].as_bytes()[w - 1] as i64
        ^ n as i64
        ^ (edges[m - 1].0 ^ edges[m - 1].1) as i64
        ^ edges[m - 1].2) % 128 ^ c;
    assert_eq!(check, 0);
}
