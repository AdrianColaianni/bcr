# BCR

Work in progress [POSIX bc](https://en.wikipedia.org/wiki/Bc_%28programming_language%29) interpreter

Written not for performace, use, or utility but because I'm bored.

Supports
- [X] Order of operations
- [ ] Expressions
	- [ ] Variables
	- [ ] Arrays
	- [X] Negation
	- [ ] Pre/post inc/decrement operators (i.e. `a++`, `--a`)
	- [X] Addition / subtraction
	- [X] Multiplication / division
	- [X] Modulus
	- [X] Power
	- [X] Parenthesis
	- [ ] Assignment operator
	- [ ] `var <opt>= expr`
	- [X] Relational expressions (i.e. `>`, `<=`)
	- [X] Boolean operators (`!`, `||`, and `&&`)
	- [ ] Standard functions (`length(exp)`, `read()`, `scale(exp)`, and `sqrt(exp)`)
- [ ] Statements
	- [ ] Expressions
		- [ ] Assignment
	- [ ] String
	- [ ] `print`
	- [ ] Statement lists
	- [ ] `if`, `while`, and `for`
	- [ ] `break`, `continue`, `halt`
	- [ ] `return`
- [ ] Pseudo statements (`limits`, `quit`, `warranty`)
- [ ] Functions
- [ ] Math lib
