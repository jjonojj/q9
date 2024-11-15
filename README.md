# q9
yet another programming language.
very lightweight (lightweight != fast), no dependencies atm.

## examples
currently, the language has no standard library, so functionality is **very very** limited.
it is dynamically typed, the only real type that exists at the moment is a number (a 64-bit floating point value).
you can still create functions, pass arguments to functions, and return from them.
```
proc get(x) {
  return x
}

get(4) 
```

the above get statement would return 4.
```
let x = 2

function setx() {
  x = 3
}

setx()
```
the above program would result in x being 3.
since the language is in very early development, 
there is no math expression parser at the moment so you can just return either a number, 
an identifier or the return value of a function.

here are some random docs, in no particular order. 
i just wrote them down whenever i implemented them (some of them are actually not implemented yet, i just noted them so i know what to add in the future):

