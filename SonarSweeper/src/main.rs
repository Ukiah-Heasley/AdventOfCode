use std::env;
use std::fs;

fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut vector = vec![];
    for line in contents.lines() {
        vector.push(line.parse::<i32>().unwrap());
    }
    println!("{:?}" , contents);
    println!("{:?}" , vector.len());

    println!("Number of increases {} , Number of Decreases {} , Number of Ties {}" , numberOfIncreases(&vector) , numberOfDecreases(&vector) , numberOfTies(&vector));
    println!("{}" , numberOfSlidingWindow(&vector));
}

fn numberOfIncreases(vector: &Vec<i32>) -> i32 {
    let mut count = 0;
    for i in 0..vector.len() - 1 {
        if vector[i] < vector[i + 1] {
            count += 1;
        }
    }
    count
}
fn numberOfDecreases(vector: &Vec<i32>) -> i32 {
    let mut count:i32 = 0;
    for i in 0..vector.len() - 1 {
        if vector[i] > vector[i + 1] {
            count += 1;
        }
    }
    count
}

fn numberOfTies(vector: &Vec<i32>) -> i32 {
    let mut count:i32 = 0;
    for i in 0..vector.len() - 1 {
        if vector[i] == vector[i + 1] {
            count += 1;
        }
    }
    count
}

fn numberOfSlidingWindow(vector: &Vec<i32>) -> i32 {
    let mut count:i32 = 0;
    for i in 0..vector.len() - 3 {
        if vector[i] + vector[i + 1] + vector[i + 2] < vector[i+1] + vector[i + 2] + vector[i + 3] {
            count += 1;
        }
    }
    count

}