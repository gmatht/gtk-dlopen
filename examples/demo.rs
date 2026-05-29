use gtk_compat::{Loader, Window, BoxWidget, Label, Button, Orientation, Application};
use std::rc::Rc;
use std::cell::RefCell;

fn main() -> Result<(), Box<dyn std::error::Error>> {
    let loader = Loader::new()?;
    println!("Loaded GTK version: {:?}", loader.version());

    // Use Application::new if desired; our simplified Application uses GMainLoop run
    let app = Application::new(loader.clone(), Some("org.example.GtkCompatDemo"))?;

    // Create UI immediately (no activate signal wiring for simplicity)
    let win = Window::new(loader.clone())?;
    win.set_title("Rust gtk-compat demo");

    let hbox = BoxWidget::new(loader.clone(), Orientation::Horizontal, 6)?;
    let label = Label::new(loader.clone(), "Count: 0")?;
    let button = Button::with_label(loader.clone(), "Click me")?;

    let counter = Rc::new(RefCell::new(0));
    let c2 = counter.clone();
    let label2 = label.clone();
    // connect clicked
    button.connect_clicked(move || {
        *c2.borrow_mut() += 1;
        let v = *c2.borrow();
        label2.set_text(&format!("Count: {}", v));
        println!("closure invoked, updated label to Count: {}", v);
    })?;

    // emit clicked once programmatically to test that signal wiring works
    button.emit_clicked()?;
    if let Some(txt) = label.get_text() {
        println!("label text after emit: {}", txt);
    } else { println!("label get_text not available"); }

    hbox.append(&label);
    hbox.append(&button);
    win.set_child(&hbox);
    win.present();

    app.run()?;
    Ok(())
}
