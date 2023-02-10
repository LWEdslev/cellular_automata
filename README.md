# cellular_automata
A framework for making 2d cellular automata. 
Standard implementation is using Conway's Game of Life with special graphics.

Run with 

`cargo run`
for default configuration

or

`cargo run fps=60 grid=200 window=600`
for user specified configuration where

- FPS changes the iteration speed of the cells.
- Grid describes how many cells per side of the grid (if grid=n there will be n² cells)
- Window describes the length and width of the square window in pixels
