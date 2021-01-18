use seed::{prelude::*, *};
use wateringcan::*;

fn init(_: Url, orders: &mut impl Orders<Msg>) -> Model {
    // Need to start the initial rendered call
    orders.after_next_render(Msg::Rendered);

    Model::default()
}

#[derive(Default)]
struct Model {
    // WateringCan implements Default
    wc: WateringCan,
}

#[derive(Copy, Clone)]
enum Msg {
    Rendered(RenderInfo),
    Clicked,
}

fn update(msg: Msg, model: &mut Model, orders: &mut impl Orders<Msg>) {
    match msg {
        Msg::Rendered(render_info) => {
            // Must call to update the status of the animations
            model.wc.tick(render_info.timestamp_delta);
            // Creates a kind of "render loop" that will be called very frequently
            orders.after_next_render(Msg::Rendered);
        },
        Msg::Clicked => {
            model.wc.create_animation(
                // ID of the animation
                "move-button", 
                Animation {
                    //Name of the style attribute to animate
                    attribute: String::from("transform"),
                    // Duration in ms
                    duration: 2000.0,
                    // Start value
                    from: 0.0,
                    // Ending value
                    to: 400.0,
                    // Prefix and suffix for the transform style (see: https://developer.mozilla.org/en-US/docs/Web/CSS/transform-function/translateX)
                    prefix: String::from("translateX("),
                    suffix: String::from("px)")
                }
            );
        }
    }
}

fn view(model: &Model) -> Node<Msg> {
    button![
        ev(Ev::Click, |_| Msg::Clicked),
        // WateringCan::animation returns something that implements UpdateEl, so it can be placed as a "child" of a component in Seed
        // Call with the animation ID
        model.wc.animation("move-button"),
        "Click me >>>",
    ]
}

#[wasm_bindgen(start)]
pub fn start() {
    App::start("app", init, update, view);
}