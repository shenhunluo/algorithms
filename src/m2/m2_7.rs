use rand::Rng;

pub fn quick_sort<T:Ord>(a:&mut [T]){
    quick_sort_with_begin_end(a, 0, a.len());
}

fn quick_sort_with_begin_end<T:Ord>(a:&mut [T],begin:usize,end:usize) {
    if end - begin > 1 {
        let p = partition(a,begin,end);
        quick_sort_with_begin_end(a, begin, p);
        quick_sort_with_begin_end(a, p+1, end);
    }
}

fn partition<T:Ord>(a:&mut [T],begin:usize,end:usize)->usize{
    let mut rng = rand::thread_rng();
    let mut x = rng.gen_range(begin..end);
    let mut i = begin;
    for j in begin..end {
        if a[j] < a[x] {
            if j == x {
                x = i;
            }
            if i == x {
                x = j;
            }
            a.swap(i, j);
            i += 1;
        }
    }
    if i != x {
        a.swap(i,x);
    }
    i
}

#[cfg(test)]
mod tests {
    use std::time::SystemTime;

    use rand::Rng;

    use super::quick_sort;
    #[test]
    fn test() {
        for _ in 0..100 {
            let mut a = vec![];
            let mut rng = rand::thread_rng();
            for _ in 0..100000 {
                a.push(rng.gen_range(0..200000));
            }
            let t1 = SystemTime::now();
            quick_sort(&mut a);
            let t2 = SystemTime::now();
            println!("{:?}",t2.duration_since(t1).unwrap());
            for i in 1..a.len() {
                if a[i] < a[i-1] {
                    panic!()
                }
            }
        }
    }
}
