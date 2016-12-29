use rand;
use rand::distributions::{IndependentSample, Range};
use rand::{Rng};

use seed::Seed;

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
                  .unwrap_or(0);
  beg = if beg > 0 && buf[beg - 1] == '-' as u8 { beg - 1 } else { beg };

  let end = (offset+1..buf.len()).find(|&x| !is_digit(buf[x]))
              .unwrap_or(buf.len());

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

fn change_ascii_integer(buf:&Vec<u8>) -> Vec<u8> {
  let offset = match find_close_number(&buf, get_random(buf.len())) {
                 Some(x) => x,
                 None => return buf.clone()
               };
  let (beg, end) = find_number_range(&buf, offset);

  let num = String::from_utf8(buf[beg .. end].to_vec()).unwrap().parse::<i64>().unwrap();
  let new_num = change_integer(num);

  [&buf[0..beg], new_num.to_string().as_bytes(), &buf[end..]]
    .iter().flat_map(|x| x.clone()).map(|&x| x).collect()
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

fn split_tokens(buf:&Vec<u8>) -> Vec<Vec<u8>> {
  let mut tokens = vec![];
  let mut token = vec![];
  for c in buf {
    if DELIMETERS.contains(&c) {
      tokens.push(token);
      token = vec![];
    } else {
      token.push(*c);
    }
  }
  tokens
}

fn shuffle_ascii_block(buf:&Vec<u8>) -> Vec<u8> {
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

fn insert_copy_block(buf:&Vec<u8>) -> Vec<u8> {
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

fn insert_const_block(buf:&Vec<u8>) -> Vec<u8> {
  let offset = get_random(buf.len());
  let size = get_random(buf.len()); // Too much?
  let block = vec![get_random(256) as u8; size];
  buf.iter().take(offset).chain(block.iter()).chain(buf.iter().skip(offset))
    .map(|&x| x).collect()
}

fn remove_block(buf:&Vec<u8>) -> Vec<u8> {
  let (to_beg, to_end) = select_block(&buf);
  buf.iter().take(to_beg).chain(buf.iter().skip(to_end))
    .map(|&x| x).collect()
}

fn random_split_point(buf1:&Vec<u8>, buf2:&Vec<u8>) -> usize {
  let mut diffs = buf1.iter().zip(buf2).enumerate().filter(|&(_, (x1, x2))| x1 != x2)
                .map(|(idx, _)| idx);
  let beg = diffs.nth(0).unwrap_or(0);
  let end = diffs.last().unwrap_or(0);
  get_random(end - beg + 1) + beg
}

fn cross_over(buf:&Vec<u8>, q:&Vec<Seed>) -> Vec<u8> {
  let buf2 = q[get_random(q.len())].load_buf(); // XXX: avoid the smae
  let offset = random_split_point(buf, &buf2);
  let (buf_front, _) = buf.split_at(offset);
  let (_, buf2_back) = buf2.split_at(offset);
  buf_front.iter().chain(buf2_back).map(|&x| x).collect()
}

pub fn mutate(buf:&Vec<u8>, q:&Vec<Seed>) -> Vec<u8> {
  match get_random(12) {
    0 => flip_bit(buf.clone()),
    1 => change_byte(buf.clone()),
    2 => change_ascii_integer(buf),
    3 => change_binary_integer(buf.clone()),
    4 => shuffle_block(buf.clone()),
    5 => shuffle_ascii_block(buf),
    6 => overwrite_copy_block(buf.clone()),
    7 => insert_copy_block(buf),
    8 => overwrite_const_block(buf.clone()),
    9 => insert_const_block(buf),
    10 => remove_block(buf),
    11 => cross_over(buf, q),
    _ => panic!("unreachable code")
  }
}
