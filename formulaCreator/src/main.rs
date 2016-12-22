extern crate rand;

use std::io::Error;
use std::io::prelude::*;
use rand::Rng;
use std::fs::File;

static AXIOMS: [fn(&String, &String, &String) -> String; 3] = [axiom1, axiom2, axiom3];

fn axiom1(p: &String, q: &String, _: &String) -> String {
    format!{"({0}>({1}>{0}))", p, q}
}

fn axiom2(p: &String, q: &String, r: &String) -> String {
    format!{"(({0}>({1}>{2}))>(({0}>{1})>({0}>{2})))", p, q, r}
}

fn axiom3(p: &String, q: &String, _:&String) -> String {
    format!{"((-{0}>-{1})>({1}>{0}))", p , q}
}


fn negate(p: String) -> String {
    format!("-{}", p)
}

fn create_new_formula(formulas: &[String]) -> String {
    let mut rng = rand::thread_rng();
    let p = rng.choose(formulas).unwrap();
    let q = rng.choose(formulas).unwrap();
    let r = rng.choose(formulas).unwrap();
    let index: usize = rng.gen();
    AXIOMS[index % 3](p,q,r)
}

fn write_result(formulas: Vec<String>) -> Result<(), Error> {
    let mut file = try!(File::create("input.txt"));
    for formula in formulas {
        try!(write!(file, "{}\n", formula));
    };
    Ok(())
}

fn main() {
    let mut formulas: Vec<String> = vec!["p".to_string(), "q".to_string(), "r".to_string(), "(q^-r)".to_string(), "-p".to_string(), "(-pv(q>r))".to_string(), "-(q>p)".to_string()];
    for n in 0..10 {
        let new = create_new_formula(&formulas);
        if n % 2 == 0 {
            formulas.push(negate(new));
        } else {
            formulas.push(new)
        }
    } 
    let result: Vec<String> = formulas.drain(7..).collect(); 
    match write_result(result) {
        Ok(_) => print!(""),
        Err(e) => panic!("ERROR {}", e)
    }
   }
