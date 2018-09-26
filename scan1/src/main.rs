use std::io::BufRead;

#[allow(unused_macros)]
macro_rules! scan {
    (!! ($ws:expr);
        $([ $yv:ident : $yt:ty ]),*
        $(( $($xv:ident : $xt:ty),* )),*
    ) => {
        $(let $yv : Vec<$yt> = {
            $ws.map(|w| w.parse().unwrap()).collect()
        };)*
        $($(let $xv;)* {
            let mut ws = $ws;
            $($xv = ws.next().unwrap().parse::<$xt>().unwrap();)*
        })*
    };
    (!! ($ws:expr) { $zv:ident : $zn:expr };
        $([ $yv:ident : $yt:ty ]),*
        $(( $($xv:ident : $xt:ty),* )),*
    ) => {
        let mut $zv = vec![];
        for _ in 0..$zn {
            $(scan!(!! ($ws); [ $yv : $yt ]); $zv.push($yv);)*
            $(scan!(!! ($ws); ($($xv : $xt),*)); $zv.push(($($xv),*));)*
        }
    };
    ($(
        $([ $yv:ident : $yt:ty ]),*
        $(( $($xv:ident : $xt:ty),* )),*
        $({ $zv:ident : $zn:expr }),*
    ;)+) => {
        let stdin = std::io::stdin();
        let mut stdin = std::io::BufReader::new(stdin.lock());
        let mut line = String::new();
        $(scan!{
            !! ({
                line.clear();
                stdin.read_line(&mut line).unwrap();
                line.split_whitespace()
            }) $({ $zv : $zn })*;
            $([ $yv : $yt ]),*
            $(( $($xv : $xt),* )),*
        })*
    };
}

fn main() {
    scan!{
        (a: i32);
        (b: i32, c: i64);
        (s: String);
        (k: usize);
        [x: i64];
        (h: usize, w: usize);
        (r: String) {board: h};
        (n: usize, m: usize);
        (u: usize, v: usize, w: i64) {edges: m};
    }

    let check = ((a ^ b) as i64
        ^ s.as_bytes()[0] as i64
        ^ x[k - 1]
        ^ board[h - 1].as_bytes()[w - 1] as i64
        ^ n as i64
        ^ (edges[m - 1].0 ^ edges[m - 1].1) as i64
        ^ edges[m - 1].2) % 128 ^ c;
    assert_eq!(check, 0);
}
