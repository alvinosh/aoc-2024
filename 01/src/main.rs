use std::{collections::HashMap, fs::read_to_string, iter::zip};

fn main1() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_to_string("./sample.txt")?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|w| w.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|row| (row[0], row[1]))
        .unzip();

    left.sort();
    right.sort();

    let dist = zip(left, right).fold(0, |acc, e| {
        return acc + if e.0 > e.1 { e.0 - e.1 } else { e.1 - e.0 };
    });

    println!("{}", dist);

    Ok(())
}

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let data = read_to_string("./sample.txt")?;

    let (mut left, mut right): (Vec<i32>, Vec<i32>) = data
        .lines()
        .map(|s| {
            s.split_whitespace()
                .filter_map(|w| w.parse::<i32>().ok())
                .collect::<Vec<i32>>()
        })
        .map(|row| (row[0], row[1]))
        .unzip();

    let mut repetition = HashMap::<i32, i32>::new();

    for n in right.iter() {
        if let Some(val) = repetition.get_mut(n) {
            *val = *val + 1;
        } else {
            repetition.insert(*n, 1);
        }
    }

    let dist = left.iter().fold(0, |acc, n| {
        let multiplier = if let Some(n) = repetition.get(n) {
            *n
        } else {
            0
        };

        acc + *n * multiplier
    });

    println!("{}", dist);

    Ok(())
}
