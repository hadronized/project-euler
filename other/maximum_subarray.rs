fn main() {
  let input = [2, 3, -1, -20, 5, 10];

  let r = solve(&input);
  println!("{:?}", r);
}

fn solve(input: &[i32]) -> Option<(i32, usize, usize)> {
  if input.len() == 0 {
    return None;
  }

  let mut p = (0, 0);
  let mut sum = 0;

  for (i, &a) in input.iter().enumerate() {
    if sum < 0 {
      p = (i, i);
      sum = a;
    } else {
      sum += a;
      p.1 = i;
    }
  }

  Some((sum, p.0, p.1))
}
