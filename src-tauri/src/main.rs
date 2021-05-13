#![cfg_attr(
  all(not(debug_assertions), target_os = "windows"),
  windows_subsystem = "windows"
)]

use tauri::{Menu, MenuItem};

fn main() {
  let menu = vec![
    // on macOS, first menu is always app name
    Menu::new("Josteph", vec![
      MenuItem::About("Josteph".to_string()),
      MenuItem::Separator,
      MenuItem::Hide,
      MenuItem::HideOthers,
      MenuItem::ShowAll,
      MenuItem::Separator,
      MenuItem::Quit,
    ]),
    Menu::new(
      "Edit",
      vec![
        MenuItem::Undo,
        MenuItem::Redo,
        MenuItem::Separator,
        MenuItem::Cut,
        MenuItem::Copy,
        MenuItem::Paste,
        MenuItem::Separator,
        MenuItem::SelectAll,
      ],
    ),
    Menu::new(
      "View",
      vec![
        MenuItem::EnterFullScreen,
      ],
    ),
    Menu::new(
      "Window",
      vec![
        MenuItem::Minimize,
        MenuItem::CloseWindow,
      ],
    )
  ];

  tauri::Builder::default()
    .menu(menu)
    .run(tauri::generate_context!())
    .expect("error while running tauri application");
}
