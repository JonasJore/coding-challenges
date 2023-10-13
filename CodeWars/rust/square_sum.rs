fn square_sum(vec: Vec<i32>) -> i32 {
    return vec.into_iter().map(|num| num.pow(2)).sum();
}

fn main() {
    let vec: Vec<i32> = vec![5, 3, 4];
    let s = square_sum(vec);
    println!("{}", s);

    tests();
}

fn tests() {
    assert_eq!(square_sum(vec![1, 2]), 5);
    assert_eq!(square_sum(vec![-1, -2]), 5);
    assert_eq!(square_sum(vec![5, 3, 4]), 50);
    assert_eq!(square_sum(vec![]), 0);
}
