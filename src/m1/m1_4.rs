pub fn quick_find_max_subarray(v:&Vec<i32>) -> (usize,usize,i32){
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