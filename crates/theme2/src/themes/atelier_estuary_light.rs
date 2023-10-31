use gpui2::rgba;

use crate::{PlayerTheme, SyntaxTheme, Theme, ThemeMetadata};

pub fn atelier_estuary_light() -> Theme {
    Theme {
        metadata: ThemeMetadata {
            name: "Atelier Estuary Light".into(),
            is_light: true,
        },
        transparent: rgba(0x00000000).into(),
        mac_os_traffic_light_red: rgba(0xec695eff).into(),
        mac_os_traffic_light_yellow: rgba(0xf4bf4eff).into(),
        mac_os_traffic_light_green: rgba(0x61c553ff).into(),
        border: rgba(0x969585ff).into(),
        border_variant: rgba(0x969585ff).into(),
        border_focused: rgba(0xbbddc6ff).into(),
        border_transparent: rgba(0x00000000).into(),
        elevated_surface: rgba(0xc5c4b9ff).into(),
        surface: rgba(0xebeae3ff).into(),
        background: rgba(0xc5c4b9ff).into(),
        filled_element: rgba(0xc5c4b9ff).into(),
        filled_element_hover: rgba(0xffffff1e).into(),
        filled_element_active: rgba(0xffffff28).into(),
        filled_element_selected: rgba(0xd9ecdfff).into(),
        filled_element_disabled: rgba(0x00000000).into(),
        ghost_element: rgba(0x00000000).into(),
        ghost_element_hover: rgba(0xffffff14).into(),
        ghost_element_active: rgba(0xffffff1e).into(),
        ghost_element_selected: rgba(0xd9ecdfff).into(),
        ghost_element_disabled: rgba(0x00000000).into(),
        text: rgba(0x22221bff).into(),
        text_muted: rgba(0x61604fff).into(),
        text_placeholder: rgba(0xba6336ff).into(),
        text_disabled: rgba(0x767463ff).into(),
        text_accent: rgba(0x37a165ff).into(),
        icon_muted: rgba(0x61604fff).into(),
        syntax: SyntaxTheme {
            highlights: vec![
                ("string.special".into(), rgba(0x9d6b7bff).into()),
                ("link_text".into(), rgba(0xae7214ff).into()),
                ("emphasis.strong".into(), rgba(0x37a165ff).into()),
                ("tag".into(), rgba(0x37a165ff).into()),
                ("primary".into(), rgba(0x302f27ff).into()),
                ("emphasis".into(), rgba(0x37a165ff).into()),
                ("hint".into(), rgba(0x758961ff).into()),
                ("title".into(), rgba(0x22221bff).into()),
                ("string.regex".into(), rgba(0x5a9d47ff).into()),
                ("attribute".into(), rgba(0x37a165ff).into()),
                ("string.escape".into(), rgba(0x5f5e4eff).into()),
                ("embedded".into(), rgba(0x22221bff).into()),
                ("punctuation.bracket".into(), rgba(0x5f5e4eff).into()),
                (
                    "function.special.definition".into(),
                    rgba(0xa5980cff).into(),
                ),
                ("operator".into(), rgba(0x5f5e4eff).into()),
                ("constant".into(), rgba(0x7c9728ff).into()),
                ("comment.doc".into(), rgba(0x5f5e4eff).into()),
                ("label".into(), rgba(0x37a165ff).into()),
                ("variable".into(), rgba(0x302f27ff).into()),
                ("punctuation".into(), rgba(0x302f27ff).into()),
                ("punctuation.delimiter".into(), rgba(0x5f5e4eff).into()),
                ("comment".into(), rgba(0x878573ff).into()),
                ("punctuation.special".into(), rgba(0x9d6b7bff).into()),
                ("string.special.symbol".into(), rgba(0x7c9725ff).into()),
                ("enum".into(), rgba(0xae7214ff).into()),
                ("variable.special".into(), rgba(0x5f9182ff).into()),
                ("link_uri".into(), rgba(0x7c9728ff).into()),
                ("punctuation.list_marker".into(), rgba(0x302f27ff).into()),
                ("number".into(), rgba(0xae7312ff).into()),
                ("function".into(), rgba(0x35a166ff).into()),
                ("text.literal".into(), rgba(0xae7214ff).into()),
                ("boolean".into(), rgba(0x7c9728ff).into()),
                ("predictive".into(), rgba(0x879a72ff).into()),
                ("type".into(), rgba(0xa5980cff).into()),
                ("constructor".into(), rgba(0x37a165ff).into()),
                ("property".into(), rgba(0xba6135ff).into()),
                ("keyword".into(), rgba(0x5f9182ff).into()),
                ("function.method".into(), rgba(0x35a166ff).into()),
                ("variant".into(), rgba(0xa5980cff).into()),
                ("string".into(), rgba(0x7c9725ff).into()),
                ("preproc".into(), rgba(0x22221bff).into()),
            ],
        },
        status_bar: rgba(0xc5c4b9ff).into(),
        title_bar: rgba(0xc5c4b9ff).into(),
        toolbar: rgba(0xf4f3ecff).into(),
        tab_bar: rgba(0xebeae3ff).into(),
        editor: rgba(0xf4f3ecff).into(),
        editor_subheader: rgba(0xebeae3ff).into(),
        editor_active_line: rgba(0xebeae3ff).into(),
        terminal: rgba(0xf4f3ecff).into(),
        image_fallback_background: rgba(0xc5c4b9ff).into(),
        git_created: rgba(0x7c9728ff).into(),
        git_modified: rgba(0x37a165ff).into(),
        git_deleted: rgba(0xba6336ff).into(),
        git_conflict: rgba(0xa5980fff).into(),
        git_ignored: rgba(0x767463ff).into(),
        git_renamed: rgba(0xa5980fff).into(),
        players: [
            PlayerTheme {
                cursor: rgba(0x37a165ff).into(),
                selection: rgba(0x37a1653d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x7c9728ff).into(),
                selection: rgba(0x7c97283d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x9d6b7bff).into(),
                selection: rgba(0x9d6b7b3d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xae7214ff).into(),
                selection: rgba(0xae72143d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x5f9182ff).into(),
                selection: rgba(0x5f91823d).into(),
            },
            PlayerTheme {
                cursor: rgba(0x5c9d49ff).into(),
                selection: rgba(0x5c9d493d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xba6336ff).into(),
                selection: rgba(0xba63363d).into(),
            },
            PlayerTheme {
                cursor: rgba(0xa5980fff).into(),
                selection: rgba(0xa5980f3d).into(),
            },
        ],
    }
}