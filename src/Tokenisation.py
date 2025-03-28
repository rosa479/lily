tokenList = [
    'RETURN',
    'NUMBER',
    'SEMI',
    'OPEN_PAREN',
    'CLOSE_PAREN'
]

class Token:
    def __init__(self, type, value=None):
        self.type = type
        self.value = value

class Tokeniser():
    def __init__(self, src):
        self.text = src
        self.tokens = self.tokenise()
         
    def peek(self, idx, chars=1):
        return self.text[idx:idx+chars] if idx+chars <= len(self.text) else None

    def consume(self, idx):
        c = self.text[idx]
        idx+=1
        return c, idx
        
    def tokenise(self):
        buffer = ""
        tokens = []
        idx = 0
        while(self.peek(idx)):
            if self.peek(idx).isspace():
                idx += 1
            elif self.peek(idx).isdigit():
                while self.peek(idx) and self.peek(idx).isdigit():
                    c, idx = self.consume(idx)
                    buffer += c
                tokens.append(Token('NUMBER', int(buffer)))
                buffer = ""
            elif self.peek(idx).isalnum():
                while self.peek(idx) and self.peek(idx).isalnum():
                    c, idx = self.consume(idx)
                    buffer += c
                if buffer == 'return':
                    tokens.append(Token('RETURN'))
                    buffer = ""
                else:
                    raise Exception(f"Invalid token: {buffer}")
            elif self.peek(idx) == ';':
                tokens.append(Token('SEMI'))
                idx += 1
            elif self.peek(idx) == '(': 
                tokens.append(Token('OPEN_PAREN'))
                idx += 1
            elif self.peek(idx) == ')':
                tokens.append(Token('CLOSE_PAREN'))
                idx += 1
            else:
                raise Exception(f"Invalid token: {self.peek(idx)}")
        
        print([[t.type, t.value] for t in tokens])
        return tokens
                
