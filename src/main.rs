mod week1;
mod week2;
mod week3;
mod week4;
use week1::karatsuba;
use week2::{inversion_input, inversion};
use week3::{quicksort_input, quicksort};
use week4::{contraction_input, contraction};

fn main() {
    // week1
    let num1 = String::from("3141592653589793238462643383279502884197169399375105820974944592");
    let num2 = String::from("2718281828459045235360287471352662497757247093699959574966967627");
    karatsuba(num1, num2);

    // week2
    let init_vec = inversion_input();
    let (_, inverted) = inversion(init_vec, 0);

    // week3
    let mut init_vec = quicksort_input();
    let l = 0; //initial position of pivot
    let r = init_vec.len(); // initial end of partition
    let mut c = [0]; // initial comparison
    let a = quicksort(&mut init_vec, l, r, &mut c);
    println!{"week3: final vec: {:?}", init_vec};
    println!{"week3 final count: {:?}", c};

    // week4
    let mut mincuts = Vec::new();
    let mut min = 99999;
    for i in 0..500 {
        let mut input = contraction_input();
        let mut mc = [0];
        let m = contraction(&mut input, &mut mc);
        mincuts.push(m);
        if m[0] < min {
            min = m[0];
        }

    }
    println!("week4: mincuts: {:?}", mincuts);
    println!("week4: min of mincuts: {:?}", min);

}
