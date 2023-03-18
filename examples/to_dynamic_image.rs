//use flameshot::params::FullscreensArgs;
use flameshot::params::GuiArgs;
//use flameshot::params::ScreenArgs;

//////////////////////////////////////////////////////////////////////
// If we enable "image" library Feature, we will be able to convert//
// a FlameshotOutput into a dynamic_image form rust Image crate.   //
///////////////////////////////////////////////////////////////////

pub fn main() {
    /// we select one of the 3 main param types and use the builder method
    /// to customize the cli arguments, it includes all flameshot cli parameters.
    let params = GuiArgs::builder()
        .accept_on_select()
        .raw() // this parameter is REQUIRED for converting to dynamic_image
        .build();
    /// This operation can error, it can error because of Os problems
    /// or flameshot problems.
    let output = flameshot::execute(params).unwrap();

    /// This operation cause an Image error
    let img = output.to_dynamic_image().unwrap();

    let convert_black_and_white = img.to_luma8().save("black_and_white.png").unwrap();
}
