# trainfck
[esolangs.org wiki page](https://esolangs.org/wiki/Trainfck)
## docs
### installation
```
git clone https://github.com/Mujk/trainfck.git
cd trainfck
cargo build 
```
### run your code
```
./trainfck my-code.trainf
```
### instructions
File extension: .trainf \
Trainfck is a minimalistic brainfuck inspired esoteric programming language with trains.\
Trains depart from stations (+) and travel in all directions. There is no limitation on stations and therefore also not on trains. \
The code is basically a minimalistic rail network.
```
         |  
         ā¬ļø  
         š  
         |  
    -ā¬ļøš-+-šā”ļø-  
         |  
         š  
         ā¬ļø  
         |  

```
When two or more trains collide, they crash. The program ends when all trains have crashed. Memory is managed in byte cells, as in brainfuck. All trains use the same byte cells. Because trainfck uses all the brainfuck operators, the language is turning complete. \
\
cells:
``` 
------------
|11|0|17|3|4|
------------
      ā¬ļø
```
The pointer points on the third cell. This byte can be increased or decreased now. The cell pointer and the cell value is controlled by the direction in which a train passes a station, all other operators are on the rails. \
This interpreter uses ASCII encoding. Everything is saved as ASCII number.\
Empty cells always have the value 0, which is null in ASCII.

### operators
- ā+ā station &#8594; directions: "ā¬ļø" cell byte +1, "ā¬ļø" cell byte -1, "ā”ļø" cell pointer +1, "ā¬ļø" cell pointer -1
- ā-ā ā|ā rails, the train can only move on rails and operators, or they will turn around
- ā^ā āvā ā>ā ā<ā changes direction at the next possible option
- ā?ā ignores the next action if the current cell equals the last cell 
- ā.ā prints out the current cell
- ā,ā takes input as value for the current cell
- "o" change direction