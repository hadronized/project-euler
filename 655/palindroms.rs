// The numbers 545, 5995 and 15151 are the three smallest palindromes divisible by 109. There are
// nine palindromes less than 100000 which are divisible by 109
//
// How many palindromes less than 10³² are divisible by 10000019?
//
// https://projecteuler.net/problem=655

fn rev_digits(mut a: u128) -> u128 {
  let mut b = 0;

  while a > 10 {
    let digit = a % 10;
    b = b * 10 + digit;
    a /= 10;
  }

  b * 10 + a % 10
}

fn is_palindrom(a: u128) -> bool {
  a == rev_digits(a)
}

fn main() {
  let n: u128 = 10000019;
  let max = 10u128.pow(32);

  for a in (n..).step_by(n as usize) {
    if a >= max {
      break;
    }

    if is_palindrom(a) {
      println!("{} passes", a);
    }
  }
}
