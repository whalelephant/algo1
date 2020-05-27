use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn get_input() -> Vec<u32> {
    let path = Path::new("./week2/input.txt");
    let display = path.display();
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };
    let reader = BufReader::new(&file);
    let oklines = reader.lines();
    let input: Vec<u32>= oklines.map(|l| l.unwrap().parse::<u32>().unwrap()).collect();
    input

}

pub fn inversion(mut input_vec: Vec<u32>, mut sum: u128 ) -> (Vec<u32>, u128) {
    // base case
    if input_vec.len() <= 1 {
        return (input_vec, 0);
    } else {
        let mut rhs = input_vec.split_off(input_vec.len() / 2);
        let mut lhs = input_vec.clone();
        
        let rhs_r  =  inversion(rhs, 0);
        let lhs_r = inversion(lhs, 0);

        let mut rhs =rhs_r.0;
        let mut rhs_sum = rhs_r.1;

        let mut lhs =lhs_r.0;
        let mut lhs_sum = lhs_r.1;
        
        let mut sorted = Vec::new();
        let mut sum = rhs_sum + lhs_sum;
        
        let mut i = 0_usize;
        let mut j = 0_usize;
        
        while i <lhs.len() && j < rhs.len() {
            if lhs[i] < rhs[j] {
                sorted.push(lhs[i]);
                i += 1;
            } else {
                sorted.push(rhs[j]);
                j += 1;
                sum = sum + lhs.len() as u128 - i as u128;
            }
        } 

        if i < lhs.len() {
            sorted.append(&mut lhs.split_off(i))
        } 
        if j < rhs.len() {
            sorted.append(&mut rhs.split_off(j))
        }
        return (sorted, sum);
    }
}
