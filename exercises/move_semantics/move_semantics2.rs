// move_semantics2.rs
//
// Expected output:
// vec0 has length 3, with contents `[22, 44, 66]`
// vec1 has length 4, with contents `[22, 44, 66, 88]`
//
// Execute `rustlings hint move_semantics2` or use the `hint` watch subcommand
// for a hint.


fn main() {
    let mut vec0 = Vec::new();

<<<<<<< HEAD
    let mut vec1 = fill_vec(vec0);
=======
    // Do not move the following line!
    fill_vec(&mut vec0);
>>>>>>> a125963

    println!("{} has length {}, with contents: `{:?}`", "vec0", vec0.len(), vec0);

<<<<<<< HEAD
    vec1.push(88);

    println!("{} has length {}, with contents `{:?}`", "vec1", vec1.len(), vec1);
=======
>>>>>>> a125963
}

fn fill_vec(vec: &mut Vec<i32>) {

    vec.push(22);
    vec.push(44);
    vec.push(66);
}
