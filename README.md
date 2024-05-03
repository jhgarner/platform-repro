# The Goal

I'd like to have a platform which moves according to some formula relating time to position.
The platform has specific start and end points.

I'd like another dynamic object to move with the platform when standing on it. The dynamic
object should use Velocity.

# My Attempted Solution

(See the main branch for a solution that tries to modify the position of the platform and the
velocity of the dynamic object)

The source code is short in src/main.rs.

First, I calculate the intended position using the formula and the elapsed time. Next, I
calculate the distance the platform would have to travel to reach that point. I divide that
by delta_time and set that as the platform's velocity. The dynamic object copies over the
platform's velocity. At low speeds it works perfectly, but at a high frame rate the objects
take off and escape the scene.

The incorrect results look like:

[Screencast from 2024-05-03 14-03-14.webm](https://github.com/jhgarner/platform-repro/assets/7400280/080f406d-41bf-4c41-a6b3-c10c52154cfe)

