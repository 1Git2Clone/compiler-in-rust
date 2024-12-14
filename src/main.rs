mod prelude;
mod run;
mod utils;

use crate::prelude::*;

fn main() -> Result<(), Error> {
    #[cfg(feature = "interpreter")]
    loop {
        let mut user_input = String::new();
        std::io::stdin().read_line(&mut user_input)?;
        user_input = user_input.trim().to_string();

        if user_input == "exit" {
            break;
        }

        let ast = &parse(&tokenize(&user_input)?)?;
        if let Some(valid_ast) = ast {
            println!("{:?}", compile(valid_ast));
        }
    }

    #[cfg(not(feature = "interpreter"))]
    {
        println!("{:?}", compile(&parse(&tokenize("3 + 2 * 6")?)?.unwrap()));
        println!("{:?}", compile(&parse(&tokenize("(3 + 2) * 6")?)?.unwrap()));
        println!("{:?}", parse(&tokenize("(3 + 2) * 6")?));
        println!("Compilation finished successfully!");
    }

    Ok(())
}
