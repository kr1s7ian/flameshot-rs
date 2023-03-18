use flameshot::params::FullscreensArgs;
//use flameshot::params::GuiArgs;
//use flameshot::params::ScreenArgs;

pub fn main() {
    /// we select one of the 3 main param types and use the builder method
    /// to customize the cli arguments, it includes all flameshot cli parameters.
    let params = FullscreensArgs::builder().path("screenshot.png").build();
    /// This operation can error, it can error because of Os problems
    /// or flameshot problems.
    let output = flameshot::execute(params).unwrap();
}
