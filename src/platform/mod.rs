#[cfg(feature = "pi")]
mod fb_display;
#[cfg(not(feature = "pi"))]
mod sim_display;

#[cfg(feature = "pi")]
pub use fb_display::FbDisplay as Display;
#[cfg(not(feature = "pi"))]
pub use sim_display::SimDisplay as Display;

pub fn init_display() -> Display {
    #[cfg(feature = "pi")]
    {
        Display::new().expect("Failed to open framebuffer")
    }

    #[cfg(not(feature = "pi"))]
    {
        Display::new()
    }
}
