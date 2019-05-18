use rand::{thread_rng, Rng};
use std::time::SystemTime;

fn quick_find_max_subarray(v:&Vec<i32>) -> (usize,usize,i32){
    let mut max_array = (0,0,0);
    let mut i = 0;
    while(i<v.len()){
        let array = find_new_array(&v,i);
        i = array.1;
        let mut sum = array.2;
        if array.2 > max_array.2{
            max_array = array;
        }
        for j in i..v.len(){
            i=j+1;
            if(sum+v[j]<0){
                break;
            }else{
                sum = sum+v[j];
                if(sum>max_array.2){
                    max_array = (array.0,j+1,sum);
                }
            }
        }
    }
    return max_array;
}

fn find_new_array(v:&Vec<i32>,left:usize)->(usize,usize,i32){
    for i in left..v.len(){
        if(v[i]>0){
            let mut sum = v[i];
            for j in i+1..v.len(){
                if(v[j]<0){
                    return (i,j,sum)
                }else{
                    sum = sum + v[j];
                }
            }
            return (i,v.len(),sum);
        }
    }
    return (v.len(),v.len(),0);
}

pub fn m_main(){
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
        println!("q-m : {:?}",t2.duration_since(t1).ok().unwrap().as_nanos()/(10 as i32).pow(j) as u128);
//        println!("m-s : {:?}",t3.duration_since(t2).ok().unwrap().as_nanos()/(10 as i32).pow(j) as u128);
        //println!("{:?}",t3.duration_since(t2).ok().unwrap().as_nanos()/t2.duration_since(t1).ok().unwrap().as_nanos());
    }
}