use crate::gpu;

pub(crate) enum RunEvent {
    Tick,
}

pub enum Event {
    /// Leimu has been initialized.
    ///
    /// Gets called once at the beginning before any other events.
    Initialized,
    /// Leimu is updating.
    ///
    /// Happens once per frame before any GPU work.
    Update,
    /// A [`gpu::Gpu`] event has happened.
    GpuEvent(gpu::Event),
    /// Leimu is cleaning up.
    ///
    /// This is the last event that gets called.
    CleanUp,
}
