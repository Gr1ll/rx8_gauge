#[cfg(feature = "simulator")]
mod sim_display;

#[cfg(feature = "pi")]
mod fb_display;

#[cfg(feature = "simulator")]
pub use sim_display::SimDisplay as Display;

#[cfg(feature = "pi")]
pub use fb_display::FbDisplay as Display;

pub fn init_display() -> Display {
    #[cfg(feature = "simulator")]
    {
        Display::new()
    }

    #[cfg(feature = "pi")]
    {
        Display::new().expect("Failed to open framebuffer")
    }
}
