pub mod sort {
    use std::time::SystemTime;
    use rand::Rng;

    pub fn m_main() {
        for i in 1 as u32..16 {
            let mut a = vec![];
            let mut rng = rand::thread_rng();
            for _j in 0..(2 as i32).pow(i) {
                a.push(rng.gen::<i32>());
            }
            let r = a.len();
            let mut a1 = a.clone();
            let mut a2 = a.clone();
            let mut a3 = a.clone();
            let t1 = SystemTime::now();
            merge_sort(&mut a1, 0, r - 1);
            let t2 = SystemTime::now();
            insert_sort(&mut a2, 0, r - 1);
            let t3 = SystemTime::now();
            //merge_insert_sort(&mut a3,0,r-1);
            insert_m_sort(&mut a3);
            let t4 = SystemTime::now();
            a.sort();
            let t5 = SystemTime::now();
            for l in 0..a.len() {
                if a1[l] != a[l] {
                    println!("merge error!!");
                    panic!();
                }
            }
            for l in 0..a.len() {
                if a2[l] != a[l] {
                    println!("insert error!!");
                    panic!();
                }
            }
            for l in 0..a.len() {
                if a3[l] != a[l] {
                    println!("insert error!!");
                    println!("a:{:?}  a3:{:?}", a, a3);
                    panic!();
                }
            }
            println!("system for 2*{} = {}:{:?}", i, (2 as i32).pow(i), t5.duration_since(t4).ok().unwrap());
            println!("inseart-n for 2*{} = {}:{:?}", i, (2 as i32).pow(i), t4.duration_since(t3).ok().unwrap());
            println!("insert for 2*{} = {}:{:?}", i, (2 as i32).pow(i), t3.duration_since(t2).ok().unwrap());
            println!("merge for 2*{} = {}:{:?}", i, (2 as i32).pow(i), t2.duration_since(t1).ok().unwrap());
        }
    }

    fn insert_sort(a: &mut Vec<i32>, p: usize, r: usize) {
        for j in p..r - p + 1 {
            let key = a[j];
            let mut i = j;
            while i > p && a[i - 1] > key {
                a[i] = a[i - 1];
                i = i - 1;
            }
            a[i] = key;
        }
    }

    fn insert_m_sort(a: &mut Vec<i32>) {
        for j in 1..a.len() {
            let key = a[j];
            let k = find_insert(a, 0, j - 1, key);
            let mut i = j;
            while i > k {
                a[i] = a[i - 1];
                i = i - 1;
            }
            a[k] = key;
        }
    }

    fn find_insert(a: &Vec<i32>, s: usize, e: usize, v: i32) -> usize {
        let q = (s + e) / 2;
        if s < e {
            if a[q] >= v {
                return find_insert(a, s, q, v);
            } else {
                return find_insert(a, q + 1, e, v);
            }
        } else {
            if v < a[s] {
                return s;
            } else {
                return s + 1;
            }
        }
    }

    fn merge_sort(a: &mut Vec<i32>, p: usize, r: usize) {
        if p < r {
            let q = (p + r) / 2;
            merge_sort(a, p, q);
            merge_sort(a, q + 1, r);
            merge(a, p, q, r);
        }
    }

    pub fn merge_insert_sort(a: &mut Vec<i32>, p: usize, r: usize) {
        if p < r {
            if r - p < 20 {
                insert_sort(a, p, r);
            } else {
                let q = (p + r) / 2;
                merge_sort(a, p, q);
                merge_sort(a, q + 1, r);
                merge(a, p, q, r);
            }
        }
    }

    pub fn merge(a: &mut Vec<i32>, p: usize, q: usize, r: usize) {
        let n1 = q - p + 1;
        let n2 = r - q;
        let mut left = vec![];
        let mut right = vec![];
        for i in 0..n1 {
            left.push(a[p + i]);
        }
        for i in 0..n2 {
            right.push(a[q + i + 1])
        }
        left.push(2147483647);
        right.push(2147483647);
        let mut j = 0;
        let mut k = 0;
        for i in p..r + 1 {
            if left[j] <= right[k] {
                a[i] = left[j];
                j = j + 1;
            } else {
                a[i] = right[k];
                k = k + 1;
            }
        }
    }
}
