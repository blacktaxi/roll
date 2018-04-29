extern crate rand;
extern crate regex;

use std::env;
use std::i64;
use std::iter;
use regex::Regex;
use rand::Rng;

fn main() {
    let re = Regex::new(r"^(?P<rolls>\d+)d(?P<die>\d+)((?P<mod>(\+|-)\d+))?$").unwrap();
    fn get_val<'r, 't>(cs: &regex::Captures<'t>, name: &'r str) -> Option<i64> {
        cs.name(name)
            .and_then(|x| i64::from_str_radix(x.as_str(), 10).ok())
    }

    let mut rng = rand::thread_rng();

    let roll_outcomes = env::args()
        .skip(1)
        .map(|a| {
            re.captures(&a).and_then(|cs| {
                match (
                    get_val(&cs, "rolls"),
                    get_val(&cs, "die"),
                    get_val(&cs, "mod"),
                ) {
                    (Some(roll_count), Some(die_faces), roll_mod) => Some(
                        iter::repeat(0)
                            .take(roll_count as usize)
                            .map(|_| rng.gen_range(1, die_faces))
                            .sum::<i64>() + roll_mod.unwrap_or(0),
                    ),
                    _ => None,
                }
            })
        })
        .collect::<Vec<Option<i64>>>();

    match (
        env::args().count(),
        // check that all of the rolls were successful, if not – show usage
        roll_outcomes.iter().all(|x| x.is_some()),
    ) {
        (1, _) | (_, false) => println!(
            "roll -- a D'n'D-like dice roller. Handy when you need a random number.

USAGE:
> roll 1d6
3

> roll 1d4+5
6


  How do the D'n'D rolls work?

                  1d4+5
    /-------------^ ^ ^-----------\\
    |               |             |
number of     number of die    modifier
  rolls           faces

  In the example above, we throw a 4-faced die one time,
  then add 5.

  Hint: you can use any number of die faces (unlike in real life).

Enjoy.
"
        ),
        _ => for roll in roll_outcomes.iter().filter_map(|x| *x) {
            println!("{}", roll)
        },
    }
}
