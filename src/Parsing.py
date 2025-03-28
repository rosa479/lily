import Tokenisation

class Node:
    def __init__(self, type):
        self.type = type

class NodeEXPR(Node):
    def __init__(self, value):
        super().__init__("EXPR")
        self.value = value

class NodeRETURN(Node):
    def __init__(self, expr):
        super().__init__("RETURN")
        self.expr = expr

class Parser:
    def __init__(self, tokens):
        self.tokens = tokens
        self.idx = 0

    def peek(self, chars=1):
        if chars == 1:
            return self.tokens[self.idx] if self.idx < len(self.tokens) else None
        return self.tokens[self.idx:self.idx+chars] if self.idx+chars <= len(self.tokens) else None

    def consume(self):
        t = self.tokens[self.idx]
        self.idx += 1
        return t

    def parseExpression(self):
        token = self.consume()
        if token.type == "NUMBER":
            return NodeEXPR(token.value)
        raise Exception("Invalid expression")

    def parse(self):
        nodes = []
        while self.idx < len(self.tokens):
            token = self.consume()
            if token.type == "RETURN" and self.peek().type == "OPEN_PAREN":
                self.consume()
                expr = self.parseExpression()
                if self.peek() == None or self.peek().type != "CLOSE_PAREN" :
                    raise Exception("Expected closing parenthesis")
                else:
                    self.consume()
                if self.peek() == None or self.peek().type != "SEMI":
                    raise Exception("Expected semicolon")
                else:
                    self.consume()
                nodes.append(NodeRETURN(expr))
            else:
                raise Exception("Unexpected token")
        return nodes

