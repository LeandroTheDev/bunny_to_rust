# Rust Project
Rust project is a experimental project, creating a 3D game using only rust language and fyrox engine.

Bunnyhop game, that you needs to jump to gain acceleration and win the level.

V0.3
- Moviment system > WASD

https://github.com/LeandroTheDev/rust_project/assets/106118473/89b79468-ccff-4c9c-ac16-56ad82340645

V0.5
- Jump system > Space
- Reset Scene > R
- Acceleration

https://github.com/LeandroTheDev/rust_project/assets/106118473/1dd31b4c-19a2-47cf-a868-1d6b955e79d9

V0.6
- Physics improvements
- Fixed some colliders bug
- Better reset scene
- Frontal colission
- Fixed the jump stop acceleration in rotated floor

https://github.com/LeandroTheDev/rust_project/assets/106118473/cbbe55a8-32e2-4d0b-9a17-f2a94fb21704

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
- cargo run --bin executer
> This will compile the project, will take a long time..., if you just testing just download the lastest [release](https://github.com/LeandroTheDev/rust_project/releases)

How to create a script?
- Open terminal
- cd to project
- fyrox-template script --name MyScript
