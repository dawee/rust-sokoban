pub trait Movable {
    fn move_up(&mut self) {}
    fn move_right(&mut self) {}
    fn move_down(&mut self) {}
    fn move_left(&mut self) {}
    fn set_position(&mut self, f64, f64) {}
}
