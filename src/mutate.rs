use rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng};

enum Endian {
  Big,
  Little
}

const DELIMETERS:[u8; 3] = ['\n' as u8, ' ' as u8, '\t' as u8];

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
  let mut i = 0;
  let mut ret = None;

  if is_digit(buf[offset]) {
    return Some(offset)
  }

  while ret == None && (offset + i < buf.len() || offset >= i) {
    if offset + i < buf.len() && is_digit(buf[offset + i]) {
      ret = Some(offset + i);
    } else if offset >= i && is_digit(buf[offset - i]) {
      ret = Some(offset - i);
    }
    i += 1;
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

fn change_ascii_integer(buf:Vec<u8>) -> Vec<u8> {
  let offset = match find_close_number(&buf, get_random(buf.len())) {
                 Some(x) => x,
                 None => return buf
               };
  let (beg, end) = find_number_range(&buf, offset);

  let num = String::from_utf8(buf[beg .. end+1].to_vec()).unwrap().parse::<i64>().unwrap();
  let new_num = change_integer(num);

  buf[0..beg].iter()
    .chain(new_num.to_string().into_bytes().iter())
    .chain(buf[end+1..].iter())
    .map(|&x| x)
    .collect()
}

fn change_binary_integer(mut buf:Vec<u8>) -> Vec<u8> {
  let size = [1, 2, 4, 8][get_random(4)];
  let endian = if get_random(2) == 0 { Endian::Little } else { Endian::Big };
  let offset = get_random(buf.len() - size + 1);

  let num = match (&endian, buf[offset..offset+size].iter()) {
              (&Endian::Little, iter) => iter.fold(0, |acc, &n| (acc << 8) + n as i64),
              (&Endian::Big, iter) => iter.rev().fold(0, |acc, &n| (acc << 8) + n as i64)
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

fn select_block<T>(buf:&Vec<T>) -> (usize, usize) {
  // XXX: How can I select a uniformly random block
  let size = get_random(buf.len()) + 1;
  if get_random(1) == 0 {
    let beg = get_random(buf.len());
    (beg, if beg + size > buf.len() { buf.len() } else { beg + size })
  } else {
    let end = get_random(buf.len()) + 1;
    (if end < size { 0 } else { end - size }, end)
  }
}

/* Block Mutation */
fn shuffle_block(mut buf:Vec<u8>) -> Vec<u8> {
  let (beg, end) = select_block(&buf);
  let mut rng = rand::thread_rng();
  rng.shuffle(&mut buf[beg..end]);
  buf
}

fn split_tokens(buf:Vec<u8>) -> Vec<Vec<u8>> {
  let mut tokens = vec![];
  let mut token = vec![];
  for c in buf {
    if DELIMETERS.contains(&c) {
      tokens.push(token);
      token = vec![];
    } else {
      token.push(c);
    }
  }
  tokens
}

fn shuffle_ascii_block(buf:Vec<u8>) -> Vec<u8> {
  let mut tokens = split_tokens(buf);
  let (beg, end) = select_block(&tokens);

  let mut rng = rand::thread_rng();
  rng.shuffle(&mut tokens[beg..end]);

  tokens.iter().flat_map(|x| x.clone()).collect()
}

fn overwrite_copy_block(mut buf:Vec<u8>) -> Vec<u8> {
  let (to_beg, to_end) = select_block(&buf);
  let size = to_end - to_beg;
  let from_beg = get_random(buf.len() - size);
  let block = buf[to_beg..to_end].to_vec();
  for i in 0..size {
    buf[from_beg + i] = block[i];
  }
  buf
}

fn insert_copy_block(buf:Vec<u8>) -> Vec<u8> {
  let (to_beg, to_end) = select_block(&buf);
  let offset = get_random(buf.len() + 1);
  let block = buf[to_beg..to_end].to_vec();
  buf.iter().take(offset).chain(block.iter()).chain(buf.iter().skip(offset))
    .map(|&x| x).collect()
}

fn overwrite_const_block(mut buf:Vec<u8>) -> Vec<u8> {
  let (to_beg, to_end) = select_block(&buf);
  let byte = get_random(256) as u8;
  for i in to_beg..to_end {
    buf[i] = byte;
  }
  buf
}

fn insert_const_block(buf:Vec<u8>) -> Vec<u8> {
  let offset = get_random(buf.len());
  let size = get_random(buf.len()); // Too much?
  let block = vec![get_random(256) as u8; size];
  buf.iter().take(offset).chain(block.iter()).chain(buf.iter().skip(offset))
    .map(|&x| x).collect()
}

fn remove_block(buf:Vec<u8>) -> Vec<u8> {
  let (to_beg, to_end) = select_block(&buf);
  buf.iter().take(to_beg).chain(buf.iter().skip(to_end))
    .map(|&x| x).collect()
}

fn cross_over(mut buf:Vec<u8>) -> Vec<u8> {
  // XXX
  buf
}

pub fn mutate(buf:&Vec<u8>) -> Vec<u8> {
  let new_buf = buf.clone();

  match get_random(12) {
    0 => flip_bit(new_buf),
    1 => change_byte(new_buf),
    2 => change_ascii_integer(new_buf),
    3 => change_binary_integer(new_buf),
    4 => shuffle_block(new_buf),
    5 => shuffle_ascii_block(new_buf),
    6 => overwrite_copy_block(new_buf),
    7 => insert_copy_block(new_buf),
    8 => overwrite_const_block(new_buf),
    9 => insert_const_block(new_buf),
    10 => remove_block(new_buf),
    11 => cross_over(new_buf),
    _ => panic!("unreachable code")
  }
}
