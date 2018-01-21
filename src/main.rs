type ParsedResult<'a, S> = (S, &'a str);

type ParserOut<'a, S> = Option<ParsedResult<'a, S>>;

macro_rules! pdo {
  ( $input: expr => { let $val: ident <- $func: ident $($e: expr),* ; $($stmt: tt)* } => $out: ident ) => (
    let ($val, _out) = $func(&$input, $($e),*)?;
    pdo!(_out => { $($stmt)* } => $out);
  );

  ( $input: expr => { $func: ident $($e: expr),* ; $($stmt: tt)* } => $out: ident ) => (
    let (_, _out) = $func(&$input, $($e),*)?;
    pdo!(_out => { $($stmt)* } => $out);
  );

  ( $input: expr => {} => $out: ident ) => ( let $out = $input; )
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
              let res <- item;
            } => output);

            if $f(res) {
                Some((res, output))
            } else {
                None
            }
        }
    };
}

sat!(digit, |x: char| x.is_digit(10));
sat!(lower, |x: char| x.is_lowercase());
sat!(upper, |x: char| x.is_uppercase());
sat!(letter, |x: char| x.is_alphabetic());
sat!(alphanum, |x: char| x.is_alphanumeric());

fn character<'a>(out: &'a str, x: char) -> ParserOut<'a, char> {
    let item = item(out)?;
    let (res, _) = item;
    if res == x {
        Some(item)
    } else {
        None
    }
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
      let f <- digit;
      let v <- many digit;
    } => out);

    let mut result = f.to_digit(10)? as _;
    v.iter().for_each(|x| {
        let x = x.to_digit(10).unwrap() as i64;
        result = result * 10 + x;
    });
    Some((result, out))
}

fn parse<'a>(input: &'a str) -> ParserOut<'a, i64> {
    pdo!(input => {
      character 'a';
      let num <- number;
    } => output);

    Some((num, output))
}

fn main() {
    println!("{:?}", parse("a123784bcd"));
}
