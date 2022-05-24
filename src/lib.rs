// We do this since rust complains about everything being FFI unsafe, yet it doesn't really matter
// since we are USUALLY having rust shared libs.
#![allow(improper_ctypes_definitions)]

use std::sync::{Arc, Mutex};
use dlauncher::extension::{ExtensionContext, ExtensionExitCode};
use dlauncher::extension::response::{ExtensionResponse, ExtensionResponseIcon};
use dlauncher::util::{copy_to_clipboard, init_logger, matches};
use lazy_static::lazy_static;

// Used so we can run stuff in static
lazy_static! {
  // Here we have a PREFIX variable which is a String wrapped in a Arc and Mutex.
  // We need this to be thread safe and we need it mutable.
  // Usually in the on_init function is where this variable would be set.
  #[derive(Debug)] static ref PREFIX: Arc<Mutex<String>> = Arc::new(Mutex::new(String::new()));
}

// No mangle is required since when we access this function from dlauncher, we want the same function
// name to be used.
// This function is used at the startup of Dlauncher, and only runs once.
#[no_mangle]
pub unsafe extern "C" fn on_init(ctx: ExtensionContext) -> ExtensionExitCode {
  init_logger();

  // we can use the ExtensionContext if we want to access the extensions config
  // to get maybe a prefix, or other user changeable thing.
  // An example below, yet it wont be used for this extension since it isn't needed
  let prefix = ctx.config.get("prefix").unwrap_or("something".to_string());
  let mut p = PREFIX.lock().unwrap();
  *p = prefix;

  // Return Ok since no errors occurred during this.
  ExtensionExitCode::Ok
}

// This function is called whenever the input is changed in the ui.
#[no_mangle]
pub unsafe extern "C" fn on_input(ctx: ExtensionContext) -> ExtensionExitCode {
  // This call is necessary if we want to do anything relating to GTK/GDK.
  gtk::set_initialized();

  if let Some(input) = ctx.input {
    // This part is not used for the extension, it is a continuation of the prefix example that was
    // in the on_init function.
    // It will check if the inputs prefix is the same as the one we stored in the PREFIX variable.
    // Since we want this to "silently error out" we just return Ok instead of an Error.
    // We also check if the query is empty here.
    // Below:
    // let p = &*PREFIX.lock().unwrap();
    //
    // if input.prefix() != prefix || input.query().is_empty() {
    //   return ExtensionExitCode::Ok;
    // }
    // You can uncomment this part if you want a prefix. Something that could require a prefix is a
    // calculator extension for example.

    // Now back to the actual extension.
    // Lets check if the users input matches "zero width space"
    // We can use the match from this to highlight certain parts of the text in the UI.

    if let Some(match_) = matches(&input.all(), "Zero Width Space", 30) {
      // We create a ExtensionResponse var which we can mutate with lines that will be reflected in the
      // UI. We have to pass the extension's name into here for identification.
      let resp = ExtensionResponse::builder(&ctx.name)
        // Now we can add a line that takes a match and a function that is called when the user selects it
        .line_match_on_enter(
          "Zero Width Space", // This should usually be what you are matching in the matches function.
          "Press enter to copy to clipboard",
          // The icon represents a icon that is themed, so it will search for the spacer-symbolic icon in your icon theme.
          ExtensionResponseIcon::themed("spacer-symbolic"),
          match_,
          move |_| {
            // Finally we can copy the zero width space character to the clipboard!
            // Now when you type in "zero" you will see this entry in the UI, and once you press enter,
            // it will copy it.
            copy_to_clipboard("\u{200B}");
          }
        )
        .build(ctx.window.clone());

      // Now we need to append it to the results
      ctx.window.append_result(resp[0].clone());
    }
  }

  ExtensionExitCode::Ok
}
