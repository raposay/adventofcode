use std::env;
use std::fs;

fn main() {
    let args: Vec<String> = env::args().collect();

    let file_path = &args[1];

    println!("opening file {}", file_path);

    let data = read_lines(file_path);

    let max = sum_data(data);

    println!("top three is {:?}", max);
    println!("sum = {}", max.iter().sum::<i32>());
}

fn read_lines(filename: &str) -> Vec<Vec<i32>> {
    let mut res = Vec::new();
    let mut inv: Vec<i32> = Vec::new();
    for line in fs::read_to_string(filename).unwrap().lines() {
        match line {
            "" => {
                res.push(inv.clone());
                inv.clear();
            }
            _ => inv.push(line.parse().unwrap()),
        }
    }
    res
}

fn sum_data(invs: Vec<Vec<i32>>) -> Vec<i32> {
    println!("{:?}", invs);
    let mut sumvec: Vec<i32> = invs.iter().map(|inv| inv.iter().sum()).collect();
    sumvec.sort();
    return sumvec.drain(sumvec.len() - 3..).collect::<Vec<i32>>();
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn page_ex() {
        let mut v = Vec::new();
        v.push(Vec::from([1000, 2000, 3000]));
        v.push(Vec::from([2000]));
        let maxcal = sum_data(v);
        assert_eq!(maxcal, 6000);
    }
}
