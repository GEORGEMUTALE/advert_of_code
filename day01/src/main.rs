// use std::fs;

// fn main() {
//     // Read the input file
//     let input = fs::read_to_string("../day01.txt").expect("Failed to read input file");
    
//     // Split the input into two lists
//     let (left, right): (Vec<i32>, Vec<i32>) = input
//         .lines()
//         .map(|line| {
//             let mut parts = line.split_whitespace();
//             (
//                 parts.next().unwrap().parse::<i32>().unwrap(),
//                 parts.next().unwrap().parse::<i32>().unwrap(),
//             )
//         })
//         .unzip();

//     // Sort both lists
//     let mut left_sorted = left.clone();
//     let mut right_sorted = right.clone();
//     left_sorted.sort();
//     right_sorted.sort();

//     // Calculate the total distance
//     let total_distance: i32 = left_sorted
//         .iter()
//         .zip(right_sorted.iter())
//         .map(|(l, r)| (l - r).abs())
//         .sum();

//     // Print the result
//     println!("Total distance: {}", total_distance);
// }
use std::collections::HashMap;
use std::fs;

fn main() {
    // Read the input file
    let input = fs::read_to_string("../day012.txt").expect("Failed to read input file");

    // Parse the input into two lists
    let (left, right): (Vec<i32>, Vec<i32>) = input
        .lines()
        .map(|line| {
            let mut parts = line.split_whitespace();
            (
                parts.next().unwrap().parse::<i32>().unwrap(),
                parts.next().unwrap().parse::<i32>().unwrap(),
            )
        })
        .unzip();

    // Create a frequency map for the right list
    let mut frequency_map: HashMap<i32, i32> = HashMap::new();
    for &num in &right {
        *frequency_map.entry(num).or_insert(0) += 1;
    }

    // Calculate the similarity score
    let similarity_score: i32 = left
        .iter()
        .map(|&num| num * frequency_map.get(&num).unwrap_or(&0))
        .sum();

    // Print the result
    println!("Similarity score: {}", similarity_score);
}
