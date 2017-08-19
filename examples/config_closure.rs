extern crate notify_rust;
use notify_rust::Notification;

fn main() {

    Notification::build(|mut n| {
        n.summary("closures ftw");
        n
    }).show().unwrap(); 
}
