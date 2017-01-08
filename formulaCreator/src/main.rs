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

fn create_new_valid(formulas: &[String]) -> String {
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

fn conj_formula(p: &String, q: &String, negated: bool) -> String {
    if negated {
        format!{"-({}^{})", p, q}
    } else {
        format!{"({}^{})",p ,q}
    }
}

fn disj_formula(p: &String, q: &String, negated: bool) -> String {
    if negated {
        format!{"-({}v{})", p, q}
    } else {
        format!{"({}v{})",p ,q}
    }

}
fn impl_formula(p: &String, q: &String, negated: bool) -> String {
    if negated {
        format!{"-({}>{})", p, q}
    } else {
        format!{"({}>{})",p ,q}
    }
}

fn create_random_formulas() -> Vec<String> {
    let mut rng = rand::thread_rng();
    let mut formulas: Vec<String> = vec!["p".to_string(), "q".to_string(), "r".to_string(), "-p".to_string(), "-q".to_string(), "-r".to_string()];
    let connectives = [conj_formula, disj_formula, impl_formula];
    for _ in 0..300 {
        let result: String;
        {
            let p = rng.choose(&formulas).unwrap();
            let q = rng.choose(&formulas).unwrap();
            let conn = rng.choose(&connectives).unwrap();
            let i: i32 = rng.gen();
            result = conn(p,q, i % 2 == 0);
        }
        if result.len() > 10 {continue;} else {formulas.push(result);}
    }
    return formulas;
}

fn main() {
    let formulas = create_random_formulas();
    let mut valid: Vec<String> = vec![];
    for n in 0..10 {
        let mut new = create_new_valid(&formulas);
        while new.len() >= 50 {
            new = create_new_valid(&formulas);
        }
        if n % 2 == 0 {
            valid.push(negate(new));
        } else {
            valid.push(new)
        }
    }
    match write_result(valid) {
        Ok(_) => print!(""),
        Err(e) => panic!("ERROR {}", e)
    }
   }
