use crate::c128::{Complex, IN2P, TPI, ZERO};

pub fn index(n: i32) -> usize {
    (2 * n.abs() - ((n.signum() == 1) as i32)) as usize
}

// includes c(-n)..c(-1), c(0), c(1)..c(n) len = 2*n+1
pub fn transform(path: Vec<Complex>, depth: usize) -> Vec<Complex> {
    let n = path.len();

    let mut c = vec![ZERO; 2 * depth + 1];

    let plen = {
        let mut plen = 0.0;
        for i in 1..n {
            plen += (path[i] - path[i - 1]).abs();
        }
        plen
    };

    let times = {
        let mut times = vec![0.0; n];

        for i in 1..n {
            times[i] += times[i - 1] + (path[i] - path[i - 1]).abs() / plen;
        }
        times
    };

    let p = {
        let mut p = vec![ZERO; n - 1];

        p[0] = (path[n - 1] - path[n - 2]) / (times[n - 1] - times[n - 2])
            - (path[1] - path[0]) / (times[1] - 0.0);

        for i in 1..p.len() {
            p[i] = (path[i] - path[i - 1]) / (times[i] - times[i - 1])
                - (path[i + 1] - path[i]) / (times[i + 1] - times[i]);

            if p[i].is_nan() {
                p[i] = ZERO;
            }
        }

        p
    };

    c[index(0)] = (0..n - 1).fold(ZERO, |v, i| {
        v + (path[i + 1] + path[i]) * (times[i + 1] - times[i]) * 0.5
    });

    let e1: Vec<Complex> = (0..n - 1).map(|i| Complex::ei(-TPI * times[i])).collect();

    let mut e = e1.clone();

    for m in 1..depth + 1 {
        let m = m as i32;
        let k = IN2P / (m as f64);

        c[index(m)] =
            k * (path[n - 1] - path[0]) + k * k * (0..n - 1).fold(ZERO, |s, i| s + (p[i] * e[i]));

        c[index(-m)] = -k * (path[n - 1] - path[0])
            + k * k * (0..n - 1).fold(ZERO, |s, i| s + (p[i] * e[i].conj()));

        for i in 0..n - 1 {
            e[i] = e[i] * e1[i];
        }
    }

    c
}
