// Author: alejandro0619
// github: github.com/alejandro0619/
// contact me: spaghetticodedev@gmail.com

use cursive::traits::*;
use cursive::views::{
    Button, 
    Dialog, 
    DummyView, 
    EditView, 
    LinearLayout, 
    SelectView, 
};
use cursive::{Cursive, CursiveExt};
use regex;
use ytdlrs_lib::api::{
    client::APIClientBuilder, 
    downloader::DownloaderBuilder, 
    search::SearchVideo,
};
mod backend;
// CLI
#[tokio::main]
async fn main() {
    let mut siv = Cursive::new();

    let menu = LinearLayout::vertical()
        .child(Button::new("Search by query", search_query))
        .child(Button::new("Search by url", search_url))
        .child(DummyView)
        .child(Button::new("Quit app", Cursive::quit));

    // Now we render from here:
    siv.add_layer(
        Dialog::new()
            .title("Main menu 🦀")
            .content(
                LinearLayout::horizontal()
                .child(menu)
            )
    );

    // Press 'q' to close the app.
    siv.add_global_callback('q', |s| s.quit());
    // We now run the app down here.
    siv.run();
}

fn search_url(c: &mut Cursive) {
    // We do delete the current layer to render the new one.
    c.pop_layer();
    c.add_layer(
        Dialog::new()
            .title("Search by url")
            .content(
                EditView::new()
                    .on_submit(search)
                    .with_name("url")
                    .fixed_width(45)                    
            )
            .button("Ok", |s| {
                let name = s
                    .call_on_name("url", |view: &mut EditView| {
                        view.get_content()
                    })
                    .unwrap();
                search(s, &name);
            }),
    );
}

async fn do_something(s: &mut Cursive, format: &str) {

}
fn search(c: &mut Cursive, format: &str){
    c.pop_layer();

    // ask if we wants mp3 or mp4
    c.add_layer(
        Dialog::new()
            .title("Select format")
            .content(
                SelectView::new()
                    .popup()
                    .item("mp3", "mp3")
                    .item("mp4", "mp4")
                    .with_name("format")
            )
            .button("Ok", |s| {
                let format = s
                    .call_on_name("format", |view: &mut SelectView| {
                        view.selection()
                    })
                    .unwrap();
            }
        ),
    );
    
}
fn search_query(_c: &mut Cursive) {
    
}

