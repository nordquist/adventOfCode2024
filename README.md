# adventOfCode2024

Day 1 - Part 1
```rust
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
        },
        Err(e) => {
            println!("An error occurred while importing locations: {}", e);
            std::process::exit(1);
        }
    }    
}
```