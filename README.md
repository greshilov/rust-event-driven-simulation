## About

Event-driven programming is a widely spread technique used in many areas, from web servers to physical simulations.  
This project aims to implement a relatively simple 2D particle collision system. That's it, we have some closed domain, several obstacles, and a bunch of moving particles. Particles can have different radiuses and masses. Obstacles and domain consist of straight-line segments. All collisions considered being **perfectly inelastic**.  

The project contains following components:  
* red-simulation
* red-demonstration
* red-server

## red-simulation
Rust library with all this fancy `event-driven` logic and physical calculations inside. 
Built using `wasm-pack` tools into `.wasm` module. All core functionality can be found there.

## red-demonstration
Vue3 application with `red-simulation` wasm library as a dependency. Created with demonstration purposes.

## red-server
A small web server that's written on rust using `rocket` and `diesel`. Created to store game results from Vue3 application.  


The project is heavily inspired by the task from Princeton algorithm course (II part). Check it out [here](https://algs4.cs.princeton.edu/61event/). It contains a lot of useful information and good references to scientific papers about this topic.

## How to build
See detailed instructions in child directories.  

## Usage
Build app using previous section.
```
cd red-demonstration && npm run serve
cd red-server && cargo run
```
Voila :) Check out http://localhost:8080
