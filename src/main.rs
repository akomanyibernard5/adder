use sexp::*;
use sexp::Atom::*;
use std::env;
use std::fs::File;
use std::io::prelude::*;

// AST representation for Adder language expressions
enum Expr {
    Num(i32),
    Add1(Box<Expr>),
    Sub1(Box<Expr>),
    Negate(Box<Expr>),
}

// Parse S-expression into AST
fn parse_expr(s: &Sexp) -> Expr {
    match s {
        // Handle integer literals
        Sexp::Atom(I(n)) => {
            Expr::Num(i32::try_from(*n).unwrap())
        }
        // Handle operations (add1, sub1, negate)
        Sexp::List(vec) => match &vec[..] {
            [Sexp::Atom(S(op)), e] if op == "add1" => Expr::Add1(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "sub1" => Expr::Sub1(Box::new(parse_expr(e))),
            [Sexp::Atom(S(op)), e] if op == "negate" => Expr::Negate(Box::new(parse_expr(e))),
            _ => panic!("Invalid expression"),
        },
        _ => panic!("Invalid expression"),
    }
}

// Compile AST to x86-64 assembly (result in rax)
fn compile_expr(e: &Expr) -> String {
    match e {
        Expr::Num(n) => format!("  mov rax, {}", *n),
        Expr::Add1(subexpr) => format!("{}\n  add rax, 1", compile_expr(subexpr)),
        Expr::Sub1(subexpr) => format!("{}\n  sub rax, 1", compile_expr(subexpr)),
        // Use neg instruction for two's complement negation
        Expr::Negate(subexpr) => format!("{}\n  neg rax", compile_expr(subexpr)),
    }
}

fn main() -> std::io::Result<()> {
    let args: Vec<String> = env::args().collect();
    if args.len() < 3 {
        eprintln!("Usage: {} <input.snek> <output.s>", args[0]);
        std::process::exit(1);
    }

    let in_name = &args[1];
    let out_name = &args[2];

    let mut in_file = File::open(in_name)?;
    let mut in_contents = String::new();
    in_file.read_to_string(&mut in_contents)?;

    let sexp = parse(&in_contents).unwrap();
    let expr = parse_expr(&sexp);

    let body = compile_expr(&expr);

    let asm_program = format!(
        "section .text
global our_code_starts_here
our_code_starts_here:
{body}
  ret
"
    );

    let mut out_file = File::create(out_name)?;
    out_file.write_all(asm_program.as_bytes())?;

    Ok(())
}