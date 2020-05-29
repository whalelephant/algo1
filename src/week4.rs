use rand::prelude::*;
use std::collections::HashMap;
use std::fs::File;
use std::io::prelude::*;
use std::io::BufReader;
use std::path::Path;

pub fn contraction_input() -> HashMap<u32, Vec<u32>> {
    let path = Path::new("./week4/testcases.txt");
    let file = match File::open(&path) {
        Err(_) => panic!("couldn't open {}"),
        Ok(file) => file,
    };
    let mut full_map = HashMap::new();
    let reader = BufReader::new(&file);
    for oklines in reader.lines() {
        let mut v = Vec::new();
        for value in oklines.unwrap().split(' ') {
            v.push(value.parse::<u32>().unwrap());
        }
        full_map.insert(v[0], v.clone());
    }
    full_map
}

pub fn contraction(arraymap: &mut HashMap<u32, Vec<u32>>, mc: &mut [u32; 1]) -> [u32; 1] {
    // base case
    if arraymap.len() <= 2 {
        let v1a = arraymap.values().next().unwrap();
        mc[0] = v1a.len() as u32 - 1 as u32;
        *mc
    } else {
        let keys = arraymap.keys().cloned().collect::<Vec<u32>>();
        // randomly select two vertex
        let mut rng = thread_rng();
        let v1 = keys.choose(&mut rng).unwrap();
        let v1a = arraymap.get(v1).unwrap();
        let y = rng.gen_range(1, v1a.len());
        let v2 = &v1a.clone()[y as usize];
        let v2a = arraymap.get(v2).unwrap();

        // make sure vertices are not unique and are connected
        if v1 != v2 {
            let mut new_v1a = v1a.clone();
            new_v1a.retain(|&v| v != *v2);

            let mut new_v2a = v2a.clone();
            new_v2a.retain(|&v| v != *v1);
            new_v2a.retain(|&v| v != *v2);
            for value in new_v2a.iter() {
                new_v1a.push(*value);

                // replace itself with v1 in other arrays
                let arr = arraymap.get_mut(value).unwrap();
                for v in arr.iter_mut() {
                    if v == v2 {
                        *v = *v1;
                    }
                }
            }

            arraymap.remove(v2).unwrap();
            arraymap.insert(*v1, new_v1a);

            contraction(arraymap, mc);
            *mc
        } else {
            contraction(arraymap, mc);
            *mc
        }
    }
}
