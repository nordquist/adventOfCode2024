use std::fs::File;
use std::io::{self, BufRead, BufReader};
use std::path::Path;

fn main() {
    day_one_part_one();
    day_one_part_two();
}

fn day_one_part_one() {
    match import_locations() {
        Ok((mut list1,mut list2)) => {
            list1.sort();
            list2.sort();
            let distances: Vec<_> = list1
                .into_iter()
                .zip(list2.into_iter())
                .map(|(x, y)| (x as i32 - y as i32).abs() as u32)
                .collect();
            let sum: u32 = distances.iter().sum();
            println!("The total distance is: {}", sum);   
        },
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
        },
        Err(e) => {
            println!("An error occurred while importing locations: {}", e);
            std::process::exit(1);
        }
    }    
}

fn import_locations() -> io::Result<(Vec<i32>, Vec<i32>)> {

    let mut list1: Vec<i32> = Vec::new();
    let mut list2: Vec<i32> = Vec::new();
    let path = Path::new("src/input");
    let display = path.display();

    let file = match File::open(&path) {
        Err(err) => panic!("Location file missing {}: {}", display, err),
        Ok(file) => file
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