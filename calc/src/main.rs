fn sum(a: i32, b: i32) -> i32 {
    a + b
}
fn main() {
    let a = 3;
    let b = 8;
    println!("{a} + {b} = {}", sum(a,b));
}

#[test]

fn sum_work() {
    let sum = sum(15, 3);
    assert!(sum == 18);
}

