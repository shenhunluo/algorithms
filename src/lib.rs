pub mod m1;
pub mod m2;


#[cfg(test)]
mod tests {
    use std::time::SystemTime;
    use crate::m2::m2_7::quick_sort;

    use super::m2::m2_6::heap_sort;
    use rand::seq::SliceRandom;

    #[test]
    fn sort_test() {
        for l in 1..10 {
            let len = 10i32.pow(l);
            let mut nums:Vec<i32> = (0..len).collect();
            let mut rng = rand::thread_rng();
            nums.shuffle(&mut rng);
            let mut nums_c = nums.clone();
            let t1 = SystemTime::now();
            let (b_c,c,s_c) = heap_sort(&mut nums_c);
            let t2 = SystemTime::now();
            let d_heap_sort = t2.duration_since(t1).unwrap();
            println!("len: {len}");
            println!("build heap max_heapify count: {b_c}");
            println!("all max_heapify count: {c}");
            println!("count / len / lg(n): {}",c as f64 / len as f64 / l as f64);
            println!("swap count: {s_c}");
            println!("swap count / len / lg(n): {}",s_c as f64 / len as f64 / l as f64);
            println!("heap_sort: {d_heap_sort:?}");
            nums_c = nums.clone();
            let t1 = SystemTime::now();
            quick_sort(&mut nums_c); 
            let t2 = SystemTime::now();
            println!("quick_sort: {:?}",t2.duration_since(t1).unwrap());
            nums_c = nums.clone();
            let t1 = SystemTime::now();
            nums_c.sort();
            let t2 = SystemTime::now();
            let d_sort = t2.duration_since(t1).unwrap();
            println!("sort: {d_sort:?}");
        }
    }
}

