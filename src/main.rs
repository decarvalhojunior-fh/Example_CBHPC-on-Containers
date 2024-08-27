
mod quicksort;
use rand::prelude::*;   

fn main() {

    let mut v1: Vec<i32> = create_array::<128>(1,32) ;
    let mut v2: Vec<i32> = create_array::<128>(1,32) ;
  
    sort1(&mut v1);
    sort2(&mut v2);

   let r3 = innerproduct(&v1, &v2);

    for item in v1 {
        println!("v1: {item}")
    }

    for item in v2 {
        println!("v2: {item}")
    }

    println!("inner product = {r3}")
}

fn create_array<const SIZE: usize>(m: i32, n: i32) -> Vec<i32> {
    let mut arr = vec![0; SIZE];
    for x in &mut arr {
        *x = thread_rng().gen_range(m..n);
    }
    arr
}

fn sort1(v: &mut Vec<i32>) {
    quicksort::q(v);
}  

fn sort2(v: &mut Vec<i32>) {
    quicksort::q(v);
} 

fn innerproduct(v1: &Vec<i32>, v2: &Vec<i32>) -> i32 {
    let mut r = 0;

    for i in 0..v1.len()-1 {
        r += v1[i]*v2[i];
    }

    return r
}

