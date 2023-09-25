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

### if/then/else

```
if <expression> then <statement> [else <statement>][elseif <expression> then <statement>] end
while <expression> <statement> end

' EXAMPLE:
if x == 1 then
    print("x is one")
elseif x == 2 then
    print("x is two")
else
    print("x is something else")
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

### while

Example:

```
x = 0
while x < 10
    print("X is " + x)
    x = x + 1
end
```

### for next

- step is optional
- the step must be a number, an expression is not allowed
- the for loop variable will only exist in scope of the loop

Example:

```
for x = 1 to 10
   print(x)
next

for x = 1 to 10 step 2
    print(x)
next

for x = 10 to 0 step - 1
    print(x)
next
```

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

## Methods

A method is called using the "dot" syntax.

Example:

```
array.push(123)
```

Method are really just functions, but can mutate the variable you are calling the method on.

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

Arrays are acually vectors

Example:

```
x = array(4,5,6) ' create an array with 3 elements

print(x[0]) ' prints 4
print(x[1]) ' prints 5
print(x[1]) ' prints 6

x[0] = 10 ' change first element from 4 to 10

x = push(x,15) ' adds an element to the end of the array
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

## String Interpolation

Very Basic supports string interpolation. Any expression between '{' and '}' will be interpreted and inserted into to the string.  
See example below:

```
print ("The result is {1+1}")
```

String interpolation is just syntatic sugar. The above string in converted to the following:

```
"The result is " + str(1+1) + ""
```

## Built-in functions

### _chr(ascii_value)_

Returns a one character string using ascii encoding. Invalid ascii value will return an empty string.

### _command()_

returns command line arguments as an array.

NOTE: The first argument will be the full path of vbas.exe, 2nd argument name of the script.

### _dir(pattern)_

will query the filesystem for all files that match a particular pattern. Uses Unix shell style patterns.

### _floor(number)_

Returns the largest integer not greater than _number_

### _input([prompt=""])_

reads input from the console

### _print(string, [newline=true], [colour=""])_

Prints a string to the console.

### _rand()_

returns a random number between 0 and 1

### _seconds()_

returns the number of seconds since the unix epoch

### _setting_get(key)_

reads a value from the settings. value returns will always be converted to a string.
Will return an empty string if the setting doesn't exist.

### _setting_set(key, value)_

saves a key/pair value to a config file. config file default name is [name of script].json.

### _str(value, [format_string])_

Converts any value to a string, optionally applying formatting to numbers.

- Nx format to x decimal places, use thousands separator e.g. `str(123456.456,"N2") => 123,456.46`
- Fx format to x decimal places, don't use thousands separator e.g. `str(123456.456,"F2") => 123456.46`

### _sqrt(num)_

returns the square root of _num_

### _val(string)_

Converts a string to a number. Will return zero if string cannot be converted to a number, or if the data type is not a string.

## File IO functions

### _append(filename, text)_

appends text to a file. If the file doesn't exist it will be created.

### _readlines(filename)_

returns an array of all lines in a text file

### _write(filename, text)_

creates a new file writes text to it. Will overwrite any existing file.

## Array functions

### _array([element],...)_

creates a new array, optionally populating with elements

### _sort(array)_

sorts an array

### _push(array, val)_

returns a new array with _val_ added to the end of an array

## Array methods

### _push(val)_

adds a value to the end of an array

### _slice(start, end)_

return a new sliced array. This does not mutate the array.

## String functions

### _instr()_

### _lcase()_

### _left(string, length)_

Returns a substring containing a specified number of characters from the beginning (left side) of a string.

### _mid(string, start, [length])_

returns part of a string using a 1 based index.
e.g mid("hello",3) returns "llo", mid("hello",3, 2) return "lo"

### _right()_

### _ucase()_

## Date and Time functions

Dates are stored as strings in ISO8601 format. There is no native datetime format. Functions that use dates will convert the string value to a date internally, and then convert the date back to a IOS8601 format string for any dates returned.

### _now()_

returns the current date using the local timezone

## Graphic functions

Very Basic has the ability to do basic 2D graphics. You can draw to a canvas and then display the canvas in a window, or save it as a image file.

### _cleargraphics()_

sets all pixels on the canvas to white

### _initgraphics(width, height)_

creates a new canvas for drawing. You must call this before calling any other graphics functions.

### _plot(x, y, colour)_

sets the colour at x,y coordinates on the canvas. Colour can either be a pre-defined colour string or a hex code (e.g. #ADFF2F)

### _rgb(red,green,blue)_

returns a hex code for the specified red, green and blue values.

### _window()_

display a window showing the canvas. Size of window will be the same as the canvas

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

Any function call prefixed with a '@' will be a system call.

e.g. `@ls("-l)` will be the bash command "ls -l".

on windows, to run the 'dir' command, use `@cmd("/c","dir")`

`@notepad()` will open notepad
