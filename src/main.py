import sys
import subprocess
import Tokenisation
import Parsing
import Generation

if len(sys.argv) == 1:
    print("No arguments provided")
    sys.exit(1)

path = sys.argv[1]

with open(path, 'r', encoding='utf-8') as file:
        content = file.read()
print(content)

source = Tokenisation.Tokeniser(content)
tokens = source.tokens

parsed = Parsing.Parser(tokens)
tree = parsed.parse()

generator = Generation.Generator(tree)
asm = generator.generate()
print(asm)
with open("lilac.asm", "w") as file:
    file.write(asm)
nasm_command = ["nasm", "-f", "elf64", "lilac.asm"]
subprocess.run(nasm_command, check=True)
ld_command = ["ld", "-o", "lilac", "lilac.o"]
subprocess.run(ld_command, check=True)
