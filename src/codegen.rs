use crate::parser::{Expr, Parser};

pub struct CodeGen {
    ptr: usize,
    variables: usize,
    loop_ptr: usize,
    loop_count: usize,
    expr_stream: Parser,
}

impl CodeGen {
    pub fn new(expr_stream: Parser) -> Self {
        Self {
            ptr: 1,
            variables: 1,
            loop_ptr: 0,
            loop_count: 0,
            expr_stream,
        }
    }

    pub fn gen(&mut self) -> Vec<String> {
        let mut code = vec![
            "global _start\n\nsection .text\n_start:".to_string(),
            "\t; prelude\n\tpush ebp\n\tmov ebp, esp\n\n\t; body".to_string(),
        ];
        while let Some(expr) = self.expr_stream.next() {
            match expr {
                Expr::Inc(n) => code.push(format!("\tadd DWORD [ebp-{}], {n}", self.ptr * 4)),
                Expr::Dec(n) => code.push(format!("\tsub DWORD [ebp-{}], {n}", self.ptr * 4)),
                Expr::IncPtr(n) => {
                    self.ptr += n;
                    if self.ptr > self.variables {
                        self.variables += self.ptr - self.variables;
                    };
                }
                Expr::DecPtr(n) => self.ptr -= n,
                Expr::StartLoop => {
                    self.loop_count += 1;
                    self.loop_ptr += 1;
                    code.push(format!("loop_{}:", self.loop_count));
                }
                Expr::EndLoop => {
                    code.push(format!(
                        "\tcmp DWORD [ebp-{}], 0\n\tjnz loop_{}\n",
                        self.ptr * 4,
                        self.loop_ptr
                    ));
                    self.loop_ptr -= 1;
                    if self.loop_ptr == self.loop_count - 1 {
                        self.loop_ptr = self.loop_count;
                    }
                }
                Expr::Print => code.push(format!(
                    "\n\t; print to stdout\n\tpush DWORD [ebp-{}]\n\tmov eax, 4\n\tmov ebx, 1\n\tmov ecx, esp\n\tmov edx, 1\n\tint 0x80\n\tadd esp, 4",
                    self.ptr * 4,
                )),
                Expr::Read => code.push(format!("\t; syscall shit")),
            };
        }
        code.push(format!(
            "\n\t; epilogue\n\txor ebx, ebx\n\tmov eax, 1\n\tint 0x80"
        ));
        code.insert(2, format!("\tsub esp, {}", self.variables * 4));
        code
    }
}
