use advent24::get_lines;
const FILENAME: &'static str = "./data/d9-input.txt";
pub fn main() {
  println!("Part 1: {}", part1(FILENAME));
  println!("Part 2: {}", part2(FILENAME));
}

fn part1(filename: &str) -> i64 {
  let diskmap = &get_lines(filename)[0].chars().map(|c| c.to_string().parse::<u32>().unwrap()).collect::<Vec<u32>>();
  let mut disk: Vec<i64> = Vec::new();
  let mut id = 0;
  for i in 0..diskmap.len() {
    if i % 2 == 0 {
      // block count
      for _b in 0..diskmap[i] {
        disk.push(id);
      }
      if i < diskmap.len() - 1 {
        for _e in 0..diskmap[i + 1] {
          disk.push(-1);
        }
      }
      id = id + 1;
    }
  }
  let mut free_slot = next_free(&disk);
  let mut done = false;
  while free_slot.is_some() && !done {
    let i2: usize;
    {
      let move_slot = disk.iter().enumerate().filter(|(_i, n)| **n >= 0).last().unwrap();
      i2 = move_slot.0;
    }
    let i1 = free_slot.unwrap();
    if i2 > i1 {
      disk = swap(disk, i1, i2);
      free_slot = next_free(&disk);
    } else {
      done = true;
    }
  }
  disk.iter().filter(|i| **i >= 0).enumerate().fold(0, |acc, (i, n)| acc + (i as i64) * n)
}

#[derive(Debug, Clone, Copy)]
struct FileDesc {
  len: usize,
  id: usize,
  empty: bool,
}

fn part2(filename: &str) -> i64 {
  let diskmap = &get_lines(filename)[0].chars().map(|c| c.to_string().parse::<usize>().unwrap()).collect::<Vec<usize>>();
  let disk: Vec<FileDesc> = diskmap
    .iter()
    .enumerate()
    .map(|(i, n)| {
      if i % 2 == 0 {
        FileDesc { len: *n, id: i / 2, empty: false }
      } else {
        FileDesc { len: *n, id: 0, empty: true }
      }
    })
    .collect();
  let mut target_disk: Vec<FileDesc> = vec![FileDesc { len: 0, id: 0, empty: false }; disk.len()];
  target_disk.copy_from_slice(&disk);
  for (_i, file) in disk.iter().enumerate().rev() {
    if !file.empty {
      let target_idx: Option<usize> = next_free2(&target_disk, file.len);
      if target_idx.is_some() {
        let target_pos = target_idx.unwrap();
        let free_pos = target_pos + 1;
        let old_idx = target_disk.iter().enumerate().find(|(_i, f)| f.id == file.id).map(|(i, _f)| i).unwrap();
        if target_pos < old_idx {
          target_disk = set_freespace(target_disk, old_idx);
          target_disk.insert(target_pos, *file);

          let size_diff = target_disk[free_pos].len - file.len;
          if size_diff > 0 {
            target_disk[free_pos].len = size_diff;
            if free_pos < target_disk.len() - 1 && target_disk[free_pos + 1].empty {
              target_disk[free_pos].len = target_disk[free_pos].len + target_disk[free_pos + 1].len;
              target_disk.remove(free_pos + 1);
            }
          } else {
            target_disk.remove(free_pos);
          }
        }
      }
    }
  }
  let temp = target_disk.iter().map(|fd| vec![fd.id; fd.len]).collect::<Vec<Vec<usize>>>();
  temp.concat().iter().enumerate().fold(0, |acc, (i, n)| acc + i * n) as i64
}

fn next_free(disk: &Vec<i64>) -> Option<usize> {
  let r = disk.iter().enumerate().find(|(_i, n)| **n == -1);
  match r {
    Some((i, _n)) => Some(i),
    None => None,
  }
}
fn next_free2(disk: &Vec<FileDesc>, len: usize) -> Option<usize> {
  let r = disk.iter().enumerate().find(|(_i, fd)| fd.empty && fd.len >= len);
  match r {
    Some((i, _n)) => Some(i),
    None => None,
  }
}
fn swap(mut disk: Vec<i64>, i1: usize, i2: usize) -> Vec<i64> {
  let t = disk[i1];
  disk[i1] = disk[i2];
  disk[i2] = t;
  disk
}
fn set_freespace(mut disk: Vec<FileDesc>, idx: usize) -> Vec<FileDesc> {
  disk[idx].id = 0;
  disk[idx].empty = true;
  if idx < disk.len() - 1 && disk[idx + 1].empty {
    disk[idx].len = disk[idx].len + disk[idx + 1].len;
    disk.remove(idx + 1);
  }
  if idx > 0 && disk[idx - 1].empty {
    disk[idx].len = disk[idx].len + disk[idx - 1].len;
    disk.remove(idx - 1);
  }
  disk
}
