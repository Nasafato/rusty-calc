import unittest

ORDER = [
    ("^"),
    ("*"),
    ("+")
]
ORDER_TABLE = { token: precedence for precedence, token in enumerate(ORDER) }

def infix_to_prefix(tokens):
    stack = []
    operators = []
    for token in tokens:
        if is_op(token) and len(operators) == 0:
            operators.append(token)
        elif is_op(token) and is_higher_order(operators[-1], token):
            stack.append(add_expression(stack, operators))
            operators.append(token)
        elif is_op(token):
            operators.append(token)
        else:
            stack.append(token)
            
    for _ in range(len(operators)):
        stack.append(add_expression(stack, operators))
    return stack[0]

def is_op(token):
    return token in "^*+"

def is_higher_order(left, right):
    return ORDER_TABLE[left] < ORDER_TABLE[right]

def add_expression(stack, ops, op_to_save=None):
    right = stack.pop()
    left = stack.pop()
    op = ops.pop()
    return  "{}{}{}".format(op, left, right)


class TestInfixToPrefix(unittest.TestCase):
    def test(self):
        tests = [
            ("2+3*5", "+2*35"),
            ("2*3+5", "+*235"),
        ]
        for tokens, solution in tests:
            self.assertEqual(infix_to_prefix(tokens), solution)

if __name__ == "__main__":
    unittest.main()