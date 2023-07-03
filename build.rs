use {
    std::{
        io,
    },
    winres::WindowsResource,
};

fn main() -> io::Result<()> {
    if cfg!(target_os = "windows") {
        WindowsResource::new()
            .set_icon("icon.ico")
            .compile()?;
    }
    Ok(())
}