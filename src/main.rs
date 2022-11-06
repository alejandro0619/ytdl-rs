// Author: alejandro0619
// github: github.com/alejandro0619/
// contact me: spaghetticodedev@gmail.com
use backend::search_by_url::search;
use cursive::view::{Nameable, Resizable};
use cursive::views::{Button, Dialog, DummyView, EditView, LinearLayout, SelectView, TextView};
use cursive::{Cursive, CursiveExt};
mod backend;
// CLI
fn main() {
    let mut siv = Cursive::new();

    let menu = LinearLayout::vertical()
        .child(Button::new("Search by query", search_query))
        .child(Button::new("Search by url", |c: &mut Cursive| {
            let enter_url = Dialog::new()
                .title("Enter the url:")
                // we set the EditView witha fixed with of 40 spaces to
                // give enough room to paste a youtube link.
                .content(EditView::new().with_name("url").fixed_width(40))
                // Now we send the video url.
                .button("Send", |s| {
                    //s.pop_layer();
                    // We get the url
                    let url = s.call_on_name("url", |v: &mut EditView| v.get_content());
                    // We add a new layer to render a dialog that asks for the format of the video.
                    s.add_layer(
                        Dialog::new()
                            .title("Select a format")
                            .content(TextView::new("Select a format to download."))
                            .button("MP4", move |cs| {
                                search(cs, 1, url.as_ref().unwrap().to_string()).unwrap()
                            }),
                    )
                });
            c.add_layer(enter_url);
            //search_by_url::search()
        }))
        .child(DummyView)
        .child(Button::new("Quit app", Cursive::quit));

    // Now we render from here:
    siv.add_layer(
        Dialog::new()
            .title("Main menu 🦀")
            .content(LinearLayout::horizontal().child(menu)),
    );

    // Press 'q' to close the app.
    siv.add_global_callback('q', |s| s.quit());
    // We now run the app down here.
    siv.run();
}

fn search_query(_c: &mut Cursive) {}
