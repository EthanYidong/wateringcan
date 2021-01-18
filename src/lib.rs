use seed::virtual_dom::{El, UpdateEl};

use std::collections::HashMap;

#[derive(Default)]
pub struct WateringCan {
    animations: HashMap<String, AnimationState>,
}

impl WateringCan {
    pub fn new() -> WateringCan {
        WateringCan::default()
    }

    pub fn create_animation(&mut self, name: impl ToString, animation: Animation) {
        self.animations.insert(name.to_string(), AnimationState {
            playing: true,
            elapsed: 0.0,
            animation,
        });
    }

    pub fn play_animation(&mut self, name: &str) {
        self.animations.get_mut(name).map(|anim| anim.playing = true);
    }

    pub fn stop_animation(&mut self, name: &str) {
        self.animations.get_mut(name).map(|anim| anim.playing = false);
    }

    pub fn tick(&mut self, elapsed: Option<f64>) {
        if let Some(elapsed) = elapsed {
            for animation in self.animations.values_mut() {
                if animation.playing {
                    animation.elapsed += elapsed;
                }
            }
        }
    }

    pub fn animation(&self, name: &str) -> Option<&AnimationState> {
        self.animations.get(name)
    }
}

pub struct AnimationState {
    playing: bool,
    elapsed: f64,
    animation: Animation,
}

impl<Ms> UpdateEl<Ms> for &AnimationState {
    fn update_el(self, el: &mut El<Ms>) {
        let animation = &self.animation;

        let progress = f64::min(1.0, self.elapsed / animation.duration);

        let current_value = progress * (animation.to - animation.from) + animation.from;
        el.add_style(animation.attribute.clone(), &format!("{}{}{}", animation.prefix, current_value, animation.suffix));
    }
}

pub struct Animation {
    pub attribute: String,
    pub duration: f64,
    pub from: f64,
    pub to: f64,
    pub prefix: String,
    pub suffix: String,
}
