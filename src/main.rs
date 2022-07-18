use std::{
    cell::RefCell,
    path::{Path, PathBuf},
    rc::Rc,
};

struct Controller {
    state: Rc<RefCell<State>>,
}
trait ControllerPrintable {
    fn print_path(&self);
}
impl Controller {
    fn new(state: &Rc<RefCell<State>>) -> Box<dyn ControllerPrintable> {
        Box::new(Controller {
            state: state.clone(),
        })
    }
}

impl ControllerPrintable for Controller {
    fn print_path(&self) {
        let path = &self.state.borrow().path;
        println!("{:?}", path);
    }
}

struct State {
    id: String,
    name: String,
    path: PathBuf,
    description: Option<String>,
}

struct Servo {
    state: Rc<RefCell<State>>,
    something: String,
    controller: Option<Box<dyn ControllerPrintable>>,
    contr_constr: fn(state: &Rc<RefCell<State>>) -> Box<dyn ControllerPrintable>,
}

impl Servo {
    fn new(
        id: String,
        name: String,
        path: PathBuf,
        something: String,
        contr_constr: fn(state: &Rc<RefCell<State>>) -> Box<dyn ControllerPrintable>,
    ) -> Servo {
        Servo {
            state: Rc::new(RefCell::new(State {
                id,
                name,
                path,
                description: None,
            })),
            something,
            controller: None,
            contr_constr,
        }
    }

    fn init(&mut self) {
        self.controller = Some((self.contr_constr)(&self.state));
    }

    fn use_controller(&self) {
        self.controller.as_ref().unwrap().print_path();
    }

    fn set_new_path(&mut self) {
        self.state.borrow_mut().path = PathBuf::from("/rzeka/kolorado");
    }
}

fn main() {
    println!("Hello, world!");
    let mut servo = Servo::new(
        "id1".to_string(),
        "servo_left".to_string(),
        PathBuf::from("/something/loc"),
        "some text some text".to_string(),
        Controller::new,
    );
    servo.init();
    servo.set_new_path();
    servo.use_controller();
}
