// clippy2.rs
// Make me compile! Execute `rustlings hint clippy2` for hints :)

fn main() {
    let mut res = 42;
    let option = Some(12);
    if let Some(x) = option {
        res += x;
    }
    println!("{}", res);

    let option2 = [Some(1), Some(2), Some(3)];
    for x in option2.iter() {
        res += x.unwrap();
    }
}
