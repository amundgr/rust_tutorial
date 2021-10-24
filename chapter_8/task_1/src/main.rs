use std::collections::HashMap;

fn main() {
    let int_vec = vec!(1,2,3,4,4,5,6,7,8,9);
    let mean = calc_mean(&int_vec);
    println!("{}", mean);
    let median = calc_median(&int_vec);
    println!("{}", median);
    let mode = calc_mode(&int_vec);
    println!("{:?}", mode);
}

fn calc_mean(int_vec : &[i32]) -> f32 {
    let mut sum = 0;

    for i in int_vec {
        sum += i;
    }
    sum as f32 / int_vec.len() as f32
}

fn calc_median(int_vec : &[i32]) -> f32{
    let mut int_vec_sorted = int_vec.to_vec();
    int_vec_sorted.sort();
    let vec_len = int_vec.len();

    if vec_len % 2 > 0 {
        return int_vec_sorted[vec_len / 2] as f32;
    }
    (int_vec_sorted[vec_len / 2] as f32 + int_vec_sorted[vec_len / 2 - 1] as f32) / 2.0
}

fn calc_mode(int_vec : &[i32]) -> HashMap<i32, i32> {
    let mut number_occ = HashMap::new();
    let mut highest_occ = 0;

    for number in int_vec {
        let count = number_occ.entry(*number).or_insert(0);
        *count += 1;
        if *count > highest_occ {
            highest_occ = *count;
        }
    }

    let mut highest_occ_vec = HashMap::new();

    for (number, occ) in &number_occ {
        if *occ == highest_occ {
            highest_occ_vec.insert(*number, *occ);
        }
    }

    return highest_occ_vec;
}