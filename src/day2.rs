use std::io::Result;
use std::{fs::File, io::Read};

fn get_data() -> Result<Vec<Vec<i8>>> {
    let mut data_file = File::open("../data/day2.txt").expect("File not valid!");
    let mut data_string = String::new();
    data_file
        .read_to_string(&mut data_string)
        .expect("Could not parse into string");

    let mut reports: Vec<Vec<i8>> = Vec::new();

    for line in data_string.split("\n") {
        if line.is_empty() {
            continue;
        }

        let nums: Vec<i8> = line
            .split(" ")
            .map(|n_str| n_str.parse::<i8>().expect("Failed to parse integer!"))
            .collect();

        reports.push(nums);
    }

    Ok(reports)
}

fn part1(reports: &Vec<Vec<i8>>) {
    let mut safe_reports: i32 = 0;

    for r in reports {
        let mut is_safe = true;
        let mut running_sum = 0;
        let overall_increasing = r.get(0).unwrap() < r.get(1).unwrap();

        for i in 1..(r.len()) {
            let n_prev = r
                .get(i - 1)
                .expect(format!("Could not get index i-1 = {}", i - 1).as_str());
            let n = r.get(i).expect(
                format!("Could not get index i = {}, report length = {}", i, r.len()).as_str(),
            );

            let diff = n - n_prev;

            // First check the differences between numbers
            if diff.abs() > 3 || diff.abs() == 0 {
                println!(
                    "Report {:?} is unsafe due to difference! n = {} - n_prev = {} - diff = {}",
                    r, n, n_prev, diff
                );
                is_safe = false;
                break;
            } else {
                println!(
                    "Report {:?} is safe for difference! n = {} - n_prev = {} - diff = {}",
                    r, n, n_prev, diff
                );
            }

            // Second, ensure all numbers are decreasing or increasing
            // This works using a running sum and absolute value. We test adding the number to the sum; if it gets further from zero,
            // we're all clear. If it gets closer to zero, the order has been reversed and it fails.
            // if running_sum == 0 || ((running_sum + diff) <= running_sum.abs()) {
            //     println!("Report {:?} is unsafe due to invalid increasing or decreasing! running_sum =  {}  - diff = {}\n\n", r, running_sum, diff );
            //     is_safe = false;
            //     break;
            // } else {
            //     println!(
            //         "Report {:?} is safe for increasing/decreasing! running_sum =  {}  - diff = {}\n\n",
            //         r, running_sum, diff
            //     );
            //     running_sum += diff;
            // }
            let curr_increasing = r.get(i - 1).unwrap() < r.get(i).unwrap();

            if curr_increasing != overall_increasing {
                is_safe = false;
                break;
            }
        }

        safe_reports += if is_safe {
            println!("Report {:?} is safe!", r);
            1
        } else {
            0
        }
    }

    println!("Part 1: Safe Reports {}", safe_reports)
}

pub fn solution() {
    println!("Day 2");
    let reports = get_data().unwrap();

    part1(&reports);
}
