# Finite logic
This is a total esolang (meaning it always terminates) which uses just 4 commands.
## Register
The interpreter for this language has a singular register which can store one bit of information
## Carousel
The interpreter uses a carousel as "mass storage". Every computation done will expand the carousel with the new single bit value. Internally this is just a queue of bools.
## Initial state
When starting up, the language has false and true written to written to the carousel at the back and front respectively. A user may also define an n byte input, which will follow the constants.
Initially, the register will be set to false.
## Commands
1. **r**oll: rotates the carousel, basically writing the back of the queue to the front
2. **l**oad: peeks the back of the queue/carousel and writes it into the register
3. **n**and: peeks the back of the queue/carousel and nands it with the register. It writes the result to the front of the queue and rolls once
4. **p**rint, this "prints" the bit currently in the register (It actually writes it to a byte and prints that as a char once 8 bits have been printed)
5. Anything thats not either r, l, n or p, will be ignored by the interpreter
## Programs
### Hello World
```
p lp rlp p rlp rlp p p //H ENDING ON R=0, 
p rlp p rlp p rlp rlp rlp //E ENDING ON R=1 
rlp rlp p rlp rlp p rlp p //L ENDING ON R=0 
lp rlp p rlp rlp p rlp p //L ENDING ON R=0 
p rlp p rlp rlp p p p //O ENDING ON R=1
rlp p rlp rlp p p p p //SP ENDING ON R=0
p rlp rlp rlp rlp rlp p p //W ENDING R=1
rlp rlp p rlp rlp p p p //O ENDING ON R=1
rlp rlp p p rlp p rlp rlp //R ENDING R=0
lp rlp p rlp rlp p rlp p //L ENDING ON R=0 
p rlp p rlp p rlp rlp p //D ENDING ON R=0
p p rlp rlp p p p rlp //!
```
