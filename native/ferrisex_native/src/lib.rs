use ferris_says;
use rustler::{Encoder, Env, Error, Term};
use std::io;
use std::str;

mod atoms {
    rustler::rustler_atoms! {
        atom ok;
    }
}

rustler::rustler_export_nifs! {
    "Elixir.FerrisEx.Native",
    [
        ("say", 1, say)
    ],
    None
}

/// A function binding for FerrisEx.Native.say/1.
fn say<'a>(env: Env<'a>, args: &[Term<'a>]) -> Result<Term<'a>, Error> {
    let text: String = args[0].decode()?;

    let mut cursor = io::Cursor::new(Vec::new());
    ferris_says::say(&text.into_bytes(), 24, &mut cursor).unwrap();
    Ok((atoms::ok(), str::from_utf8(&cursor.into_inner()).unwrap()).encode(env))
}
