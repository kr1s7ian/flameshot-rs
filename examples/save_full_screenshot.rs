use flameshot::params::FullArgs;
//use flameshot::params::GuiArgs;
//use flameshot::params::ScreenArgs;

pub fn main() {
    /// We select one of the 3 main param types and use the builder method to
    /// customize the cli arguments, it includes all flameshot cli args.
    let params = FullArgs::builder().path("screenshot.png").build();
    /// This operation can error, it can error because of OS problems
    /// or Flameshot problems.
    let output = flameshot::execute(params).unwrap();
}
