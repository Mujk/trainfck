# trainfck
work in process ⚡ not usable yet ⚡  

[esolangs.org wiki page](https://esolangs.org/wiki/Trainfck)

## language
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
### docs
File extension: .trainf \
Trainfck is an esoteric programming inspired by brainfuck and railways. Trains depart from stations (+) and travel in all directions. There is no limitation on stations and therefore also not on trains.
```
         |  
         ⬆️  
         🚆  
         |  
    -⬅️🚆-+-🚆➡️-  
         |  
         🚆  
         ⬇️  
         |  

```
When two or more trains collide, they crash. The program ends when all trains have crashed. Memory is managed in byte cells, as in brainfuck. All trains use the same byte cells. Because trainfck uses all the brainfuck operators, the language is turning complete. \
\
cells:
``` 
------------
|11|0|17|3|4|
------------
      ⬆️
```
The pointer points on the 3 cell. This byte can be increased or decreased now. The cell pointer and the cell value is controlled by the direction where a train passes a station, all other operators are on the rails. \
There is no nil in trainfck. Empty cells have always the value 0.

### operators
- “+” station
- "⬆️" cell byte + 1
- "⬇️" cell byte - 1
- "➡️" cell pointer goes to next cell
- "⬅️" cell pointer goes to last cell
- “-” “|” rails, the train can only move on rails and operators, or they will turn around
- “^” “v” “>” “<” changes direction at the next possible option
- “!” ignores the next action if the current cells equals 0 
- “.” prints out the current cell
- “,” takes input as value for the current cell
- "o" change direction