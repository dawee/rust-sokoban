enum CharacterState {
    WalkingUp,
    WalkingRight,
    WalkingDown,
    WalkingLeft
}

struct Character {
    transform: Transform,
    sprite: Sprite,
    state: CharacterState
}

impl Chunk for Character {

    fn on_key_pressed(&mut self, key: &Key) {
        (self.state, self.sprite) = match key {
            Key::Up => match self.state {
                WalkingUp => (self.state, self.sprite),
                _ => (WalkingUp, Sprite::new("walk_up"))
            },
            Key::Right => match self.state {
                WalkingRight => (self.state, self.sprite),
                _ => (WalkingRight, Sprite::new("walk_right"))
            },
            /* and so on... */
        };
    }

    fn update(&mut self, dt: f64) {
        self.transform = match self.state {
            WalkingUp => self.transform.translate_y(-10.0 * dt),
            /* and so on */
            _ => self.transform
        };

        self.sprite.update(dt);
    }

    fn render(&self, scene: &mut Scene, origin: &Transform) {
        let transform = origin.apply(self.transform);

        scene.render(&self.sprite, &transform);
    }

}
