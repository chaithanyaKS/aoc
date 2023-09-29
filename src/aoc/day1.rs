use std::fs;
pub fn day1p1() {
    let contents = fs::read_to_string("input.txt").expect("File not found");

    let values: Vec<usize> = contents
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|s| s.parse::<usize>().expect("Not a number"))
                .sum::<usize>()
        })
        .collect();
    println!("{:?}", values.iter().max())
}

pub fn day1p2() {
    let contents = fs::read_to_string("input.txt").expect("File not found");

    let mut values: Vec<usize> = contents
        .split("\n\n")
        .map(|s| {
            s.split('\n')
                .map(|s| s.parse::<usize>().expect("Not a number"))
                .sum::<usize>()
        })
        .collect();
    values.sort_by(|a, b| b.cmp(a));
    println!("{:?}", values.into_iter().take(3).sum::<usize>())
}
