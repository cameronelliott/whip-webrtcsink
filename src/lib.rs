use gst::{glib, prelude::StaticType};
use webrtcsink::webrtcsink::WebRTCSink;





mod signaller;
mod rgb2gray;
//pub mod webrtcsink;

//use crate::signaller::mod::Signaller;

fn plugin_init(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {
    //register(plugin)?;

    rgb2gray::register(plugin)?;

    Ok(())
}


pub fn register(plugin: &gst::Plugin) -> Result<(), glib::BoolError> {

    
    // let custom_signaller = Signaller::default();
    
    // let webrtcsink = webrtcsink::webrtcsink::WebRTCSink::with_signaller(Box::new(custom_signaller));

    gst::Element::register(
        Some(plugin),
        "xwebrtcsink",
        gst::Rank::None,
        WebRTCSink::static_type(),
    )
}


gst::plugin_define!(
    xwebrtcsink,
    env!("CARGO_PKG_DESCRIPTION"),
    plugin_init,
    concat!(env!("CARGO_PKG_VERSION"), "-", env!("COMMIT_ID")),
    "MIT",
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_NAME"),
    env!("CARGO_PKG_REPOSITORY"),
    env!("BUILD_REL_DATE")
);



