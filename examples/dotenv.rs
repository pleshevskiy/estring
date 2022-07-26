use estring::{EString, Pair, SepVec, Trim};

const DOTENV_CONTENT: &str = "
DATABASE_URL=postgres://user:password@localhost:5432/recipes
APP_HOST=http://localhost:3000
";

fn main() -> estring::Result<()> {
    EString::from(DOTENV_CONTENT)
        .parse::<Trim<SepVec<Pair<&str, '=', &str>, '\n'>>>()?
        .iter()
        .for_each(|p @ Pair(key, value)| {
            println!("pair: {}", p);

            std::env::set_var(key, value);
        });

    println!(
        "envs: {:#?}",
        std::env::vars()
            .filter(|(k, ..)| ["DATABASE_URL", "APP_HOST"].contains(&k.as_str()))
            .collect::<Vec<_>>()
    );

    Ok(())
}
