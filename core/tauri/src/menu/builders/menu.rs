// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

use crate::{menu::*, Icon, Manager, Runtime};

/// A builder type for [`Menu`]
///
/// # Example
///
/// ```no_run
/// use tauri::menu::*;
/// tauri::Builder::default()
///   .setup(move |app| {
///     let handle = app.handle();
///     # let icon1 = tauri::Icon::Rgba {
///     #   rgba: Vec::new(),
///     #   width: 0,
///     #   height: 0,
///     # };
///     let menu = MenuBuilder::new(handle)
///       .item(&MenuItem::new(handle, "MenuItem 1", true, None))
///       .items(&[
///         &CheckMenuItem::new(handle, "CheckMenuItem 1", true, true, None),
///         &IconMenuItem::new(handle, "IconMenuItem 1", true, Some(icon1), None),
///       ])
///       .separator()
///       .cut()
///       .copy()
///       .paste()
///       .separator()
///       .text("MenuItem 2")
///       .check("CheckMenuItem 2")
///       .icon("IconMenuItem 2", app.default_window_icon().cloned().unwrap())
///       .build()?;
///     app.set_menu(menu);
///     Ok(())
///   });
/// ```
pub struct MenuBuilder<'m, R: Runtime, M: Manager<R>> {
  id: Option<MenuId>,
  manager: &'m M,
  items: Vec<MenuItemKind<R>>,
}

impl<'m, R: Runtime, M: Manager<R>> MenuBuilder<'m, R, M> {
  /// Create a new menu builder.
  pub fn new(manager: &'m M) -> Self {
    Self {
      id: None,
      items: Vec::new(),
      manager,
    }
  }

  /// Create a new menu builder with the specified id.
  pub fn with_id<I: Into<MenuId>>(manager: &'m M, id: I) -> Self {
    Self {
      id: Some(id.into()),
      items: Vec::new(),
      manager,
    }
  }

  /// Set the id for this menu.
  pub fn id<I: Into<MenuId>>(mut self, id: I) -> Self {
    self.id.replace(id.into());
    self
  }

  /// Add this item to the menu.
  pub fn item(mut self, item: &dyn IsMenuItem<R>) -> Self {
    self.items.push(item.kind());
    self
  }

  /// Add these items to the menu.
  pub fn items(mut self, items: &[&dyn IsMenuItem<R>]) -> Self {
    for item in items {
      self = self.item(*item);
    }
    self
  }

  /// Add a [MenuItem] to the menu.
  pub fn text<S: AsRef<str>>(mut self, text: S) -> Self {
    self
      .items
      .push(MenuItem::new(self.manager, text, true, None).kind());
    self
  }

  /// Add a [CheckMenuItem] to the menu.
  pub fn check<S: AsRef<str>>(mut self, text: S) -> Self {
    self
      .items
      .push(CheckMenuItem::new(self.manager, text, true, true, None).kind());
    self
  }

  /// Add an [IconMenuItem] to the menu.
  pub fn icon<S: AsRef<str>>(mut self, text: S, icon: Icon) -> Self {
    self
      .items
      .push(IconMenuItem::new(self.manager, text, true, Some(icon), None).kind());
    self
  }

  /// Add an [IconMenuItem] with a native icon to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux**: Unsupported.
  pub fn native_icon<S: AsRef<str>>(mut self, text: S, icon: NativeIcon) -> Self {
    self
      .items
      .push(IconMenuItem::with_native_icon(self.manager, text, true, Some(icon), None).kind());
    self
  }

  /// Add Separator menu item to the menu.
  pub fn separator(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::separator(self.manager).kind());
    self
  }

  /// Add Copy menu item to the menu.
  pub fn copy(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::copy(self.manager, None).kind());
    self
  }

  /// Add Cut menu item to the menu.
  pub fn cut(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::cut(self.manager, None).kind());
    self
  }

  /// Add Paste menu item to the menu.
  pub fn paste(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::paste(self.manager, None).kind());
    self
  }

  /// Add SelectAll menu item to the menu.
  pub fn select_all(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::select_all(self.manager, None).kind());
    self
  }

  /// Add Undo menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux:** Unsupported.
  pub fn undo(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::undo(self.manager, None).kind());
    self
  }
  /// Add Redo menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux:** Unsupported.
  pub fn redo(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::redo(self.manager, None).kind());
    self
  }

  /// Add Minimize window menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn minimize(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::minimize(self.manager, None).kind());
    self
  }

  /// Add Maximize window menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn maximize(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::maximize(self.manager, None).kind());
    self
  }

  /// Add Fullscreen menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux:** Unsupported.
  pub fn fullscreen(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::fullscreen(self.manager, None).kind());
    self
  }

  /// Add Hide window menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn hide(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::hide(self.manager, None).kind());
    self
  }

  /// Add Hide other windows menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn hide_others(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::hide_others(self.manager, None).kind());
    self
  }

  /// Add Show all app windows menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux:** Unsupported.
  pub fn show_all(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::show_all(self.manager, None).kind());
    self
  }

  /// Add Close window menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn close_window(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::close_window(self.manager, None).kind());
    self
  }

  /// Add Quit app menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Linux:** Unsupported.
  pub fn quit(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::quit(self.manager, None).kind());
    self
  }

  /// Add About app menu item to the menu.
  pub fn about(mut self, metadata: Option<AboutMetadata>) -> Self {
    self
      .items
      .push(PredefinedMenuItem::about(self.manager, None, metadata).kind());
    self
  }

  /// Add Services menu item to the menu.
  ///
  /// ## Platform-specific:
  ///
  /// - **Windows / Linux:** Unsupported.
  pub fn services(mut self) -> Self {
    self
      .items
      .push(PredefinedMenuItem::services(self.manager, None).kind());
    self
  }

  /// Builds this menu
  pub fn build(self) -> crate::Result<Menu<R>> {
    if self.items.is_empty() {
      Ok(if let Some(id) = self.id {
        Menu::with_id(self.manager, id)
      } else {
        Menu::new(self.manager)
      })
    } else {
      let items = self
        .items
        .iter()
        .map(|i| i as &dyn IsMenuItem<R>)
        .collect::<Vec<_>>();
      if let Some(id) = self.id {
        Menu::with_id_and_items(self.manager, id, &items)
      } else {
        Menu::with_items(self.manager, &items)
      }
    }
  }
}