# Very Basic

A very stripped back BASIC like language. It contains only what you need to create a turing complete program. No synactic sugar, nothing unnessary. You should be able to learn it in less than 5 minutes.

## Compiling

On linux you may need to install the following dependencies

```
sudo apt install libfontconfig libfontconfig1-dev
sudo apt install pkg-config
```

then simply

```
git clone https://github.com/malworthy/verybasic.git
cd verybasic_rust
cargo build
```

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

Using 'else if' like below will not work and results in a compile error. Very basic is too basic to understand this.

```
if x == 1 then
    print("hello")
else if x ==2 then
    print("won't work)
end
```

instead re-write as

```
if x == 1 then
    print("hello")
else
    if x ==2 then
        print("won't work)
    end
end
```

Note: The expression evaluated for if and while must return a boolean. For example `if 1 then print("hello") end` won't work.

## Functions

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
^ (to the power of)
mod (remainder)
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
!#$ print("goodbye")
```

## Built-in functions

### _array([element],...)_

creates a new array, optionally populating with elements

### _command()_

returns command line arguments as an array.  

NOTE: The first argument will be the full path of vbas.exe, 2nd argument name of the script. 

### _print(string, [newline=true], [colour=""])_

Prints a string to the console.

### _input([prompt=""])_

reads input from the console

### _seconds()_

returns the number of seconds since the unix epoch

### _dir(pattern)_

will query the filesystem for all files that match a particular pattern. Uses Unix shell style patterns.

### _rand()_

returns a random number between 0 and 1

### _readlines(filename)_

returns an array of all lines in a text file

### _write(filename, text)_

creates a new file writes text to it. Will overwrite any existing file.

### _append(filename, text)_

appends text to a file. If the file doesn't exist it will be created.

### _str(value)_

Converts any value to a string

### _val(string)_

Converts a string to a number. Will return zero if string cannot be converted to a number, or if the data type is not a string.

### _chr(ascii_value)_

Returns a one character string using ascii encoding. Invalid ascii value will return an empty string.

### _floor(number)_

Returns the largest integer not greater than _number_

## String function

### _instr()_

### _lcase()_

### _left(string, length)_

Returns a substring containing a specified number of characters from the beginning (left side) of a string.

### _mid(string, start, [length])_

returns part of a string using a 1 based index.
e.g mid("hello",3) returns "llo", mid("hello",3, 2) return "lo"

abcdefghijklmopqrstuvwxyz

### _right()_

### _ucase()_

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

returns a hex code for the specified red, green and blue values.

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
