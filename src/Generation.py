import Parsing

class Generator:
    def __init__(self, parsed):
        self.parsed = parsed
        self.asm = "global _start\n_start:\n"

    def generate(self):
        self.asm += "   mov rax, 60\n"
        self.asm += "   mov rdi, " + str(self.parsed[0].expr.value) + "\n"
        self.asm += "   syscall\n"
        return self.asm
