mod eval;
mod lexer;
mod parser;
mod preproc;

use std::panic;

fn main() {
    // HOLY FUCK A CUSTOM PANIC HOOK?!?!?!
    panic::set_hook(Box::new(|info| {
        // replace this print statement when implementing multithreading
        println!("\x1b[1;31m╭ q9::panic\x1b[0m\x1b[1m - main thread panicked:\x1b[0m");
        if let Some(s) = info.payload().downcast_ref::<String>() {
            println!("\x1b[1;31m╰-->\x1b[0m {}", s);
        } else {
            println!("(non-printable panic) {:?}", info.payload().type_id());
        }
    }));

    // actual main
    let mut parser = parser::Parser::new(
        r#"
        let x = 2
        "#,
    );
    
    let mut evalu = eval::Q9Eval::new(parser.parse().unwrap());
    evalu.eval(evalu.prog.content.clone(), None);
}
