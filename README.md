# Rust Project
Rust project is a experimental project, creating a 3D game using only rust language and fyrox engine.

Bunnyhop game, that you needs to jump to gain acceleration and win the level.

# FAQ

Controls
- WASD / Movement
- R / Reset scene
- Space / Jump, hold to auto jump

How to play?
- First you need to learn the basic moviments using the WASD you can moviment you character.
- Pressing the space you can jump.
- The bunnyhop system, you gain acceleration while in the air, and while moving your player from right to left, use this to gain velocity to go to others platforms in the scene.
- You can find some parts in the scenario that you can gain some additional jump height.
- The rotated floor gives you a big boost in jump height but be careful, if you stand to much in you will lost acceleration.
- If you frontal collide with something you will lose the acceleration.

How to test the project?
- Download Rust
- Open terminal
- cargo install fyrox-template
- fyrox-template init --name bunny_to_rust --style 3d
- place all files to this new folder
- cargo run --package executor --release
> This will compile the project, will take a long time..., if you just testing just download the lastest [release](https://github.com/LeandroTheDev/rust_project/releases)

Can i modify this and reupload?
- yes.