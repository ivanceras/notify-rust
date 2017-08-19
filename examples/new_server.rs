
extern crate dbus;

use std::sync::Arc;
use dbus::{Connection, BusType, NameFlag};
use dbus::tree::Factory;

fn main() {
    // Let's start by starting up a connection to the session bus and register a name.
    let c = Connection::get_private(BusType::Session).unwrap();
    c.register_name("com.example.dbustest", NameFlag::ReplaceExisting as u32).unwrap();

    // The choice of factory tells us what type of tree we want,
    // and if we want any extra data inside. We pick the simplest variant.
    let f = Factory::new_fn::<()>();

    // We create the signal first, since we'll need it in both inside the method callback
    // and when creating the tree.
    let signal = Arc::new(f.signal("HelloHappened", ()).sarg::<&str,_>("sender"));
    let signal2 = signal.clone();

    // We create a tree with one object path inside and make that path introspectable.
    let tree = f.tree(()).add(f.object_path("/org/freedesktop/Notifications", ()).introspectable().add(

        // We add an interface to the object path...
        f.interface("org.freedesktop.Notifications", ()).add_m(

            // ...and a method inside the interface.
            f.method("Notify", (), move |m| {

                // This is the callback that will be called when another peer on the bus calls our method.
                // the callback receives "MethodInfo" struct and can return either an error, or a list of
                // messages to send back.

                let sender = m.msg.sender().unwrap();
                let s = format!("Hello {}!", sender);
                let mret = m.msg.method_return().append1(s);

                let sig = signal.msg(m.path.get_name(), m.iface.get_name())
                    .append1(&*sender);

                // Two messages will be returned - one is the method return (and should always be there),
                // and in our case we also have a signal we want to send at the same time.
                Ok(vec!(mret, sig))

                    // Our method has one output argument, no input arguments.
            })
        .inarg::<&str,_>("app_name")
        .inarg::<u32,_>("replaces_id")
        .inarg::<&str,_>("app_icon")
        .inarg::<&str,_>("summary")
        .inarg::<&str,_>("body")
        .inarg::<&[&str],_>("actions")
        //.inarg::<,_>("hints"   "a{sv}"),
        .inarg::<i32,_>("timeout")
            .outarg::<&str,_>("reply")

        // We also add the signal to the interface. This is mainly for introspection.
        ).add_s(signal2)
    ));

    // We register all object paths in the tree.
    tree.set_registered(&c, true).unwrap();

    // ...and serve other peers forever.
    c.iter(1000).with(tree).count();
}
