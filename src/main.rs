


mod quicksort;
use rand::prelude::*;   
use mpi::traits::*;
use mpi::topology::Color;
use mpi::Count;
use mpi::datatype::{PartitionMut, Partition};

fn main() {

    let universe = mpi::initialize().unwrap();
    let world = universe.world();
    let size = world.size();
    let rank = world.rank();

    let oddness_comm = world.split_by_color(Color::with_value(world.rank() % 2));
    assert!(oddness_comm.is_some());
    let oddness_comm = oddness_comm.unwrap();
    
    let v0: Vec<i32> = create_array::<128>(1,32) ;

    let v1 = if rank % 2 == 0 { 
                          sort1(v0, oddness_comm) 
                       } 
                       else 
                       { 
                          sort2(v0, oddness_comm) 
                       };

    let r3 = innerproduct(&v1, world);

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

fn sort1<T:Communicator>(v: Vec<i32>, comm: T) -> Vec<i32> {

    let size: i32 = comm.size();
    let rank: i32 = comm.rank();
    let v_size = v.len();

    let bucket_size = 32/size as usize;

    let mut count0: Vec<Count> = vec![0; size as usize];
    for key in &v {
        let ix = *key as usize / bucket_size; 
        count0[ix] += 1;
    }

    let send_displs: Vec<Count> = count0
        .iter()
        .scan(0, |acc, &x| {
            let tmp = *acc;
            *acc += x;
            Some(tmp)
        })
        .collect();

    
    let mut msg = vec![0; v_size];

    let mut count1: Vec<Count> = vec![0; size as usize];
    for key in &v {
        let ix = *key as usize / bucket_size;
        msg[send_displs[ix] as usize + count1[ix] as usize] = *key;
        count1[ix] += 1;
    }

    // assert!(count0 == count1);

    comm.all_to_all_into(&count0, &mut count1);

    let recv_displs: Vec<Count> = count1
    .iter()
    .scan(0, |acc, &x| {
        let tmp = *acc;
        *acc += x;
        Some(tmp)
    })
    .collect();

    println!("size={size}");

    for i in 0..2 {
        let c10 = count0[i as usize];
        let c20 = send_displs[i as usize];
        let c11 = count1[i as usize];
        let c21 = recv_displs[i as usize];
        println!("rank={rank}, i = {i}, count0={c10}, send_displs={c20}");
        println!("rank={rank}, i = {i}, count1={c11}, recv_displs={c21}");
    }

    let mut size_buf = 0;
    for c in &count1 {
        size_buf += c;
    }

    println!("size_buf = {size_buf}");

    let mut buf: Vec<i32> = vec![0; size_buf as usize];
       
    let partition_send = Partition::new(&msg[..], &count0[..], &send_displs[..]);
    let mut partition_recv = PartitionMut::new(&mut buf[..], &count1[..], &recv_displs[..]);
    comm.all_to_all_varcount_into(&partition_send, &mut partition_recv);
    quicksort::q(&mut buf);
//    for item in &buf {
//        println!("rank={rank} -- item={item}");
//    }
    
    return buf;

}  

fn sort2<T:Communicator>(v: Vec<i32>, comm: T) -> Vec<i32> {
    return v;
} 

fn innerproduct<T:Communicator>(v: &Vec<i32>, comm: T) -> i32 {
    let mut r = 0;

    for i in 0..v.len()-1 {
        r += v[i]*v[i];
    }

    return r
}

