use std::fs;
use regex::Regex;
struct submarine{
    forward: f64,
    down: f64,
    up: f64,

}
impl submarine {
    fn new() -> submarine {
        submarine {
            forward: 0.0,
            down: 0.0,
            up: 0.0,
        }
    }
    fn calculate_location(&self) -> f64 {

        (self.up - self.down) * self.forward
    }
}
struct advSub {
    aim: f64,
    horizontal: f64,
    depth: f64,
}
impl advSub {
    fn new() -> advSub {
        advSub {
            aim: 0.0,
            horizontal: 0.0,
            depth: 0.0,
        }
    }
    fn multiply_by_depth(&self) -> f64 {
        self.horizontal * self.depth
    }


    fn decode_error(){
        unimplemented!()
    }

}


fn main() {
    let contents = fs::read_to_string("input.txt").expect("Something went wrong reading the file");
    let mut sub = submarine::new();
    let mut advSub = advSub::new();
    let error = fs::read_to_string("error.txt").expect("Something went wrong reading the file");
    let mut error_vec = Vec::new();


    for line in contents.lines() {
        let split_vec = line.split(" ").collect::<Vec<&str>>();
        match split_vec[0]{
            "forward" => {
                sub.forward += split_vec[1].parse::<f64>().unwrap();
                advSub.horizontal += split_vec[1].parse::<f64>().unwrap();
                advSub.depth += split_vec[1].parse::<f64>().unwrap() * advSub.aim;
                },
            "down" => {
                sub.down += split_vec[1].parse::<f64>().unwrap();
                advSub.aim += (split_vec[1].parse::<f64>().unwrap());
            },
            "up" => {
                sub.up += split_vec[1].parse::<f64>().unwrap();
                advSub.aim -= split_vec[1].parse::<f64>().unwrap() ;
            },

            _ => panic!("Unknown Direction"),
        }


        }
    for line in error.lines(){
        error_vec.push(line.parse::<binary>().unwrap());

    }
    println!("{}", advSub.multiply_by_depth());
    println!("{}", advSub.aim *advSub.horizontal);
    println!("{:?}", error_vec);
}
