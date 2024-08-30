use mpi::traits::Communicator;
use mpi::traits::*;

pub fn perform<T:Communicator>(mut v0: Vec<i32>, comm: T) -> Vec<i32> {

    let size = comm.size();
    let rank = comm.rank();

    for i in 1..v0.len() {        
        v0[i] += v0[i-1];
    }

    let sum = v0[v0.len()-1];

    mpi::request::scope(|scope| {

        for r in 0..size {
            if rank < r {
                let dst = comm.process_at_rank(r);
                let sreq = dst.immediate_send(scope, &sum); 
                sreq.wait();
            }
         }

         for r in 0..size {
            if rank > r {
                let src = comm.process_at_rank(r);                
                let (sum_previous, status) = src.receive::<i32>(); 
                for i in 0..v0.len() {        
                    v0[i] += sum_previous;
                }
            }
         }
    });
    
    return v0
}