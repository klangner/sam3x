/// Abstract driver definition


pub trait Driver {
    /// Tick is called on every driver in each loop iteration
    /// Some drivers will ignore this. But some may need it for its functionality
    fn tick(&self);
}
