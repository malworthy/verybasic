# Very Basic

A very stripped back BASIC like language. It contains only what you need to create a turing complete program. No synactic sugar, nothing unnessary. You should be able to learn it in less than 5 minutes.

Currently a work in progress and NOT usable as yet.

This is my first attempt at rust, so is a learning experence. Will be constantly re-factored.

## Control Flow/Looping

Very simple. if/then/else and while.

```
if <expression> then <statement> [else <statement>] end
while <expression> <statement> end

' EXAMPLE:
if x == 1 then
    print("x is one")
else
    print("x is something other than one")
end

x = 0
while x < 10
    print("X is " + x)
    x = x + 1
end
```

## Functions

Functions must be declared before you call them in the top level script. If you don't explicitly return a value, the function will return zero.

Example:

```
function addNumbers(a,b)
    return a+b
end

' prints '2'
print(addNumbers(1,1))
```

## Data types

There are 3 datatypes. String, Number & Boolean. There are no 'true' and 'false' keywords.

```
x = "hello"  ' string
x = 123  ' number floating point
x = (1 == 2) ' boolean, if this case 'false'
```

## Operators

```
==
+
-
/
*
>
>=
<
<=
<>
not
and
or
```

## Comments

Use a single quote for comments. No block comments.

```
' This is a comment
```

## Whitespace

Any character that is not a letter, number or operator is treated as whitespace.

Example below. Semi-colon and other symbols are just ignored.

```
print("hello, world");
!@#$ print("goodbye")
```

## Built-in functions

None as yet.

My intention is that any function call not recognized as a built in or user defined function will be a system call.

e.g. ls("-l) will be the bash command "ls -l".
To override any name clashes and force a system call, prefix function with '@'. e.g. @ls("-l")
