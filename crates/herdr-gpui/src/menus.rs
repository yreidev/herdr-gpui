//! Shared definitions for the macOS native and the Linux and Windows
//! in-window menu bars.
//! Every item dispatches the same action the palette and the keymap use.

pub(crate) mod in_window;

use crate::{
    CheckForUpdates, Quit, RunCommand, ShowLogs,
    actions::{Copy, Cut, Paste, SelectAll, SetLayout},
    config::{Layout, LayoutMode},
    controls::Command,
};
#[cfg(target_os = "macos")]
use crate::{Hide, HideOthers, Minimize, ShowAll};
#[cfg(feature = "qa-menu")]
use crate::{
    PlaySound, ShowHerdrNotDetected, ShowUpdatePreview,
    actions::{
        RingBellPreview, ShowToastPreview, ShowUpdateDownloadPreview, ShowUpdateHomebrewPreview,
    },
};
use gpui::{App, Menu, MenuItem, OsAction};
#[cfg(feature = "qa-menu")]
use herdr_client::protocol::SemanticNotificationKind;
use rust_i18n::t;

/// Linux and Windows have no global menu bar, so each window draws its own.
pub(crate) const IN_WINDOW: bool = cfg!(any(target_os = "linux", windows));

/// Only headings are needed while the in-window menus are closed.
struct Headings(Vec<(gpui::SharedString, bool)>);
impl gpui::Global for Headings {}

/// Installs the menu bar, checking the layout the latest config picked.
pub(crate) fn install(cx: &mut App) {
    let layout = cx
        .try_global::<crate::app::InitialAppearance>()
        .map_or_else(Layout::default, |appearance| appearance.config.layout);
    let menus = menus(layout);
    cx.set_global(Headings(
        menus
            .iter()
            .map(|menu| (menu.name.clone(), menu.disabled))
            .collect(),
    ));
    cx.set_menus(menus);
}

/// View > Layout: Herdr's densities, their rounded versions, the layouts
/// with a design of their own, then Devices, which lists agents by device;
/// the one in use checked.
fn layout_menu(current: LayoutMode) -> MenuItem {
    let items = LayoutMode::ALL
        .iter()
        .enumerate()
        .flat_map(|(index, &mode)| {
            let group = matches!(index, 3 | 6 | 9).then(MenuItem::separator);
            group.into_iter().chain([
                MenuItem::action(t!(mode.label()), SetLayout { mode }).checked(mode == current)
            ])
        });
    MenuItem::submenu(Menu::new(t!("Layout")).items(items))
}

