// move_semantics5.rs
// Make me compile only by reordering the lines in `main()`, but without
// adding, changing or removing any of them.
// Execute `rustlings hint move_semantics5` or use the `hint` watch subcommand for a hint.

fn main() {
    let mut x = 100;
    let y = &mut x;
    *y += 100; // I understand that move occurs here and that's why we can later create the new
               // reference - i.e. because y goes out of scope here.
    let z = &mut x;
    *z += 1000;
    assert_eq!(x, 1200);
}
