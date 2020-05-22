use std::io::{self, Read, stdin, Write};

fn partition(vs: &mut [i32]) -> (&mut [i32], &mut [i32]) {
    let pivot_pos: usize = rand::random::<usize>() % vs.len();
    // Swap pivot to beginning of array, and put smallest elements towards pivot
    vs.swap(0, pivot_pos);
    let pivot = vs[0];

    let mut last = 0;
    for n in 1..(vs.len()) {
        if vs[n] <= pivot {
            last += 1;
            vs.swap(n, last);
        }
    }
    vs.swap(0, last);

    let (p1, p2) = vs.split_at_mut(last);
    (p1, &mut p2[1..]) // exclude pivot element
}

fn quicksort(vs: &mut [i32]) {
    if vs.len() > 1 {
        let (p1, p2) = partition(vs);

        quicksort(p1);
        quicksort(p2);
    }
}

fn quicksort_flat(vs: &mut [i32]) {
    let mut parts = Vec::new();
    parts.push(vs);
    while let Some(p) = parts.pop() {
        if p.len() > 1 {
            let (p1, p2) = partition(p);
            parts.push(p1);
            parts.push(p2);
        }
    }
}

fn read_list() -> Result<Vec<i32>, Box<dyn std::error::Error>> {
    let mut buf = String::new();
    stdin().read_line(&mut buf)?;

    let mut vs = Vec::new();
    for num in buf.split_whitespace() {
        match num.parse::<i32>() {
            Ok(n) => vs.push(n),
            Err(e) => return Err(e.into()),
        }
    }
    Ok(vs)
}

fn main() {
    print!("Enter comma-separated list: ");
    io::stdout().flush().unwrap();

    if let Ok(mut values) = read_list() {
        quicksort_flat(&mut values[..]);
        println!("{:?}", values);
    } else {
        println!("invalid list");
    }
}
