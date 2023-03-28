use std::ops::Deref;

use wasm_bindgen::{self, JsCast, JsValue};
use web_sys::{self, window, Element, console, Event};
use leptos_reactive::{self, create_signal, Scope, create_runtime, create_scope, SignalUpdate, SignalGet, create_effect};

#[derive(Debug, Clone)]
struct El(Element);

impl Deref for El {
    type Target = Element;

    fn deref(&self) -> &Self::Target {
        &self.0
    }
}

impl El {
    fn new(tagname: &str)-> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();

        let el = document.create_element(tagname).unwrap();
        Self(el)
    }

    fn on(self, event_name: &str, cb: impl FnMut(Event)+'static) -> Self {
        use wasm_bindgen::prelude::Closure;
        let closure = Closure::wrap(Box::new(cb) as Box<dyn FnMut(Event)>);
        self.0.add_event_listener_with_callback(
            event_name, 
            closure.as_ref().unchecked_ref()
        ).unwrap();
        closure.forget();
        self
    }

    fn attr(self, key: &str, value: &str) -> Self {
        self.0.set_attribute(key, value).unwrap();
        self
    }

    fn text(self, text: &str) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node(text);
        self.0.append_child(&node).unwrap();
        self
    }

    fn child(self, el: &El)-> Self {
        self.0.append_child(el).unwrap();
        self
    }

    fn dyn_text(self, cx: Scope, callback: impl Fn()-> String+ 'static) -> Self {
        let window = window().unwrap();
        let document = window.document().unwrap();
        let node = document.create_text_node("");
        self.0.append_child(&node);
        create_effect(cx, move|_| {
            let s = callback();
            node.set_data(&s);
        });
        self
    }
}

fn mount(callback: impl FnOnce(Scope)-> El + 'static) {
    let runtime = create_runtime();
    _ = create_scope(runtime, |cx|{
        let root = callback(cx);
        let window = window().unwrap();
        let document = window.document().unwrap();
        let body = document.body().unwrap();
        body.append_child(&root).unwrap();
    });
}

fn main() {
    mount(|cx| {
        let (count, set_count) = create_signal(cx, 0);
        El::new("div")
            .child(
                &El::new("button")
                    .on("click", move |_| set_count.update(|n| *n -=1))
                    .attr("id", "de-button")
                    .text("-")
            )
            .text(" Value: ")
            .dyn_text(cx, move || {
                count.get().to_string()
            })
            .child(&El::new("button")
                .on("click", move |_| set_count.update(|n| *n +=1))
                .attr("id", "in-button")
                .text("+")
        )
    });
}
