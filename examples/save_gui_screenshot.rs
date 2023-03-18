//use flameshot::params::FullscreensArgs;
use flameshot::params::GuiArgs;
//use flameshot::params::ScreenArgs;

pub fn main() {
    /// we select one of the 3 main param types and use the builder method
    /// to customize the cli arguments, it includes all flameshot cli parameters.
    let params = GuiArgs::builder()
        .path("screenshot.png")
        .accept_on_select()
        //.region((100, 100, 100, 100)) // just to show how one could use region param
        .build();
    /// This operation can error, it can error because of Os problems
    /// or flameshot problems.
    let output = flameshot::execute(params).unwrap();
}
