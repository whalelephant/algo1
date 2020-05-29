use std::error::Error;
use std::fs::File;
use std::io::BufReader;
use std::io::prelude::*;
use std::path::Path;

pub fn quicksort_input() -> Vec<u32> {
    let path = Path::new("./week3/input.txt");
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

pub fn quicksort(
    param_int_array: &mut [u32], 
    start: usize, 
    end: usize, 
    sum: &mut [u32])
{
    
    if start >= end {
        return;
    }

    // case 2
    // param_int_array.swap(start, end-1);

    // case 1
    let length = end - start;
    let mut mid = length/2;
    if length % 2 == 0 {
        mid -= 1;
    }

    if length > 2 {
        let mut arr = [param_int_array[start], param_int_array[start + mid as usize], param_int_array[end-1]];
    
        if arr[0] < arr[1] {
            if arr[1] < arr[2] {
                param_int_array.swap(start, start + mid as usize);
            } else if arr[0] < arr[2] {
                param_int_array.swap(start, end-1);
            }
        } else if arr[0] > arr[1] {
            if arr[1] > arr[2] {
                param_int_array.swap(start, start + mid as usize);
            } else if arr[0] > arr[2] {
                param_int_array.swap(start, end-1);
            }
        }
    }

    let pivot = partition(param_int_array, start, end);
    sum[0] = sum[0] + (end - start - 1) as u32;
    
    quicksort(param_int_array, start, (pivot - 1) as usize, sum); //lhs
    quicksort(param_int_array, pivot as usize, end, sum);   //rhs
}

fn partition(param_int_array: &mut [u32], l: usize, r: usize ) -> u32 {
    let pivot = param_int_array[l];
    let mut i = l+1;
    let mut j = l+1;
    while j < r {
        if param_int_array[j] < pivot {
            param_int_array.swap(j, i);
            i+=1;
        }
        j+=1;
    }
    param_int_array.swap(l, i-1);

    return i as u32;
}

// answers
// 162085
// 164123
// 138382