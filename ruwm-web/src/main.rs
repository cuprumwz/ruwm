#![recursion_limit = "1024"]

use std::cell::RefCell;
use std::rc::Rc;

use log::*;

use yew::prelude::*;
use yew_router::prelude::*;

use edge_frame::frame::*;
use edge_frame::redust::*;
use edge_frame::role::*;
use edge_frame::wifi::*;

use crate::battery::*;
use crate::middleware::apply_middleware;
use crate::state::*;
use crate::valve::*;
use crate::ws::open;

mod battery;
mod error;
mod middleware;
mod state;
mod valve;
mod ws;

#[derive(Debug, Routable, Copy, Clone, PartialEq, Eq, Hash)]
enum Routes {
    #[at("/wifi")]
    Wifi,
    #[at("/")]
    Home,
}

#[function_component(App)]
fn app() -> Html {
    // let ws = use_state(|| {
    //     let (sender, receiver) = open("/ws").unwrap();

    //     (
    //         Rc::new(RefCell::new(sender)),
    //         Rc::new(RefCell::new(receiver)),
    //     )
    // });

    // let request_id_gen = use_mut_ref(|| 0_usize);

    // let store = apply_middleware(
    //     use_store(|| Rc::new(AppState::new())),
    //     ws.0.clone(),
    //     ws.1.clone(),
    //     request_id_gen,
    // )
    // .unwrap();

    let store = use_store(|| Rc::new(AppState::new()));

    html! {
        <ContextProvider<UseStoreHandle<AppState>> context={store.clone()}>
            <BrowserRouter>
                <Switch<Routes> render={Switch::render(render)}/>
            </BrowserRouter>
        </ContextProvider<UseStoreHandle<AppState>>>
    }
}

fn render(route: &Routes) -> Html {
    html! {
        <Role<AppState> role={RoleValue::User} projection={AppState::role()} auth=true>
            <Frame
                app_title="RUWM"
                app_url="https://github.com/ivmarkov/edge-frame">
                <Nav>
                    <Role<AppState> role={RoleValue::User} projection={AppState::role()}>
                        <RouteNavItem<Routes> text="Home" route={Routes::Home}/>
                    </Role<AppState>>
                    <Role<AppState> role={RoleValue::Admin} projection={AppState::role()}>
                        <NavGroup text="Settings">
                            <WifiNavItem<Routes> route={Routes::Wifi}/>
                        </NavGroup>
                    </Role<AppState>>
                </Nav>
                <Status>
                    <Role<AppState> role={RoleValue::User} projection={AppState::role()}>
                        <WifiStatusItem<Routes, AppState> route={Routes::Wifi} projection={AppState::wifi()}/>
                    </Role<AppState>>
                </Status>
                <Content>
                    {
                        match route {
                            Routes::Home => html! {
                                <>
                                <Valve<AppState> projection={AppState::valve()}/>
                                <Battery<AppState> projection={AppState::battery()}/>
                                </>
                            },
                            Routes::Wifi => html! {
                                <Role<AppState> role={RoleValue::Admin} projection={AppState::role()} auth=true>
                                    <Wifi<AppState> projection={AppState::wifi()}/>
                                </Role<AppState>>
                            },
                        }
                    }
                </Content>
            </Frame>
        </Role<AppState>>
    }
}

fn main() {
    wasm_logger::init(wasm_logger::Config::default());

    yew::start_app::<App>();
}