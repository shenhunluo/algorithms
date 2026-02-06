#[macro_export]
macro_rules! left {
    ($i:expr) => {
        (($i+1) << 1) - 1
    };
}

#[macro_export]
macro_rules! right {
    ($i:expr) => {
        ($i+1) << 1
    };
}

#[macro_export]
macro_rules! parent {
    ($i:expr) => {
        (($i+1) >> 1) -1
    };
}


pub fn max_heapify<T:Ord+Copy>(a:&mut [T],lengh:usize,i:usize,count:&mut usize,swap_c:&mut usize){
    *count = *count + 1;
    let left = left!(i);
    let right = right!(i);
    let largest = if left < lengh && a[left] > a[i] {
        left
    } else {
        i
    };
    let largest = if right < lengh && a[right] > a[largest] {
        right
    } else {
        largest
    };
    if largest != i {
        *swap_c += 1;
        a.swap(largest, i);
        max_heapify(a,lengh,largest,count,swap_c);
    }
}

pub fn build_max_heap<T:Ord+Copy>(a:&mut [T],count:&mut usize,swap_c:&mut usize){
    for i in (0..a.len()/2).rev() {
        max_heapify(a, a.len(),i,count,swap_c);
    }
}

pub fn heap_sort<T:Ord+Copy>(a:&mut [T])->(usize,usize,usize){
    let mut count = 0;
    let mut swap_c = 0;
    build_max_heap(a,&mut count,&mut swap_c);
    let build_max_heap_count = count;
    for i in (1..a.len()).rev() {
        swap_c += 1;
        a.swap(0, i);
        max_heapify(a, i, 0,&mut count,&mut swap_c);
    }
    (build_max_heap_count,count,swap_c)
}
