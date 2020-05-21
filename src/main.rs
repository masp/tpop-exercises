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
    vs.swap(0, last + 1);

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

fn main() {
    let mut values: [i32; 5] = [5, 3, 4, 1, 2];

    quicksort(&mut values);

    println!("{:?}", values);
}
