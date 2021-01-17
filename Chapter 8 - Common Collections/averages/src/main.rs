#![feature(map_into_keys_values)]
use std::{collections::HashMap, io};

fn main() {
    let mut arr: Vec<i64> = Vec::new();
    // let mut arr = vec![1, 2, 3, 4, 5, 5];
    loop {
        println!("Please input a number:");
        let mut numbers = String::new();
        io::stdin()
            .read_line(&mut numbers)
            .expect("Failed to read line");

        let integers: i64 = match numbers.trim().parse() {
            Ok(numbers) => numbers,
            Err(_) => break,
        };
        arr.push(integers);
    }

    let m = mean(&arr);

    println!("The average value of arr is {}", m);
    median(&mut arr);
    mode(&arr);
}

fn mean(m: &Vec<i64>) -> f64 {
    let mut increment: i64 = 0;
    for i in m.iter() {
        increment += i;
    }
    increment as f64 / m.len() as f64
}

fn mode(m: &Vec<i64>) {
    let mut hmap: HashMap<i64, i64> = HashMap::new();
    for i in m {
        *hmap.entry(*i).or_insert(0) += 1;
    }
    let vec: Vec<i64> = hmap.clone().into_values().collect();
    println!("{:?}", vec);
    let sum: i64 = vec.iter().sum();
    if sum == vec.len() as i64 {
        println!("All values appeared just once.")
    } else {
        println!(
            "The mode is {}",
            hmap.into_iter()
                .max_by_key(|(_, v)| *v)
                .map(|(k, _)| k)
                .unwrap_or_default()
        )
    }
}

fn median(m: &mut Vec<i64>) {
    m.sort();
    let mid = m.len() / 2;
    if m.len() % 2 > 0 {
        println!("The median is {}", m[mid]);
    } else if m.len() % 2 == 0 {
        println!(
            "The median is {}",
            (m[mid - 1] as f64 + m[mid] as f64) / 2.0
        );
    }
}
