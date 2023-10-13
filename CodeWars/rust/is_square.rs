fn is_square(n: i64) -> bool {
    let result: fn(i64) -> bool = |d| {
        let sqrt = (d as f64).sqrt();
        return sqrt * sqrt == (d as f64);
    };

    return result(n);
}

fn main() {
    println!("{}", is_square(637532408400745));
    fixed_tests();
}

fn fixed_tests() {
    assert_eq!(
        is_square(-1),
        false,
        "\nYour answer (left) is not the expected answer (right)."
    );
    assert_eq!(
        is_square(0),
        true,
        "\nYour answer (left) is not the expected answer (right)."
    );
    assert_eq!(
        is_square(3),
        false,
        "\nYour answer (left) is not the expected answer (right)."
    );
    assert_eq!(
        is_square(4),
        true,
        "\nYour answer (left) is not the expected answer (right)."
    );
    assert_eq!(
        is_square(25),
        true,
        "\nYour answer (left) is not the expected answer (right)."
    );
    assert_eq!(
        is_square(26),
        false,
        "\nYour answer (left) is not the expected answer (right)."
    );
    println!("Tests ran successfully");
}
