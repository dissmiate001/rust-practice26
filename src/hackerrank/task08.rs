// https://www.hackerrank.com/challenges/breaking-best-and-worst-records/problem

pub fn breaking_records(scores: &[i32]) -> (i32, i32) {
    let mut max = scores[0];
    let mut min = scores[0];

    let mut max_count = 0;
    let mut min_count = 0;

    for &x in scores {
        if x > max {
            max = x;
            max_count += 1;
        }

        if x < min {
            min = x;
            min_count += 1;
        }
    }

    (max_count, min_count)
}

#[test]
fn test0() {
    let scores = vec![10, 5, 20, 20, 4, 5, 2, 25, 1];

    let real = breaking_records(&scores);

    let expected = (2, 4);

    assert_eq!(real, expected);
}
