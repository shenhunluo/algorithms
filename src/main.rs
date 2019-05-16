mod m1;
extern crate rand;
use m1::m1_4::quick_find_max_subarray;
use m1::m1_2::sort::merge_sort;
use rand::{thread_rng, Rng};
use std::time::SystemTime;

fn main(){
    for j in 1..10 {
        let mut rng = thread_rng();
        let mut v = vec![];
        for i in 0..(10 as i32).pow(j) {
            v.push(rng.gen::<i32>() % 10);
        }
        //println!("{:?}", v);
        //println!("{:?}", quick_find_max_subarray(&v));
        let r = v.len()-1;
        let t1 = SystemTime::now();
        quick_find_max_subarray(&v);
//        let mut sum = 0;
//        for k in 0..v.len(){
//            sum = sum + v[k];
//        }
        let t2 = SystemTime::now();
        merge_sort(&mut v, 0, r);
        let t3 = SystemTime::now();
        println!("q-m : {:?}",t2.duration_since(t1).ok().unwrap().as_nanos()/(10 as i32).pow(j) as u128);
//        println!("m-s : {:?}",t3.duration_since(t2).ok().unwrap().as_nanos()/(10 as i32).pow(j) as u128);
        //println!("{:?}",t3.duration_since(t2).ok().unwrap().as_nanos()/t2.duration_since(t1).ok().unwrap().as_nanos());
    }
}