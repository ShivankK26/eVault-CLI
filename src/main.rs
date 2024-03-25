mod db;
use db::*;

fn clr() {
    println!("{}[2J", 27 as char);
}

fn main() {
    println!("Hello, world!");
}
