# Very Basic

A very stripped back BASIC like language. It contains only what you need to create a turing complete program. No synactic sugar, nothing unnessary. You should be able to learn it in less than 5 minutes.

All variables are stored on the stack. There is no heap or garbage collection. Memory is freed when variables go out of scope. All variables are passed by val to functions, and assignment means the variable will be copied. Very Basic doesn't ever use references.

Example:

```
a = array(1,2,3)

' b is copy of 'a'
b = a

b[0] = 5

print a[0]  ' prints 1
print b[0]  ' prints 5
```

Other things to note:

- Very Basic is not object orientated, there are no classes, inheritance or interfaces
- Very Basic does not have closures.  You cannot declare a function within a function.
- Functions are first class citizens and can be passed as parameters to functions (via the funtion name).
- Very Basic is not designed for speed or memory efficiency
- Very Basic is a dynamically typed language, like Python or Javascript

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

The function keyword can be shortened to `fn`

```
fn foo(x)
    if x < 0 then
        0 exit
    end if
    x
end
```

## Methods and "dot" calling.

A method is called using the "dot" syntax.

Example:

```
array.push(123)
```

Method are really just functions, but can mutate the variable you are calling the method on. Any function can be called using dot syntax. `"1,2,3".split(",")` is the same as `split("1,2,3",",")`

## Pattern Matching

match [expression]
when [operator] [Expression] then [statements]
when [Expression[,Expression,...]] then [statements]
when [Expression] to [Expression] then [statements]
...
else [expression]
end

Example:

```
x = match i
    when 0 then  1      ' match single value
    when 2 to 3 then  2 ' match range
    when 4,5,6 then 7   ' match multiples
    when <=6 then  5    ' match using operators
    when < 8 then  6
    when >=5 then  4
    when > 3 then  3
    else  7             ' must supply else
end
```

## Data types

There are 4 datatypes. String, Number, Boolean and Array.
Arrays can hold any datatype

```
x = "hello"  ' string
x = 123  ' number floating point
x = true ' boolean
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

x.push(15) ' adds an element to the end of the array
```

## Operators

Standard operators:

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

### Addional operators

_in [expression],[expresssion],..._

Example:

```
if x in 1,2,3 then print("x is 1 2 or 3");
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

## Short-cuts

The following short-cuts are available:

- `function` can be replace with `fn`
- `end` can be replaced with `;`

```
fn dbl_num(x) x*2;

if dbl_num(2)==4 then print("doubled");
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

### _clear()_

clears the console

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

### _round(num, precision)_

round a number to [precision] decimal places

### _seconds()_

returns the number of seconds since the unix epoch

### _setting_get(key)_

reads a value from the settings. value returns will always be converted to a string.
Will return an empty string if the setting doesn't exist.

### _setting_set(key, value)_

saves a key/pair value to a config file. config file default name is [name of script].json.

### _sleep(milliseconds)_

pause the program for a specified number of milliseconds

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

### _dim(size, [value])_

creates a new array of a specified size, all elements will default to zero, or optionally to the specified value

### _find(array, item)_

return the index of _item_ in the array. if _item_ is not found it returns -1

### _max(array)_

returns the largest element in the array

### _push(array, val)_

returns a new array with _val_ added to the end of an array

## _shuffle(array)_

return a new array with all elements ordered at random

### _sort(array)_

returns a sorted array

## Array methods

### _push(val)_

adds a value to the end of an array

### _slice(start, end)_

return a new sliced array. This does not mutate the array.

### _filter(operator, value, [[operator], [value]...])_

Example:

```
array.filter(">", 7) ' all elements greater than 7
array.filter("<", 5, ">", 10) ' all elements less than 5 or greater than 10
```

## String functions

## _asc(string)_

get ascii value of the first character of a string. Will return 0 if string is empty or parameter is not a string

### _instr(string1, string2, [start],[compare])_

returns the index of string2 found in string1, using a 1 based index. If not found, then it returns zero.

- start = start position to search
- compare = if this value is 1, then it does a case insenstive comparison

### _lcase(string)_

returns the lower case value of a string

### _left(string, length)_

Returns a substring containing a specified number of characters from the beginning (left side) of a string.

### _mid(string, start, [length])_

returns part of a string using a 1 based index.
e.g mid("hello",3) returns "llo", mid("hello",3, 2) return "lo"

### _right(length)_

returns the right most characters of a string

### _replace(string, search, replace)_

replace part of a string

### _split(string, delimiter, [remove_empty = false])_

splits a string based on a delimiter and returns an array of its part. If remove_empty is true then any empty elements are removed from the array.

### _ucase(string)_

returns the upper case value of a string

## Date and Time functions

Dates are stored as strings in ISO8601 format. There is no native datetime format. Functions that use dates will convert the string value to a date internally, and then convert the date back to a IOS8601 format string for any dates returned.

### _now()_

returns the current date using the local timezone

### _dateadd(date, interval, number)_

valid intervals are:

- s = second
- n = minute
- h = hour
- d = day
- w = week
- m = month
- y = year

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
