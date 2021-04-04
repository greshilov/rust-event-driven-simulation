## About

Event-driven programming is a widely spread technique used in many areas, from web servers to physical simulations.  
This project aims to implement a relatively simple 2D particle collision system. That's it, you have some closed domain, several obstacles, and a bunch of particles. Particles can have different radiuses and masses. Obstacles and domain consist of straight-line segments. All collisions considered being **perfectly inelastic**.  

The project is heavily inspired by the task from Princeton algorithm course (II part). Check it out [here](https://algs4.cs.princeton.edu/61event/). It contains a lot of useful information and good references to scientific papers about this topic.

## How to build

1. Install [wasm pack](https://rustwasm.github.io/wasm-pack/installer/).
2. Navigate to rust-event-driven-simulation subdir and run following command.
```
wasm-pack build
```

3. Go to rust-event-driven-demonstration subdir and install required dependencies.
```
npm install
```

## Usage
Build app using previous section.
```
cd rust-event-driven-demonstration && npm run serve
```
Voila :) Check out http://localhost:8080
