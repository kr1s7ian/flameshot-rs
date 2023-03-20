Easily integrate flameshot into your application by leveraging it's simple cli api now easily integrated in rust with this crate.

This example is using the 'image' crate feature.

```rust
//use flameshot::FullArgs;
use flameshot::GuiArgs;
//use flameshot::ScreenArgs;

//////////////////////////////////////////////////////////////////////
// If we enable "image" library Feature, we will be able to convert//
// a FlameshotOutput into a DynamicImage form rust Image crate.   //
///////////////////////////////////////////////////////////////////

pub fn main() {
    /// We select one of the 3 main param types and use the builder method to
    /// customize the cli arguments, it includes all flameshot cli args.
    let params = GuiArgs::builder()
        .accept_on_select()
        .raw() // this parameter is REQUIRED for converting to DynamicImage
        .build();
    /// This operation can error, it can error because of OS problems
    /// or Flameshot problems.
    let output = flameshot::execute(params).unwrap();

    /// This operation cause an Image error
    let img = output.to_dynamic_image().unwrap();

    let convert_black_and_white = img.to_luma8().save("black_and_white.png").unwrap();
}


```

Make sure to install flameshot and add it to your env path. Now you are ready to use this crate to grab screenshots using flameshot while being able to optionally get a dynamic_image of the screenshot for manipulation. If you don't want to use dynamic_image or image crate at all you can find the raw image in the stdout field of FlameshotOutput.output

Check out other [examples](/examples).
