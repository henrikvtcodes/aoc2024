use std::io::Result;
use std::{fs::File, io::Read};

type Report = Vec<i8>;

fn get_data() -> Result<Vec<Report>> {
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

fn report_is_safe(r: &Report) -> bool {
    let mut is_safe = true;
    let overall_increasing = r.get(0).unwrap() < r.get(1).unwrap();

    for i in 1..(r.len()) {
        let n_prev = r
            .get(i - 1)
            .expect(format!("Could not get index i-1 = {}", i - 1).as_str());
        let n = r
            .get(i)
            .expect(format!("Could not get index i = {}, report length = {}", i, r.len()).as_str());

        let diff = n - n_prev;

        // First check the differences between numbers
        if diff.abs() > 3 || diff.abs() == 0 {
            is_safe = false;
            break;
        } else {
        }

        // Second, ensure all numbers are decreasing or increasing
        let curr_increasing = r.get(i - 1).unwrap() < r.get(i).unwrap();
        if curr_increasing != overall_increasing {
            is_safe = false;
            break;
        }
    }

    is_safe
}

fn part1(reports: &Vec<Report>) {
    let mut safe_reports: i32 = 0;

    for r in reports {
        let is_safe = report_is_safe(&r);
        safe_reports += if is_safe { 1 } else { 0 }
    }

    println!("Part 1: Safe Reports {}", safe_reports)
}

fn part2(reports: &Vec<Report>) {
    let mut unsafe_reports: Vec<Report> = Vec::new();
    let mut safe_reports: i32 = 0;

    for r in reports {
        let is_safe = report_is_safe(&r);
        if is_safe {
            safe_reports += 1;
        } else {
            unsafe_reports.push(r.to_vec());
        }
    }

    println!(
        "Part 2: starting with n = {} unsafe reports ",
        unsafe_reports.len()
    );

    for r in unsafe_reports {
        let mut is_safe = false;
        for i in 0..r.len() {
            let mut r2 = r.to_vec();
            r2.remove(i);
            is_safe = report_is_safe(&r2);
            if is_safe {
                break;
            }
        }

        safe_reports += if is_safe { 1 } else { 0 }
    }

    println!("Part 2: Safe Reports {}", safe_reports)
}

pub fn solution() {
    println!("Day 2");
    let reports = get_data().unwrap();

    part1(&reports);
    part2(&reports);
}
