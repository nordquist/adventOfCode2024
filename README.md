# adventOfCode2024

Day 1 - Part 1
```rust
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
```

Day 1 - Part 2
```rust
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
```

Day 2 - Part 1
```rust
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
```

Day 2 - Part 2
```rust
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
    println!("Number of safe reports after damper {}", safe_reports); 
}
```