use monkey::repl::start_repl;

fn main() {
    let stdin  = std::io::stdin();
    let mut stdin  = std::io::BufReader::new(stdin);
    let stdout = std::io::stdout();
    let mut stdout = std::io::BufWriter::new(stdout);
    start_repl(&mut stdin, &mut stdout);
}
