#[macro_use]
use uint;

uint::construct_uint! {
	pub struct U512 (8);
}

// ignoring cases with uneven lengths, less than length 2
pub fn karatsuba(mut _num1_string: String, mut _num2_string: String) -> U512 {

    if _num1_string.len() > _num2_string.len() {
        let mut padding2 = String::with_capacity(_num1_string.len()-_num2_string.len());
        padding2.replace_range(..padding2.len(), "0");
        _num2_string.insert_str(0, &padding2);
    } else if _num2_string.len() > _num1_string.len() {
        let mut padding1 = String::with_capacity(_num2_string.len()-_num1_string.len());
        padding1.replace_range(..padding1.len(), "0");
        _num1_string.insert_str(0, &padding1);
    }
    let input1_len = _num1_string.len();
    let input2_len = _num2_string.len();

    //base case
    if input1_len == 1 && input2_len == 1 {

        let a = string_to_uint(_num1_string.clone());
        let b = string_to_uint(_num2_string.clone());
        let base_ans = a*b;
        base_ans

    } else {

        let b = _num1_string.split_off(input1_len / 2 as usize);
        let a = _num1_string;
        
        let d = _num2_string.split_off(input1_len / 2 as usize);
        let c = _num2_string;

        let a_plus_b = string_to_uint(a.clone()) + string_to_uint(b.clone());
        let c_plus_d = string_to_uint(c.clone()) + string_to_uint(d.clone());

        let step1 = karatsuba(a.clone(), c.clone());
        let step2 = karatsuba(b.clone(), d.clone());

        let step3 = karatsuba(a_plus_b.to_string(), c_plus_d.to_string());

        let gauss = step3 - step1 - step2;
        let mut l = input1_len;
        if input1_len % 2 != 0 {
            l = l + 1usize;
        }
        let len = string_to_uint(String::from(l.to_string()));


        let power = string_to_uint(String::from("10"));

        let ans = power.pow(len)*step1 + power.pow(len / 2)*gauss + step2;
        
        ans
    
    }
}

pub fn string_to_uint(input: String) -> U512 {
    let p = U512::from_dec_str(&input).unwrap();
    p
}

