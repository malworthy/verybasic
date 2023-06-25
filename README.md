# Very Basic

A very stripped back BASIC like language. It contains only what you need to create a turing complete program. No synactic sugar, nothing unnessary. You should be able to learn it in less than 5 minutes.

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

Functions must be declared before you call them in the top level script.
Functions will return the result of the last expression executed. All functions must return a value. An empty function body will be a compile error.

Example:

```
function addNumbers(a,b)
    a+b
end

' prints '2'
print(addNumbers(1,1))
```

To exit a function, use the keyword exit as below.

```
' the function below returns zero if x is less than zero.  Otherwise returns the value of x.
function foo(x)
    if x < 0 then
        0 exit
    end if
    x
end

```

## Data types

There are 4 datatypes. String, Number, Boolean and Array. There are no 'true' and 'false' keywords.
Arrays can hold any datatype

```
x = "hello"  ' string
x = 123  ' number floating point
x = (1 == 2) ' boolean, if this case 'false'
x = array(1,1,1) ' create an array of 3 elements with the number 1
```

## Arrays

Arrays are immutable. You can read an element as follows:

```
x = array(4,5,6)
print(x[0]) ' prints 4
print(x[1]) ' prints 5
print(x[1]) ' prints 6
```

There is no ability to change values in an array. Something like `x[0] = 0` won't work.

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

### _print(string, [newline=true], [colour=""])_

Prints a string to the console.

### _input([prompt=""])_

reads input from the console

### _array([element],...)_

creates a new array, optionally populating with elements

### _seconds()_

returns the number of seconds since the unix epoch

### _dir(pattern)_

will query the filesystem for all files that match a particular pattern. Uses Unix shell style patterns.

### _rand()_

returns a random number between 0 and 1

### _readlines(filename)_

returns an array of all lines in a text file

## Graphic functions

Very Basic has the ability to do basic 2D graphics. You can draw to a canvas and then display the canvas in a window, or save it as a image file.

### _initgraphics(width, height)_

creates a new canvas for drawing. You must call this before calling any other graphics functions.

### _cleargraphics()_

sets all pixels on the canvas to white

### _plot(x, y, colour)_

sets the colour at x,y coordinates on the canvas. Colour can either be a pre-defined colour string or a hex code (e.g. #ADFF2F)

### _window()_

display a window showing the canvas. Size of window will be the same as the canvas

### _rgb(red,green,blue)_

returns a hex code for the speficied red, green and blue values.

### Colours

the following pre-defined string values for colours are available:

- darkblue
- blue
- purple
- yellow
- pink
- red
- green
- black
- white

Other colours can be specified using Hex Codes. A hex code starts with '#' and must be 6 digits.

## System calls

Any function call not recognized as a built in or user defined function will be a system call.

e.g. ls("-l) will be the bash command "ls -l".

To override any name clashes and force a system call, prefix function with '@'. e.g. @ls("-l")
