use core::result::Result::Ok;
use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    day_two_part_two();
    day_one_part_one();
    day_one_part_two();
    day_two_part_one();
    day_two_part_two();
}

fn day_one_part_one() {
    match import_locations() {
        Ok((mut list1, mut list2)) => {
            list1.sort();
            list2.sort();
            let distances: Vec<_> = list1
                .into_iter()
                .zip(list2.into_iter())
                .map(|(x, y)| (x as i32 - y as i32).abs() as u32)
                .collect();
            let sum: u32 = distances.iter().sum();
            println!("The total distance is: {}", sum);
        }
        Err(e) => {
            println!("An error occurred while importing locations: {}", e);
            std::process::exit(1);
        }
    }
}

fn day_one_part_two() {
    match import_locations() {
        Ok((list1, list2)) => {
            let mut sum = 0;
            for &i in &list1 {
                let n = list2.iter().filter(|&&x| x == i).count();
                sum += i * n as i32;
            }
            println!("The similarity score is: {}", sum);
        }
        Err(e) => {
            println!("An error occurred while importing locations: {}", e);
            std::process::exit(1);
        }
    }
}

fn day_two_part_one() {
    let reports = import_reports();
    let mut safety = vec![true; reports.len()];

    for (index, report) in reports.iter().enumerate() {
        let direction = report[0] - report[1];
        let mut prev_value: Option<i32> = None;        
        for &value in report {
            if let Some(prev) = prev_value {
                let diff = (prev - value).abs();
                if diff > 3 || diff == 0 {
                    safety[index] = false;
                    break;
                }
                else if direction < 0 && (prev - value) > 0 {
                    safety[index] = false;
                    break;                    
                }
                else if direction > 0 && (prev - value) < 0 {
                    safety[index] = false;
                    break;                         
                }
            }
            prev_value = Some(value);
        }
    }
    println!("Number of safe reports: {}", safety.iter().filter(|&&s| s == true).count()); 
}

fn day_two_part_two() {
    let reports = import_reports();
    let mut safe_reports: u16 = 0 as u16;
    let mut safed_reports: u16 = 0 as u16;

    for report in &reports {
        match check_report_safety(report.to_vec()) {
            Ok(unsafe_tuples) => {
                if unsafe_tuples.len() < 1 {
                    safe_reports = safe_reports+1;
                }
                else if unsafe_tuples.len() >= 1 {
                    for index in 0..report.len() {
                        let mut new_report = report.clone();
                        new_report.remove(index);
                        match check_report_safety(new_report) {
                            Ok(new_unsafe_tuples) => {
                                if new_unsafe_tuples.len() < 1 {
                                    safed_reports = safed_reports+1;
                                    break;
                                }
                            },
                            Err(e) => {
                                println!("An error occurred while re-checking: {}", e);
                                std::process::exit(1);
                            }
                        }               
                    }       
                }
            }
            Err(e) => {
                println!("An error occurred: {}", e);
                std::process::exit(1);
            }
        }
    }
    println!("Number of safe reports: {} + safe reports after damper {}, total safe reports: {}", safe_reports, safed_reports, (safe_reports+safed_reports)); 
}

fn check_report_safety(report: Vec<i32>) -> io::Result<Vec<(i32, i32)>> {
    let mut unsafe_tuples: Vec<(i32, i32)> = Vec::new();
    let direction = report[0] - report[1];
    let mut prev_value: Option<i32> = None;
    for (index, &value) in report.iter().enumerate() {
        if let Some(prev) = prev_value {
            let diff = (prev - value).abs();
            if (diff > 3 || diff == 0) || (direction < 0 && (prev - value) > 0) || (direction > 0 && (prev - value) < 0) {
                unsafe_tuples.push(((index-1).try_into().unwrap(),prev));
            }
        }
        prev_value = Some(value);
    }
    Ok(unsafe_tuples)
}

fn import_locations() -> io::Result<(Vec<i32>, Vec<i32>)> {
    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let path = Path::new("src/input_day_one");
    let display = path.display();

    let file = match File::open(&path) {
        Err(err) => panic!("Location file missing {}: {}", display, err),
        Ok(file) => file,
    };

    let reader = BufReader::new(file);

    for line in reader.lines() {
        match line {
            Ok(line) => {
                let parts: Vec<&str> = line.split_whitespace().collect();
                list1.push(parts[0].parse::<i32>().expect("Error parsing to integer"));
                list2.push(parts[1].parse::<i32>().expect("Error parsing to integer"));
            }
            Err(e) => println!("Error reading line: {}", e),
        }
    }

    Ok((list1, list2))
}

fn import_reports() -> Vec<Vec<i32>> {
    let path = Path::new("src/input_day_two");
    let file = File::open(&path).expect("Could not open file");
    let reader = BufReader::new(file);
    
    let mut reports = Vec::new();

    for line in reader.lines() {
        if let Ok(line) = line {
            let report: Vec<i32> = line
                .split_whitespace()
                .map(|s| s.parse().expect("Parse error"))
                .collect();
            reports.push(report);
        }
    }
    
    reports
}