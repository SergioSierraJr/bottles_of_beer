use std::env;
use std::str::FromStr;

fn main() {
    let mut bottles_of_beer: i32;
    if env::args().len() > 2 {
        match i32::from_str(&env::args().collect::<Vec<_>>()[1]) {
            Ok(..) => bottles_of_beer = i32::from_str(&env::args().collect::<Vec<_>>()[1]).unwrap(),
            _err => {
                println!("That is not a number!");
                return;
            }
        }
    } else {
        bottles_of_beer = 99;
    }

    while bottles_of_beer >= 0 {
        if bottles_of_beer == 0 {
            println!("Now theres no more bottles of beer on the wall!");
            return;
        }
        println!("{} Bottles of beer on the wall, {} bottles of beer!", bottles_of_beer, bottles_of_beer);
        bottles_of_beer -= 1;
        println!("Take one down and pass it around, now theres {} bottles of beer on the wall!", bottles_of_beer)
    }
}
