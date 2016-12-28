use rand;
use rand::distributions::{IndependentSample, Range};

enum Endian {
  Big,
  Little
}

fn get_random(sz:usize) -> usize {
  let rnd = Range::new(0, sz);
  let mut rng = rand::thread_rng();
  rnd.ind_sample(&mut rng)
}

/* Minimal Mutation */
fn flip_bit(mut buf:Vec<u8>) -> Vec<u8> {
  let offset = get_random(buf.len() * 8);
  buf[offset/8] = buf[offset/8] ^ (1 << (offset % 8));
  buf
}

fn change_byte(mut buf:Vec<u8>) -> Vec<u8> {
  let offset = get_random(buf.len());
  buf[offset] = get_random(256) as u8;
  buf
}

fn is_digit(c:u8) -> bool {
  if c >= '0' as u8 && c <= '9' as u8 { true } else { false }
}

fn find_close_number(buf:&Vec<u8>, offset:usize) -> Option<usize> {
  let mut i = offset;
  let mut ret = None;

  if is_digit(buf[offset]) {
    return Some(offset)
  }

  while ret == None && (offset + i < buf.len() || offset - i >= 0) {
    if offset + i < buf.len() && is_digit(buf[offset + i]) {
      ret = Some(offset + i);
    } else if offset - i >= 0 && is_digit(buf[offset - i]) {
      ret = Some(offset - i);
    }
  }
  ret
}

fn find_number_range(buf:&Vec<u8>, offset:usize) -> (usize, usize) {
  let mut beg = (1..offset+1).rev().find(|&x| !is_digit(buf[x - 1]))
                  .unwrap_or(offset);
  beg = if beg > 0 && buf[beg - 1] == '-' as u8 { beg - 1 } else { beg };

  let end = (offset..buf.len()-1).find(|&x| !is_digit(buf[x + 1]))
              .unwrap_or(offset);

  (beg, end)
}

fn change_integer(num:i64) -> i64 {
  match get_random(3) {
    0 => num + get_random(30) as i64 + 1,
    1 => num - get_random(30) as i64 - 1,
    2 => 0xffffffff, // XXX: dictionary?
    _ => panic!("unreachable code")
  }
}

fn change_ascii_integer(mut buf:Vec<u8>) -> Vec<u8> {
  let offset = match find_close_number(&buf, get_random(buf.len())) {
                 Some(x) => x,
                 None => return buf
               };
  let (beg, end) = find_number_range(&buf, offset);
  let num = String::from_utf8(buf[beg .. end+1].to_vec()).unwrap().parse::<i64>().unwrap();

  let new_num = change_integer(num);

  let mut ret = buf[0..beg].to_vec(); // Better idea to write this code??
  ret.extend(new_num.to_string().into_bytes());
  ret.extend(buf[end+1..].to_vec());
  ret
}

fn change_binary_integer(mut buf:Vec<u8>) -> Vec<u8> {
  let size = [1, 2, 4, 8][get_random(4)];
  let endian = if get_random(2) == 0 { Endian::Little } else { Endian::Big };
  let offset = get_random(buf.len() - size + 1);

  let num: i64 = match endian {
                   Endian::Little => buf[offset..offset+size].iter()
                                       .fold(0, |acc, &n| (acc << 8) + n as i64),
                   Endian::Big => buf[offset..offset+size].iter().rev()
                                    .fold(0, |acc, &n| (acc << 8) + n as i64)
                 };

  let new_num = change_integer(num);

  for i in offset..offset+size {
    let j = i - offset;
    buf[i] = match endian { // Dirty code...
               Endian::Little => (new_num >> (8*(size - j - 1))) & 0xff,
               Endian::Big => (new_num >> (8*j)) & 0xff
             } as u8;
  }

  buf
}

/* Block Mutation */
fn shuffle_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn shuffle_ascii_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

// TODO: uniform distribution??
fn overwrite_copy_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn insert_copy_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn overwrite_const_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn insert_const_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn remove_block(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

fn cross_over(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

pub fn mutate(buf:&Vec<u8>) -> Vec<u8> {
  let new_buf = buf.clone();

  match get_random(11) {
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
