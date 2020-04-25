use std::rc::Rc;
use std::cell::Cell;

use wasm_bindgen::prelude::*;
use serde_derive::{Serialize, Deserialize};
use futures_signals::signal::{Signal, SignalExt, Mutable};
use futures_signals::signal_vec::{SignalVec, SignalVecExt, MutableVec};
use dominator::{Dom, text_signal, html, clone, events, link};


use crate::utils::*;

// this is the instance of the app but i do not see it in the samples in the repository
thread_local! {
    pub static APP: Rc<App> = App::new();
}


#[derive(Debug, Serialize, Deserialize)]
pub struct App {
  cnct_name: Mutable<String>,
  cnct_mesg: Mutable<String>
}


impl App {
  pub fn new() -> Rc<Self> {
      Rc::new(App {
        cnct_name: Mutable::new("".to_string()),
        cnct_mesg: Mutable::new("".to_string()),
      })
  }



  pub fn render(app: Rc<Self>) -> Dom {

    // this section is just for the POST part
    html!("div", {
      .class(&*CT_MAIN_OUTTR)
      .children(&mut [
        html!("div", {
          .children(&mut [
            html!("div", {
              .children(&mut [
                html!("div", {
                  .class(&*AT_LINE_FLD)
                  .children(&mut [ 
                    
                    html!("p", {
                      .text("name")
                    }),
                    html!("input", {
                      .property_signal("value", app.cnct_name.signal_cloned())
                      .event(clone!(app => move | event: events::Input| {
                        app.cnct_name.set_neq(event.value().unwrap_throw());
                      }))
                    })
                  ])
                }),
                html!("div",{
                  .children(&mut[
                    
                    html!("p", {
                    .text("Message")
                    }),
                    html!("textarea", {
                      .property_signal("value", app.cnct_mesg.signal_cloned())
                      .event(clone!(app => move |event: events::Input| {
                        app.cnct_mesg.set_neq(event.value().unwrap_throw());
                      }))
                    }),
                  ])
                }),
                html!("div", {
                  .children(&mut[
                    
                    html!("button", {
                      .text("submit")
                    })
                  ])
                })
              ])
            }),
            // this will go into another module .rs file  but will be the 'map' part. this will be connected by diesel
            // this is going to be the POST request
            // i put it here for now to remind myself were it will be inserted. A additional X will go there for a DELETE
            // in this section there will also be a + symbol for a edit it is not a production ready thing i just want to get a CRUD POC 
            // working.
            html!("div", {
              .children(&mut [
                html!("p", {
                  .text("name - message")
                  }
                )
              ])
            })
          ])
        })
      ])
    })
  }
}
