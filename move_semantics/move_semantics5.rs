// move_semantics5.rs
// Make me compile only be reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` for hints :)


fn main() {
    let mut x = 100;
    let y = &mut x;
    // let z = &mut x;
    *y += 100;
    *y += 1000;
    assert_eq!(x, 1200);
}
