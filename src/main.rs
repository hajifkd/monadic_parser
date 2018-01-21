type ParsedResult<'a, S> = (S, &'a str);

type ParserOut<'a, S> = Option<ParsedResult<'a, S>>;

macro_rules! pdo {
    ( $input: expr => { let $val: ident <- return $e: expr ; $($stmt: tt)* } ) => {{
        let $val = $e;
        pdo!($input => { $($stmt)* })
    }};

    ( $input: expr => { let $val: ident <- $func: ident ($($e: expr),*) ; $($stmt: tt)* } ) => {{
        let ($val, _out) = $func(&$input, $($e),*)?;
        pdo!(_out => { $($stmt)* })
    }};

    ( $input: expr => { return $e: expr ; $($stmt: tt)* } ) => {{
        Some(($e, $input));
        pdo!($input => { $($stmt)* })
    }};

    ( $input: expr => { $func: ident ($($e: expr),*) ; $($stmt: tt)* } ) => {{
        let (_, _out) = $func(&$input, $($e),*)?;
        pdo!(_out => { $($stmt)* })
    }};

    ( $input: expr => { return $e: expr } ) => {{
        if let Some(_r) = $e {
            Some((_r, $input))
        } else {
            None
        }
    }};

    ( $input: expr => { $func: ident ($($e: expr),*) } ) => {{
        $func(&$input, $($e),*)
    }};

    ( $input: expr => {} ) => {{ Some(((), $input)) }}
}

fn item<'a>(out: &'a str) -> ParserOut<'a, char> {
    if out.is_empty() {
        None
    } else {
        let (res, out) = out.split_at(1);
        Some((res.chars().next().unwrap(), out))
    }
}

macro_rules! sat {
    ($n: ident, $f: expr) => {
        fn $n<'a>(input: &'a str) -> ParserOut<'a, char> {
            pdo!(input => {
                let res <- item();
                return if $f(res) {
                    Some(res)
                } else {
                    None
                }
            })
        }
    };
}

sat!(digit, |x: char| x.is_digit(10));
sat!(lower, |x: char| x.is_lowercase());
sat!(upper, |x: char| x.is_uppercase());
sat!(letter, |x: char| x.is_alphabetic());
sat!(alphanum, |x: char| x.is_alphanumeric());

fn character<'a>(input: &'a str, x: char) -> ParserOut<'a, char> {
    pdo!(input => {
        let res <- item();
        return if res == x {
            Some(res)
        } else {
            None
        }
    })
}

fn many<'a, F, T>(out: &'a str, p: F) -> ParserOut<'a, Vec<T>>
where
    F: Fn(&'a str) -> ParserOut<'a, T>,
{
    let mut result = vec![];
    let mut output = out;
    let mut parse_result = p(output);

    while let Some(o) = parse_result {
        let (res, out) = o;
        output = out;
        result.push(res);
        parse_result = p(output);
    }

    Some((result, output))
}

fn number<'a>(input: &'a str) -> ParserOut<'a, i64> {
    pdo!(input => {
      let f <- digit();
      let v <- many(digit);
      return {
          Some(v.iter().fold(f.to_digit(10).unwrap() as _,
                             |acc, &x| acc * 10 + x.to_digit(10).unwrap() as i64))
      }
    })
}

fn parse<'a>(input: &'a str) -> ParserOut<'a, i64> {
    pdo!(input => {
        character('a');
        number()
    })
}

fn main() {
    println!("{:?}", parse("a123784bcd"));
}
