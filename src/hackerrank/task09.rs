// https://www.hackerrank.com/challenges/migratory-birds/problem

pub fn migratory_birds(arr: &[i32]) -> i32 {
    let mut count = vec![0; 6];

    for &x in arr {
        count[x as usize] += 1;
    }

    let mut best = 1;

    for i in 2..6 {
        if count[i] > count[best] {
            best = i;
        }
    }

    best as i32
}

#[test]
fn test0() {
    let arr = vec![1, 4, 4, 4, 5, 3];

    let real = migratory_birds(&arr);

    let expected = 4;

    assert_eq!(real, expected);
}
