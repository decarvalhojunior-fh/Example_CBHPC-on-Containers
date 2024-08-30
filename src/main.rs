mod prefixsum;
mod sort;
mod innerproduct;

use rand::prelude::*;   
use mpi::traits::*;
use mpi::topology::Color;

fn main() {

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let oddness_comm = world.split_by_color(Color::with_value(world.rank() % 2));
    assert!(oddness_comm.is_some());
    let oddness_comm = oddness_comm.unwrap();
    
    

    let v1 = if rank % 2 == 0 { 
                          let v0: Vec<i32> = create_array::<128>(0,31) ;
                          sort::perform(v0, oddness_comm) 
                       } 
                       else 
                       { 
                          let v0: Vec<i32> = create_array::<128>(0,7) ;
                          prefixsum::perform(v0, oddness_comm) 
                       };

    let r3 = innerproduct::perform(&v1, world);

    for item in v1 {
        println!("rank {rank} -- v1: {item}")
    }

    println!("rank {rank} -- inner product = {r3}")
}



fn create_array<const SIZE: usize>(m: i32, n: i32) -> Vec<i32> {
    let mut arr = vec![0; SIZE];
    for x in &mut arr {
       *x = thread_rng().gen_range(m..n);
    }
    arr
}


 