/// The menu bar, with `layout`'s mode checked under View > Layout.
pub(crate) fn menus(layout: Layout) -> Vec<Menu> {
    vec![
        Menu {
            name: "Herdr".into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("About Herdr"),
                    RunCommand {
                        command: Command::About,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Command Palette"),
                    RunCommand {
                        command: Command::Palette,
                    },
                ),
                MenuItem::action(
                    t!("Settings"),
                    RunCommand {
                        command: Command::Settings,
                    },
                ),
                MenuItem::action(
                    t!("Keyboard Shortcuts"),
                    RunCommand {
                        command: Command::Keybinds,
                    },
                ),
                MenuItem::action(t!("Check for Updates..."), CheckForUpdates),
                // macOS reserves Hide for every app; without these items
                // Cmd-H has no key equivalent and does nothing.
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Hide Herdr"), Hide),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Hide Others"), HideOthers),
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Show All"), ShowAll),
                MenuItem::separator(),
                MenuItem::action(t!("Quit Herdr"), Quit),
            ],
        },
        Menu {
            name: t!("File").into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("New Workspace"),
                    RunCommand {
                        command: Command::Workspace,
                    },
                ),
                MenuItem::action(
                    t!("New Worktree..."),
                    RunCommand {
                        command: Command::NewWorktree,
                    },
                ),
                MenuItem::action(
                    t!("New Tab"),
                    RunCommand {
                        command: Command::Tab,
                    },
                ),
                MenuItem::action(
                    t!("Go To"),
                    RunCommand {
                        command: Command::WorkspacePicker,
                    },
                ),
                MenuItem::action(
                    t!("Go to Symbol..."),
                    RunCommand {
                        command: Command::GoToSymbol,
                    },
                ),
                MenuItem::action(
                    t!("Go to File..."),
                    RunCommand {
                        command: Command::GoToFile,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Close Pane..."),
                    RunCommand {
                        command: Command::ClosePane,
                    },
                ),
                MenuItem::action(
                    t!("Close Tab..."),
                    RunCommand {
                        command: Command::CloseTab,
                    },
                ),
                MenuItem::action(
                    t!("Reopen Closed Tab"),
                    RunCommand {
                        command: Command::ReopenClosedTab,
                    },
                ),
            ],
        },
        // OS actions also route these items to native text fields, such as
        // the file panels, the way every macOS app's Edit menu does.
        Menu {
            name: t!("Edit").into(),
            disabled: false,
            items: vec![
                MenuItem::os_action(t!("Cut"), Cut, OsAction::Cut),
                MenuItem::os_action(t!("Copy"), Copy, OsAction::Copy),
                MenuItem::os_action(t!("Paste"), Paste, OsAction::Paste),
                MenuItem::separator(),
                MenuItem::os_action(t!("Select All"), SelectAll, OsAction::SelectAll),
            ],
        },
        Menu {
            name: t!("View").into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("Increase Font Size"),
                    RunCommand {
                        command: Command::IncreaseFontSize,
                    },
                ),
                MenuItem::action(
                    t!("Decrease Font Size"),
                    RunCommand {
                        command: Command::DecreaseFontSize,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Reset Font Size"),
                    RunCommand {
                        command: Command::ResetFontSize,
                    },
                ),
                MenuItem::separator(),
                layout_menu(layout.mode),
                MenuItem::action(
                    t!("Devices Overview"),
                    RunCommand {
                        command: Command::DevicesOverview,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Toggle Full Screen"),
                    RunCommand {
                        command: Command::ToggleFullScreen,
                    },
                ),
                MenuItem::action(
                    t!("Toggle Zen Mode"),
                    RunCommand {
                        command: Command::ToggleZenMode,
                    },
                ),
            ],
        },
        Menu {
            name: t!("Terminal").into(),
            disabled: false,
            items: vec![
                MenuItem::action(
                    t!("Split Vertically (Right)"),
                    RunCommand {
                        command: Command::SplitRight,
                    },
                ),
                MenuItem::action(
                    t!("Split Horizontally (Down)"),
                    RunCommand {
                        command: Command::SplitDown,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Next Tab"),
                    RunCommand {
                        command: Command::NextTab,
                    },
                ),
                MenuItem::action(
                    t!("Previous Tab"),
                    RunCommand {
                        command: Command::PreviousTab,
                    },
                ),
                MenuItem::action(
                    t!("Back"),
                    RunCommand {
                        command: Command::Back,
                    },
                ),
                MenuItem::action(
                    t!("Forward"),
                    RunCommand {
                        command: Command::Forward,
                    },
                ),
                MenuItem::action(
                    t!("Toggle Pane Zoom"),
                    RunCommand {
                        command: Command::Zoom,
                    },
                ),
                MenuItem::action(
                    t!("Clear Pane"),
                    RunCommand {
                        command: Command::ClearPane,
                    },
                ),
                MenuItem::action(
                    t!("Find"),
                    RunCommand {
                        command: Command::Find,
                    },
                ),
                MenuItem::action(
                    t!("Find Next"),
                    RunCommand {
                        command: Command::FindNext,
                    },
                ),
                MenuItem::action(
                    t!("Find Previous"),
                    RunCommand {
                        command: Command::FindPrevious,
                    },
                ),
                MenuItem::action(
                    t!("Copy Mode"),
                    RunCommand {
                        command: Command::CopyMode,
                    },
                ),
                MenuItem::action(
                    t!("Open Scrollback in Editor"),
                    RunCommand {
                        command: Command::EditScrollback,
                    },
                ),
                MenuItem::action(
                    t!("Open Notification Target"),
                    RunCommand {
                        command: Command::OpenNotificationTarget,
                    },
                ),
                MenuItem::action(
                    t!("Toggle Sidebar"),
                    RunCommand {
                        command: Command::ToggleSidebar,
                    },
                ),
                MenuItem::action(
                    t!("Toggle Status Bar"),
                    RunCommand {
                        command: Command::ToggleStatusBar,
                    },
                ),
                MenuItem::action(
                    t!("Open VS Code"),
                    RunCommand {
                        command: Command::OpenCode,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    t!("Reconnect"),
                    RunCommand {
                        command: Command::Reconnect,
                    },
                ),
            ],
        },
        Menu {
            name: t!("Window").into(),
            disabled: false,
            items: vec![
                // Without this item Cmd-M has no key equivalent and does
                // nothing; the pane zoom above is unrelated to the window.
                #[cfg(target_os = "macos")]
                MenuItem::action(t!("Minimize"), Minimize),
                #[cfg(target_os = "macos")]
                MenuItem::separator(),
                MenuItem::action(
                    t!("New Window"),
                    RunCommand {
                        command: Command::NewWindow,
                    },
                ),
                MenuItem::action(
                    t!("Cycle Through Windows"),
                    RunCommand {
                        command: Command::CycleWindows,
                    },
                ),
                MenuItem::action(
                    t!("Close Window"),
                    RunCommand {
                        command: Command::CloseWindow,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(t!("Logs"), ShowLogs),
            ],
        },
        #[cfg(feature = "qa-menu")]
        Menu {
            name: "QA".into(),
            disabled: false,
            items: vec![
                MenuItem::action("Show herdr non-detected modal", ShowHerdrNotDetected),
                MenuItem::action("Show app update available", ShowUpdatePreview),
                MenuItem::action(
                    "Show update download progress (50%)",
                    ShowUpdateDownloadPreview,
                ),
                MenuItem::action("Show Homebrew update progress", ShowUpdateHomebrewPreview),
                MenuItem::action("Play Sound", PlaySound),
                MenuItem::action("Ring Bell in 3 Seconds", RingBellPreview),
                #[cfg(any(target_os = "macos", windows))]
                MenuItem::action(
                    "Enable badge",
                    crate::actions::SetBadgePreview { enabled: true },
                ),
                #[cfg(any(target_os = "macos", windows))]
                MenuItem::action(
                    "Disable badge preview",
                    crate::actions::SetBadgePreview { enabled: false },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Show NeedsAttention toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::NeedsAttention,
                    },
                ),
                MenuItem::action(
                    "Show Finished toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::Finished,
                    },
                ),
                MenuItem::action(
                    "Show UpdateInstalled toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::UpdateInstalled,
                    },
                ),
                MenuItem::action(
                    "Show Custom toast",
                    ShowToastPreview {
                        kind: SemanticNotificationKind::Custom,
                    },
                ),
                MenuItem::separator(),
                MenuItem::action(
                    "Send notification in 3 seconds",
                    crate::actions::ShowSystemNotificationPreview,
                ),
            ],
        },
    ]
}

#[cfg(test)]
#[allow(clippy::unwrap_used, clippy::expect_used)]
mod tests;
