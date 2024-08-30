
use mpi::collective::SystemOperation;
use mpi::traits::{Communicator, CommunicatorCollectives};
use mpi::topology::Color;

pub fn perform<T:Communicator>(v0: &Vec<i32>, comm: T) -> i32 {

    let oddness_comm = comm.split_by_color(Color::with_value(comm.rank() % 2));
    assert!(oddness_comm.is_some());
    let oddness_comm = oddness_comm.unwrap();
 
    let neighbour_comm = comm.split_by_color(Color::with_value(comm.rank() / 2));
    assert!(neighbour_comm.is_some());
    let neighbour_comm = neighbour_comm.unwrap();
 
    let mut v1 = vec![0; v0.len()];
    oddness_comm.all_reduce_into(v0, &mut v1, SystemOperation::sum());

    let mut r0 = 0;

    for i in 0..v1.len()-1 {
        r0 += v1[i] * v1[i];
    }

    let mut r1 = 0;
    neighbour_comm.all_reduce_into(&r0, &mut r1, SystemOperation::sum());

    return r1
}