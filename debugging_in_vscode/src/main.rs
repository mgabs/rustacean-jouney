fn main() {
    let x: i32 = 10;
    let _y = "#ff0000";
    println!("Hello, world {}!", x);
}

#[cfg(tests)]

#[test]
fn main_is_ok1() {
    assert_eq!(1, 1);
}

#[test]
fn main_is_ok2() {
    assert_eq!(2, 2);
}

#[test]
fn main_is_ok3() {
    assert_eq!(2, 2);
}