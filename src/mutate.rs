use rand;
use rand::distributions::{IndependentSample, Range};

/* Minimal Mutation */
fn flip_bit(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn change_byte(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn change_ascii_integer(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn change_binary_integer(buf:Vec<u8>) -> Vec<u8> {
  buf
}

/* Block Mutation */
fn shuffle_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn overwrite_copy_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn insert_copy_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn overwrite_const_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn insert_const_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn remove_block(buf:Vec<u8>) -> Vec<u8> {
  buf
}

fn cross_over(buf:Vec<u8>) -> Vec<u8> {
  buf
}

pub fn mutate(buf:&Vec<u8>) -> Vec<u8> {
  let new_buf = buf.clone();

  let rnd = Range::new(0, 10);
  let mut rng = rand::thread_rng();

  match rnd.ind_sample(&mut rng) {
    0 => flip_bit(new_buf),
    1 => change_byte(new_buf),
    2 => change_ascii_integer(new_buf),
    3 => change_binary_integer(new_buf),
    4 => shuffle_block(new_buf),
    5 => overwrite_copy_block(new_buf),
    6 => insert_copy_block(new_buf),
    7 => overwrite_const_block(new_buf),
    8 => insert_const_block(new_buf),
    9 => remove_block(new_buf),
    10 => cross_over(new_buf),
    _ => panic!("unreachable code")
  }
}
