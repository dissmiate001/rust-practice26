// https://www.hackerrank.com/challenges/sock-merchant/problem

pub fn sock_merchant(arr: &[i32]) -> i32 {
    let mut count = vec![0; 101];

    for &x in arr {
        count[x as usize] += 1;
    }

    let mut pairs = 0;

    for c in count {
        pairs += c / 2;
    }

    pairs
}

#[test]
fn test0() {
    let arr = vec![10, 20, 20, 10, 10, 30, 50, 10, 20];

    let real = sock_merchant(&arr);

    let expected = 3;

    assert_eq!(real, expected);
}
