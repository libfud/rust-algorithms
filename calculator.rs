#![crate_id = "rcalc"]
#![crate_type = "bin"]

//! Polish notation calculator.

use std::io;

static bad_operator : &'static str = "Improperly placed or missing operator!";
static bad_expr : &'static str = "Poorly formatted expression!";
static despair: &'static str =
"I don't quite know what happened here. Maybe something very illogical. Maybe an oversight on my part. But this is going to be a rough day if this string is printed.";

///Wrapper to evaluate a given expression. Checks to make sure that it's a
///valid expression, then does the appropriate action given the operator.
pub fn eval(expr: &str) -> ~str {
    if validate(expr) == false { return bad_expr.to_owned() }
    let (operator, terms) = tokenize(expr);
    let answer = match operator.slice_from(0) {
        "+" => add(&terms),
        "-" => sub(&terms),
        "*" => mul(&terms),
        "/" => div(&terms),
        "%" => rem(&terms),
        _   => bad_expr.to_owned() 
    };

    answer
}

fn validate(expr: &str) -> bool {
    let mut lparenth = 0;
    let mut rparenth = 0;
    let mut operators = 0;
    let mut char_counter = 0;
    for c in expr.chars() {
        match c {
            '0'..'9' | ' ' => { },
            '(' => { lparenth += 1 },
            ')' => { 
                if lparenth - 1 == rparenth && char_counter + 1 < expr.len() {
                    return false
                } else {
                    rparenth += 1;
                }
            },
            '+' | '-' | '*' | '/' | '%' => { operators += 1 },
            _   => { 
                return false }
        }
        char_counter += 1;
    }
    if lparenth != rparenth { return false }
    if lparenth != operators { return false }

    true
}

fn tokenize(expr: &str) -> (~str, Vec<f64>) {
    let mut terms = Vec::new();
    let mut buf = StrBuf::new();
    let mut point_flag = false;
    let mut counter = 0;
    let mut skip = 0;

    let operator: &str = expr.slice_from(1).words().next().unwrap();
    // This essentially forbids terms from touching operators. Which I am
    // 100% fine with.
    match operator {
        "+" | "-" | "*" | "/" | "%" => { },
        _   => { return (bad_operator.to_owned(), terms) }
    }

    for c in expr.slice_from(operator.len() + 1).chars() {
        if skip != 0 { skip -= 1 }
        else  {
            match c {
                //We already have an operator. > 1 operator is forbidden
                '+' | '-' | '*' | '/' | '%' => {
                    return (bad_operator.to_owned(), terms)
                },

                '0'..'9'    => { buf.push_char(c) },

                '.'         => { 
                    if point_flag == false {
                        point_flag = true;
                        //prepend a zero in preparation for the cast
                        if buf.len() == 0 { buf.push_char('0') }
                        buf.push_char(c);
                    } else {
                        return ("Improperly formatted term!".to_owned(), terms)
                    }
                },

                //This potentially signifies the end of a number.
                ' '         => {
                    if buf.len() > 0 {
                        match from_str::<f64>(buf.to_str()) {
                            Some(num)   => terms.push(num),
                            _           => {
                                println!("{}", despair);
                                return (operator.to_owned(), terms)
                            }
                        }
                        buf = "".to_strbuf(); 
                    }
                },
                
                //the start of another expression
                '('         => {
                    //make sure to add the number in the buffer to the vector
                    //if it's present
                    if buf.len() > 0 {
                        match from_str::<f64>(buf.to_str()) {
                            Some(num)   => { terms.push(num) },
                            _           => {
                                println!("{}", despair);
                                return (bad_operator.to_owned(), terms);
                            }
                        }
                        buf = "".to_strbuf();
                    }

                    let term_len = find_sub_expr_len(expr.slice_from(
                    counter + 1)); // omit the leading parenthesis

                    //evaluating only the slice of the expression holding the
                    //sub expression
                    let term = eval(expr.slice(counter, 
                        counter + term_len + 1));

                    //account for failure
                    if term == bad_expr.to_owned() {
                        return (bad_expr.to_owned(), terms)
                    }
                    
                    match from_str::<f64>(term) {
                        Some(num)   => { terms.push(num) },
                        _           => {
                            println!("{}", despair);
                            return (operator.to_owned(), terms)
                        }
                    }
                    
                    //Account for the subexpression's length
                    counter += term_len;
                    skip = term_len; //an inelegant, ugly hack
                },

                ')'         => {
                    if buf.len() > 0 {
                        match from_str::<f64>(buf.to_str()) {
                            Some(num)   => { terms.push(num) },
                            _           => {
                                println!("{}", despair);
                                return (bad_operator.to_owned(), terms);
                            }
                        }
                        buf = "".to_strbuf();
                    }
                }

                _           => { // this is fine, this should catch bad inputs
                    return (bad_expr.to_owned(), terms);
                }
            }
            counter += 1;
        }
    }

    (operator.to_owned(), terms)
}

fn find_sub_expr_len(expr: &str) -> uint {
    let mut len = 1;
    let mut lparenth = 1;
    let mut rparenth = 0;
    for c in expr.slice_from(1).chars() {
        match c {
            '(' => { lparenth += 1; },
            ')' => { rparenth += 1; },
            _   => { }
        }
        len += 1;
        if rparenth == lparenth { break }
    }
    len 
}

fn add(terms: &Vec<f64>) -> ~str {
    let total = terms.iter().fold(0.0, |total, &term| total + term);

    total.to_str().to_owned()
}

fn sub(terms: &Vec<f64>) -> ~str {
    if terms.len() < 1 {
        println!("Subtraction requires at least one term!");
        return bad_expr.to_owned()
    } 
    let def = terms.as_slice()[0];
    if terms.len() == 1 { return (0f64 - def).to_str().to_owned() }
    let diff = terms.slice_from(1).iter().fold(def, |diff, &term| diff - term);

    diff.to_str().to_owned()
}

fn mul(terms: &Vec<f64>) -> ~str {
    let product = terms.iter().fold(1.0, |product, &term| product * term);
    
    product.to_str().to_owned()
}

fn div(terms: &Vec<f64>) -> ~str {
    if terms.len() < 1 {
        println!("Division requires at least one term!");
        return bad_expr.to_owned()
    }
    if terms.len() == 1 { 
        return (1f64 / terms.as_slice()[0]).to_str().to_owned();
    }
    let def = terms.as_slice()[0];
    let quot = terms.slice_from(1).iter().fold(def ,|quot,&term| quot / term);

    quot.to_str().to_owned()
}

fn rem(terms: &Vec<f64>) -> ~str {
    if terms.len() < 1 {
        println!("Modulus operations require at least two terms!");
        return bad_expr.to_owned()
    }
    if terms.len() == 1 { return "1".to_owned() } // 1 % anything = 1 }
    let def = terms.as_slice()[0];
    let rem = terms.slice_from(1).iter().fold(def, |rem, &term| rem % term);

    rem.to_str().to_owned()
}

fn main() {
    let mut reader = io::stdin();
    let mut expr;

    loop {
        expr = reader.read_line().ok().unwrap_or("exit ".to_owned());
        if expr.trim() == "exit".to_owned() { break }
        let output = eval(expr.trim());
        println!("{}", output);
    }
}
