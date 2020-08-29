use std::env;
use calc::Config;
use std::process;


fn main() {
    let mut config = Config::new(env::args()).unwrap_or_else(|err|{
        eprintln!("Invalid input: {}", err);
        process::exit(1);
    });

    let expression = config.parse_expression();
    let rpn_expression = calc::rpn(expression.iter().map(|s| &**s).collect());
    let result = calc::calculate(rpn_expression);
    println!("{}", result.unwrap());
}
