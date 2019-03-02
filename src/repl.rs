use crate::token::Token;
use crate::lexer::Lexer;

const PROMPT: &'static str = ">> ";

pub fn start_repl<R, W>(reader: &mut R, writer: &mut W)
    where R: std::io::BufRead,
          W: std::io::Write,
{
    let mut src = String::new();

    loop {
        write!(writer, "{}", PROMPT).unwrap();
        writer.flush().unwrap();
        //reader.read_to_string(&mut src).unwrap(); // ファイルを読むならこっちがいいかも
        reader.read_line(&mut src).unwrap(); // REPLならこちら
        if src.len() == 0 {
            writeln!(writer, "").unwrap();
            writer.flush().unwrap();
            continue;
        }
        let mut lex: Lexer = Lexer::new(src.chars());
        let mut toks: Vec<Token> = Vec::new();
        loop {
            let tok = lex.next_token();
            toks.push(tok.clone());
            if tok == Token::EOF {
                break;
            }
        }
        writeln!(writer, "{:?}", toks).unwrap();
        writer.flush().unwrap();
        src.clear();
    }
}
