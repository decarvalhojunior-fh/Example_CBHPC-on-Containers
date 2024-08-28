use mpi::traits::*;
use mpi::Count;
use mpi::datatype::{PartitionMut, Partition};


pub fn sort<T:Communicator>(v: Vec<i32>, comm: T) -> Vec<i32> {

    let v_size = v.len();

    let mut buf = bucket_sort(v, &comm);

    // sort buckets, locally.
    buf.sort();

    balance_distribution(buf, v_size, &comm)
}

fn bucket_sort<T:Communicator>(v: Vec<i32>, comm: &T) -> Vec<i32> {

    let size: i32 = comm.size();
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

    comm.all_to_all_into(&count0, &mut count1);

    let recv_displs: Vec<Count> = count1
    .iter()
    .scan(0, |acc, &x| {
        let tmp = *acc;
        *acc += x;
        Some(tmp)
    })
    .collect();

    let mut size_buf = 0;
    for c in &count1 {
        size_buf += c;
    }

    let mut buf: Vec<i32> = vec![0; size_buf as usize];
       
    let partition_send = Partition::new(&msg[..], &count0[..], &send_displs[..]);
    let mut partition_recv = PartitionMut::new(&mut buf[..], &count1[..], &recv_displs[..]);
    comm.all_to_all_varcount_into(&partition_send, &mut partition_recv);

    return buf;
}




fn balance_distribution<T:Communicator>(buf: Vec<i32>, v_size: usize, comm: &T) -> Vec<i32> {

    let size: i32 = comm.size();

    let size_buf = buf.len() as i32;

    let mut a0 = vec![0; size as usize];
    comm.all_gather_into(&size_buf, &mut a0[..]);

    let a1 = vec![v_size as i32; size as usize];

    let (b0, b1, b2) = compute_alignment_vectors(a0, a1);

    redistribute(buf, b0, b1, b2, comm)
}  


fn redistribute<T:Communicator>(buf: Vec<i32>, b0:Vec<i32>, b1:Vec<i32>, b2:Vec<i32>, comm: &T) -> Vec<i32> {

    let size: i32 = comm.size();
    let rank: i32 = comm.rank();

    let mut buf2 = Vec::<i32>::new();

    mpi::request::multiple_scope(size as usize*2, |scope, coll| {

        let mut pos = 0;
        for i in 0..b0.len() {
            if rank == b0[i] && b1[i] != rank {
                let buf_send = &buf[pos .. pos + b2[i] as usize]; 
                let dst = comm.process_at_rank(b1[i]);
                let sreq = dst.immediate_send(scope, buf_send);
                coll.add(sreq);
                pos += b2[i] as usize;
            } else if rank == b0[i] && b1[i] == rank {
                pos += b2[i] as usize;
            }
        }

        pos = 0;
        for i in 0..b0.len() {
            if rank == b0[i] && b1[i] != rank {
                pos += b2[i] as usize;
            } else if rank == b1[i] && b0[i] != rank {
                let src = comm.process_at_rank(b0[i]);
                let (mut buf_recv, status) = src.receive_vec::<i32>();
                buf2.append(&mut buf_recv);
            } else if rank == b0[i] && rank == b1[i] {
                for v in &buf[pos .. pos + b2[i] as usize] {
                    buf2.push(*v);
                }
                pos += b2[i] as usize;
            }
        }

        let mut result = vec![];
        coll.wait_all(&mut result);
        
    });

    return buf2
}


fn compute_alignment_vectors(mut a0: Vec<i32>, mut a1: Vec<i32>) -> (Vec<i32>, Vec<i32>, Vec<i32>) 
{
    let mut b0 = Vec::new();
    let mut b1 = Vec::new();
    let mut b2 = Vec::new();
    let mut x0 = 0;
    let mut x1 = 0;

    while !a0.is_empty() || !a1.is_empty() {

        if a0[0] < a1[0] {
          b0.push(x0);     
          b1.push(x1);      
          b2.push(a0[0]);   
          a1[0] -= a0[0];   
          a0.remove(0);  
          x0 += 1;         
        }
        else if a1[0] < a0[0] {
            b0.push(x0);
            b1.push(x1);
            b2.push(a1[0]);
            a0[0] -= a1[0];
            a1.remove(0);  
            x1 += 1;
          }
        else {
            b0.push(x0);
            b1.push(x1);
            b2.push(a1[0]);
            a0.remove(0);  
            a1.remove(0);  
            x0 += 1;
            x1 += 1;
        }
    }

    return (b0, b1, b2)

/*  for i in 0..b0.len() {
        println!("rank={rank}, b0[{i}]={}, b1[{i}]={}, b2[{i}]={}", b0[i], b1[i], b2[i]);
    }*/
}