// https://www.hackerrank.com/challenges/between-two-sets/problem

fn gcd(mut a: i32, mut b: i32) -> i32 {
    while b != 0 {
        let t = b;
        b = a % b;
        a = t;
    }
    a
}

fn lcm(a: i32, b: i32) -> i32 {
    a * b / gcd(a, b)
}

pub fn get_total_x(a: &[i32], b: &[i32]) -> i32 {
    let mut l = a[0];

    for &x in a {
        l = lcm(l, x);
    }

    let mut g = b[0];

    for &x in b {
        g = gcd(g, x);
    }

    let mut count = 0;
    let mut i = l;

    while i <= g {
        if g % i == 0 {
            count += 1;
        }
        i += l;
    }

    count
}

#[test]
fn test0() {
    let a = vec![2, 4];
    let b = vec![16, 32, 96];

    let real = get_total_x(&a, &b);

    let expected = 3;

    assert_eq!(real, expected);
}
