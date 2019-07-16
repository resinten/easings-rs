# easings-rs

This is a very simple implementation of the the easings found at https://easings.net/en.
The intent is to provide easy functionality without a ton of bells and whistles.

Install by placing this in the dependencies of your `Cargo.toml`:

```
easings-rs = "0.1.0"
```

There is a single enum with a variant for each easing effect documented on the above site,
along with a `Linear` easing as well. They are provided as `<category><in/out/inout>`, such as
`ElasticOut` or `SineInOut`. There is one exported function called `ease` which can called to
get the easing value for a percentage progress (`f32` between 0 and 1).

So, let's say you are using this in a game engine like Amethyst, and you are easing between
two positions for an object:

```
#[derive(Component)]
pub struct EaseEffect {
    pub duration_frames: u64,
    pub start_frame: u64,
    pub easing: Easing,
}

// ...

let frames_elapsed = time.frame_number() - ease_effect.start_frame;
let percent = frames_elapsed as f32 / ease_effect.duration_frames as f32;
let progress = ease(ease_effect.easing, percent);
*transform.translation_mut() = source + progress * (destination - source);

```

```

```
