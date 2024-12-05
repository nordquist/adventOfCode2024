use core::include_bytes;
use core::result::Result::Ok;
use std::fs::File;
use std::io::prelude::*;
use std::io::{self, BufRead, BufReader};
use std::path::Path;
use regex::Regex;

fn main() {
    day_one_part_one();
    day_one_part_two();
    day_two_part_one();
    day_two_part_two();
    day_three_part_one();
    day_three_part_two();
    day_four_part_one();
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
            println!("0x78A59F25: {}", sum);
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
            println!("0x78A59F26: {}", sum);
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
    println!("0x78A59F89: {}", safety.iter().filter(|&&s| s == true).count()); 
}

fn day_two_part_two() {
    let reports = import_reports();
    let mut safe_reports: u16 = 0 as u16;
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
                                    safe_reports = safe_reports+1;
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
    println!("0x78A59F8A {}", safe_reports); 
}

fn day_three_part_one() {
    match import_multiplications() {
        Ok(contents) => {
            let re = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();
            let pairs: Vec<_> = re.captures_iter(&contents)
                .filter_map(|cap| {
                    let x = cap[1].parse::<i32>();
                    let y = cap[2].parse::<i32>();
                    match (x, y) {
                        (Ok(x), Ok(y)) => Some((x, y)),
                        _ => None,
                    }
                })
                .collect();
           
            let mut sum_of_products  = 0;
            for pair in &pairs {
                sum_of_products = sum_of_products + (pair.0 * pair.1);
            }
            println!("0x78A59FED: {:?}", sum_of_products);
        },
        Err(e) => println!("Error reading the file: {}", e),
    }
}

fn day_three_part_two() {
    match import_multiplications() {
        Ok(contents) => {
            let delimiters = Regex::new(r"(do|don't)(\W|\z)").unwrap();
            let mut start: usize = 0;
            let mut lines = Vec::new();
            for mat in delimiters.find_iter(&contents) {
                let end = mat.start();
                lines.push(&contents[start..end]);
                start = end;
            }
            lines.push(&contents[start..]);

            let mut sum_of_products = 0;
            for line in &lines {
                if line.starts_with("don't()") {
                    continue;
                }
                let regex = Regex::new(r"mul\(\s*(-?\d+)\s*,\s*(-?\d+)\s*\)").unwrap();
                let pairs: Vec<_> = regex.captures_iter(line)
                .filter_map(|cap| {
                    let x = cap[1].parse::<i32>();
                    let y = cap[2].parse::<i32>();
                    match (x, y) {
                        (Ok(x), Ok(y)) => Some((x, y)),
                        _ => None,
                    }
                })
                .collect();
                
                for pair in &pairs {
                    sum_of_products = sum_of_products + (pair.0 * pair.1);
                }                    
            }

            println!("0x78A59FEE: {:?}", sum_of_products);
        },
        Err(e) => println!("Error reading the file: {}", e),
    }    
}

fn day_four_part_one() {
    let bytes = import_bytes();
    let matrix = bytes
        .split(|&c| c == b'\n')
        .collect::<Vec<_>>();

    let mut criteria = [0; 4];

    let occurrences = (0..matrix[0].len() as isize)
        .flat_map(|x| (0..matrix.len() as isize)
            .map(move |y| (x, y)))
        .flat_map(|(x, y)| {
            [
                [(x, y), (x + 1, y - 1), (x + 2, y - 2), (x + 3, y - 3)], 
                [(x, y), (x + 1, y), (x + 2, y), (x + 3, y)],
                [(x, y), (x + 1, y + 1), (x + 2, y + 2), (x + 3, y + 3)],
                [(x, y), (x, y + 1), (x, y + 2), (x, y + 3)],
            ]
        })
        .filter(|coords| {
            let mut iter = coords.iter().map(|(x, y)| {
                matrix.get(*y as usize)
                    .and_then(|row| row.get(*x as usize).copied())
                    .unwrap_or_default()
            });
            criteria.fill_with(|| iter.next().unwrap());
            &criteria == b"XMAS" || &criteria == b"SAMX"
        });
    
    let count = occurrences.count();
    println!("0x78A5A051: {:?}", count);
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

fn import_multiplications() -> io::Result<String> {
    let path = Path::new("src/input_day_three");
    let mut file = File::open(&path).expect("Could not open file");
    let mut contents = String::new();
    file.read_to_string(&mut contents)?;
    Ok(contents)
}

fn import_bytes() -> &'static [u8] {
    return include_bytes!("input_day_four");
}