use std::fmt;

struct ParsedResult<'a, S> {
    res: S,
    out: &'a str,
}

impl<'a, S> fmt::Debug for ParsedResult<'a, S>
where
    S: fmt::Display,
{
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "ParsedResult {{ res: {}, out: {} }}", self.res, self.out)
    }
}

type ParserOut<'a, S> = Option<ParsedResult<'a, S>>;

impl<'a, S> ParsedResult<'a, S> {
    fn new(s: &'a str) -> ParsedResult<'a, ()> {
        ParsedResult { res: (), out: s }
    }
}

fn item<'a, S>(out: &ParsedResult<'a, S>) -> ParserOut<'a, char> {
    let &ParsedResult { ref out, .. } = out;

    if out.is_empty() {
        None
    } else {
        let (res, out) = out.split_at(1);
        Some(ParsedResult {
            res: res.chars().next().unwrap(),
            out: out,
        })
    }
}

macro_rules! sat {
    ($n: ident, $f: expr) => {
        fn $n<'a, S>(out: &ParsedResult<'a, S>) -> ParserOut<'a, char> {
            let item = item(out)?;
            if $f(item.res) {
                Some(item)
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

fn char<'a, S>(out: &ParsedResult<'a, S>, x: char) -> ParserOut<'a, char> {
    let item = item(out)?;
    if item.res == x {
        Some(item)
    } else {
        None
    }
}

/*fn many<'a, S, F, T, U>(out: &ParsedResult<'a, S>, p: F) -> ParserOut<'a, Vec<U>>
where
    F: Fn(&ParsedResult<'a, T>) -> U,
{
    let result = vec![];
    let mut output = *out;
    let mut parse_result = Some(output);

    while let Some(o) = parse_result {
        output = o;
        parse_result = p(&output);
    }

    Some(ParsedResult { res: result, out: output.out })
}*/

fn parse<'a>(input: &ParsedResult<'a, ()>) -> ParserOut<'a, char> {
    let output = char(&input, 'a')?;

    Some(output)
}

fn main() {
    let input = ParsedResult::<()>::new("abcd");
    println!("{:?}", parse(&input));
}
