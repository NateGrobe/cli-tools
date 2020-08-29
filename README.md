# CLI Tools
Some CLI apps I created to practice rust.

### Todo
A program that searches through files in a folder recursively and displays any 
comments I have marked as TODO.

##### Example:
```bash
todo # searches through all files starting in the current working directory
todo . # the same as calling todo
todo ~/path/to/dir # runs todo in the specified folder
```

### Calc
A simple BEDMAS calculator program. This implementation uses the __shunting-yard
algorithm__ to convert the input into __reverse polish notation__. From there it
is fairly simple to calculate the result by adding values to a stack and 
performing an operation on the first two elements of the stack.

##### Example:
```bash
calc "(3+4)(5+6)"
77

calc "3+4* 2/(1-5)^2" # note spaces are ignored
3.5
```
