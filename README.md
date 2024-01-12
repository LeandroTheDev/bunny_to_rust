# Bunny to Rust
Bunny to Rust is a experimental project, creating a 3D game using only [rust](https://www.rust-lang.org/) language and [fyrox engine](https://fyrox.rs/).

The game style is the bunnyhop inspirated in counter strike bhops and surf maps, that means you needs to jump to gain acceleration, and slide or surf to get even more acceleration to pass the levels, is a very hard to learn for the first time, but after learning the basic mechanics you can go fast than light.

You can *``DOWNLOAD``* the latest stable version from the [releases](https://github.com/LeandroTheDev/rust_project/releases) section

``Actually Features``
- Automatic Jump by holding space
- Acceleration
- Slider to surf
- A simple scene to test
- Speedrun Timer

``Future Features``
- Guns/Enemies
- Wall slide
- Third Person
- Multiplayer / In long future maybe...

# FAQ

Controls
- WASD / Movement
- R / Reset scene
- Space / Jump, hold to auto jump

How to play?
- First you need to learn the basic moviments using the WASD you can moviment you character.
- Pressing the space you can jump.
- The bunnyhop system, you gain acceleration while in the air, and while moving your player from right to left, use this to gain velocity to go to others platforms in the scene.
- You can surf in sliders to gain more velocity.
- If you frontal collide with something you will lose all the acceleration.
- The objective is to get to the end of the map in short time.
- Getting high velocity maybe difficult to past some parts of the maps, so if you are not pressing the W you will slowly lose acceleration and not gain acceleration in air
- High velocity also gives you less acceleration
- Sliders can increase acceleration
- The timer will start after the first jump

How to compile the project?
- Download Rust
- Open terminal
- cargo install fyrox-template
- fyrox-template init --name bunny_to_rust --style 3d
- > Obs: project template always get from latest version and this project can be outdated for template, if this happens download in releases the executors for this version
- place all files to this new folder and overwrite if needed
- cargo run --package executor --release
> This will compile the project, will take a long time..., if you just testing just download the lastest [release](https://github.com/LeandroTheDev/rust_project/releases)

Can i modify this and reupload?
- yes.