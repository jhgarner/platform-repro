# The Goal

I'd like to have a platform which moves according to some formula relating time to position.
The platform has specific start and end points, so I'd like to use position directly instead
of going through Velocity.

I'd like another dynamic object to move with the platform when standing on it. The dynamic
object should use Velocity.

# My Attempted Solution

I tried to use the delta_time to calculate what the platform's velocity would be, and apply that
velocity to the dynamic object. Although this works well when the game is running quickly, it
causes the dynamic object to lag behind the platform when I introduce a large delay between when
I calculate the velocity and when the velocity is applied.

Here it is at a high speed:

[Screencast from 2024-05-02 16-43-55.webm](https://github.com/jhgarner/platform-repro/assets/7400280/f7e552f7-0502-4826-be87-206f9d29088c)

And here it is at a low speed:

[Screencast from 2024-05-02 16-43-23.webm](https://github.com/jhgarner/platform-repro/assets/7400280/22d33cb9-f08f-4fa9-8f14-ac523377f7ea)

What's the right way to make something like this work? The entire demo is <100 lines in the `src/main.rs` file.
