pub mod gtk_fe {
    extern crate gtk;

    use self::gtk::prelude::*;

    use self::gtk::{Window, TreeView, ListStore,Stack,StackSidebar};
    use xsetwacom::*;
    use xsetwacom::getting::get_devices;
    use xsetwacom::getting::get_sets_from_id;


    pub fn init() {
        if gtk::init().is_err() {
            println!("Failed to init the gtk");
        }

        let glade_src = include_str!("fe.glade");
        let builder = gtk::Builder::new_from_string(glade_src);

        let window: gtk::Window = builder.get_object("window").unwrap();
        let sidebar: gtk::StackSidebar = builder.get_object("sidebar").unwrap();
        let stack: gtk::Stack = builder.get_object("stack").unwrap();

        let list_devices: Vec<Device> = get_devices();

        let mut trees: Vec<TreeView> = Vec::new();
        for device in list_devices{
            let mut tree = create_and_setup_view();
            let liststore = create_and_fill_model(get_sets_from_id(device.id));
            tree.set_model(&liststore);
            trees.push(tree);
        }


        for tree in trees{
            stack.add(&tree);
        }


        window.show_all();

        window.connect_delete_event(|_, _| {
            gtk::main_quit();
            Inhibit(false)
        });

        gtk::main();
    }

    fn create_and_fill_model(sets: Vec<Set>) -> gtk::ListStore {
        // Creation of a model with two rows.
        let model = gtk::ListStore::new(&[String::static_type(), String::static_type()]);
        // Filling up the tree view.
        for set in sets {
            if set.id != 0 {
                model.insert_with_values(None, &[0, 1], &[&format!("{} {}", set.typ, set.id)
                    .to_owned(), &set.mapped_to]);
            } else {
                model.insert_with_values(None, &[0, 1], &[&set.typ, &set.mapped_to]);
            }
        }
        return model;
    }

    fn append_column(tree: &TreeView, id: i32) {
        let column = gtk::TreeViewColumn::new();

        let cell = gtk::CellRendererText::new();

        column.pack_start(&cell, true);
        // Association of the view's column with the model's `id` column.
        column.add_attribute(&cell, "text", id);
        tree.append_column(&column);
    }

    fn create_and_setup_view() -> TreeView {
        // Creating the tree view.
        let tree = gtk::TreeView::new();

        tree.set_headers_visible(false);
        // Creating the two columns inside the view.
        append_column(&tree, 0);
        append_column(&tree, 1);
        return tree
    }
}