//use flameshot::params::FullArgs;
use flameshot::params::GuiArgs;
//use flameshot::params::ScreenArgs;

pub fn main() {
    /// We select one of the 3 main param types and use the builder method to
    /// customize the cli arguments, it includes all flameshot cli args.
    let params = GuiArgs::builder()
        .path("screenshot.png")
        .accept_on_select()
        //.region((100, 100, 100, 100)) // just to show how one could use region param
        .build();
    /// This operation can error, it can error because of OS problems
    /// or Flameshot problems.
    let output = flameshot::execute(params).unwrap();
}
