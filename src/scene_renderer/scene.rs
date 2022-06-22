use gtk::{
    cairo::{self},
    gdk_pixbuf::Pixbuf,
    prelude::GdkContextExt,
};
use lazy_static::lazy_static;
use std::sync::Mutex;

lazy_static! {
    pub static ref SCENE: Mutex<Scene> = Mutex::new(Scene::new());
}

pub struct Scene {
    hierarchy: Vec<Object>
}

impl Scene {
    pub fn new() -> Scene {
        Scene { hierarchy: vec![] }
    }

    pub fn update(&self, ctx: &cairo::Context, scene_size: (f64, f64)) {
        let drawer = Drawer::new(ctx, scene_size);

        for object in &self.hierarchy {
            drawer.draw(object);
        }
    }

    pub fn send_event(&self, click_pos : (f64, f64)) -> Option<&Object> {
        for item in self.hierarchy.iter() {
            if item.within_bounds(click_pos) {
                return Some(&item);
            }
        }
        None
    }

    pub fn instantiate(&mut self, obj: Object, parent: Option<&Object>) {
        if parent == None {
            self.hierarchy.push(obj);
        } else {
            let parent = parent.unwrap();
            for o in self.hierarchy.iter_mut() {
                if o == parent {
                    o.children.push(obj);
                    break;
                }
            }
        }
    }
}

struct Drawer<'a> {
    context: &'a cairo::Context,
    scene_size: (f64, f64),
}

impl Drawer<'_> {
    pub fn new(ctx: &cairo::Context, size: (f64, f64)) -> Drawer<'_> {
        Drawer {
            context: ctx,
            scene_size: size,
        }
    }

    pub fn draw(&self, obj: &Object) {
        if !obj.image.is_empty() {
            // TODO image scaling and positioning
            let image_surface = Pixbuf::from_file(obj.image.as_str()).unwrap();
            let target_size = self.scale_to_scene_size(obj.scale);
            let image_surface = image_surface
                .scale_simple(
                    target_size.0 as i32,
                    target_size.1 as i32,
                    gtk::gdk_pixbuf::InterpType::Bilinear,
                )
                .unwrap();

            let target_position = self.scale_to_scene_size(obj.position);
            self.context
                .set_source_pixbuf(&image_surface, target_position.0, target_position.1);
            self.context.paint().unwrap();
        }

        for child in &obj.children {
            self.draw(child);
        }
    }

    fn scale_to_scene_size(&self, unscaled: (f64, f64)) -> (f64, f64) {
        (
            unscaled.0 * self.scene_size.0,
            unscaled.1 * self.scene_size.1,
        )
    }
}

pub struct Object {
    pub id: i128,
    pub position: (f64, f64),
    pub rotation: f64,
    pub scale: (f64, f64),
    pub image: String,
    pub children: Vec<Object>,
}

impl Object {
    pub fn new() -> Object {
        unsafe {
            CURR_ID += 1 as i128;
            Object {
                id: CURR_ID,
                position: (0.0, 0.0),
                rotation: 0.0,
                scale: (1.0, 1.0),
                image: "".to_string(),
                children: vec![],
            }
        }
    }

    pub fn with_image(mut self, image_src: &str) -> Object {
        self.image = image_src.to_string();
        self
    }

    pub fn with_position(mut self, position: (f64, f64)) -> Object {
        self.position = position;
        self
    }

    pub fn with_scale(mut self, scale: (f64, f64)) -> Object {
        self.scale = scale;
        self
    }

    #[allow(unused)]
    pub fn with_rotation(mut self, rot: f64) -> Object {
        self.rotation = rot;
        self
    }

    pub fn within_bounds(&self, pos : (f64, f64)) -> bool {
        self.position.0 < pos.0 && self.position.0 + self.scale.0 > pos.0
            && self.position.1 < pos.1 && self.position.1 + self.scale.1 > pos.1
    }
}

impl PartialEq for Object {
    fn eq(&self, other: &Self) -> bool {
        self.id == other.id
    }
}

static mut CURR_ID: i128 = 0;
