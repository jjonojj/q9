# q9
yet another programming language.
very lightweight, only dependency is regex.

here are some random docs, in no particular order. 
i just wrote them down whenever i implemented them (some of them are actually not implemented yet, i just noted them so i know what to add in the future):

## imports / uses **(not implemented yet)**
you can import other files like this:
```
use <std::math>

```
the preprocessor will search for the file in ./q9 (so for this example it would try and include ./q9/std/math.q9) 
**and** in the paths provided by the environment variable Q9_STD_PATH (for example, if ~/q9lib is in the path, it will
try and open the file ~/q9lib/std/math.q9). paths in Q9_STD_PATH are separated by a colon.

## function arguments (not implemented yet)

pretty self explanatory, pass arguments to functions.


