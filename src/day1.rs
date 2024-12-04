use std::io::Result;
use std::{env, fs::File, io::Read};

struct DataGetter {
    left: Vec<i32>,
    right: Vec<i32>,
}

fn get_data() -> Result<DataGetter> {
    let mut data_file = File::open("../data/day1.txt").expect("File not valid!");
    let mut data_string = String::new();
    data_file
        .read_to_string(&mut data_string)
        .expect("Could not parse into string");

    let mut left: Vec<i32> = Vec::new();
    let mut right: Vec<i32> = Vec::new();

    for line in data_string.split("\n") {
        if line.is_empty() {
            continue;
        }

        let nums: Vec<i32> = line
            .split("   ")
            .map(|n_str| n_str.parse::<i32>().expect("Failed to parse integer!"))
            .collect();

        left.push(nums[0]);
        right.push(nums[1]);
    }

    left.sort();
    right.sort();

    Ok(DataGetter { left, right })
}

fn part1(left: &Vec<i32>, right: &Vec<i32>) -> Result<()> {
    let mut total_distance = 0;

    for i in 0..left.len() {
        total_distance += (left.get(i).unwrap() - right.get(i).unwrap()).abs();
    }

    println!("Part 1: Total Distance: {}", total_distance);

    Ok(())
}

fn part2(left: &Vec<i32>, right: &Vec<i32>) -> Result<()> {
    let mut sim_score = 0;

    for n1 in left {
        let mut multiplier = 0;

        for n2 in right {
            if n1 == n2 {
                multiplier += 1;
            }
        }

        sim_score += n1 * multiplier;
    }

    println!("Part 2: Similarity Score {}", sim_score);

    Ok(())
}

pub fn solution() {
    let data = get_data().unwrap();

    let _ = part1(&data.left, &data.right);
    let _ = part2(&data.left, &data.right);
}
