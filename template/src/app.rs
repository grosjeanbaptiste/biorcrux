use biorcrux::widgets::{AppShell, NavItem, Stat};
use dioxus::prelude::*;

#[component]
pub fn App() -> Element {
    let mut active = use_signal(|| "home".to_string());
    let items = vec![
        NavItem::new("home", "Home").with_icon("⌂"),
        NavItem::new("about", "About").with_icon("ⓘ"),
    ];
    rsx! {
        style { {biorcrux::design::STYLESHEET} }
        style { {biorcrux::widgets::STYLESHEET} }
        AppShell {
            items,
            active: active(),
            nav_label: "__TITLE__",
            title: "__TITLE__",
            on_select: move |id| active.set(id),
            h1 { "__TITLE__" }
            Stat { label: "Active page", value: active() }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn renders_the_shell_with_its_navigation() {
        let mut dom = VirtualDom::new(App);
        dom.rebuild_in_place();
        let html = dioxus_ssr::render(&dom);
        assert!(html.contains("bx-shell") && html.contains("Home"));
    }
}
