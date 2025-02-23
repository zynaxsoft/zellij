//! Definition of the actions that can be bound to keys.

use super::command::RunCommandAction;
use super::layout::{
    FloatingPaneLayout, Layout, RunPlugin, RunPluginLocation, SwapFloatingLayout, SwapTiledLayout,
    TiledPaneLayout,
};
use crate::cli::CliAction;
use crate::data::InputMode;
use crate::data::{Direction, Resize};
use crate::input::config::{Config, ConfigError, KdlError};
use crate::input::options::OnForceClose;
use crate::setup::{find_default_config_dir, get_layout_dir};
use miette::{NamedSource, Report};
use serde::{Deserialize, Serialize};

use std::path::PathBuf;
use std::str::FromStr;

use crate::position::Position;

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum ResizeDirection {
    Left,
    Right,
    Up,
    Down,
    Increase,
    Decrease,
}
impl FromStr for ResizeDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Left" | "left" => Ok(ResizeDirection::Left),
            "Right" | "right" => Ok(ResizeDirection::Right),
            "Up" | "up" => Ok(ResizeDirection::Up),
            "Down" | "down" => Ok(ResizeDirection::Down),
            "Increase" | "increase" | "+" => Ok(ResizeDirection::Increase),
            "Decrease" | "decrease" | "-" => Ok(ResizeDirection::Decrease),
            _ => Err(format!(
                "Failed to parse ResizeDirection. Unknown ResizeDirection: {}",
                s
            )),
        }
    }
}

#[derive(Clone, Copy, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SearchDirection {
    Down,
    Up,
}

impl FromStr for SearchDirection {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "Down" | "down" => Ok(SearchDirection::Down),
            "Up" | "up" => Ok(SearchDirection::Up),
            _ => Err(format!(
                "Failed to parse SearchDirection. Unknown SearchDirection: {}",
                s
            )),
        }
    }
}

#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum SearchOption {
    CaseSensitivity,
    WholeWord,
    Wrap,
}

impl FromStr for SearchOption {
    type Err = String;
    fn from_str(s: &str) -> Result<Self, Self::Err> {
        match s {
            "CaseSensitivity" | "casesensitivity" | "Casesensitivity" => {
                Ok(SearchOption::CaseSensitivity)
            },
            "WholeWord" | "wholeword" | "Wholeword" => Ok(SearchOption::WholeWord),
            "Wrap" | "wrap" => Ok(SearchOption::Wrap),
            _ => Err(format!(
                "Failed to parse SearchOption. Unknown SearchOption: {}",
                s
            )),
        }
    }
}

// As these actions are bound to the default config, please
// do take care when refactoring - or renaming.
// They might need to be adjusted in the default config
// as well `../../assets/config/default.yaml`
/// Actions that can be bound to keys.
#[derive(Clone, Debug, PartialEq, Eq, Deserialize, Serialize)]
pub enum Action {
    /// Quit Zellij.
    Quit,
    /// Write to the terminal.
    Write(Vec<u8>),
    /// Write Characters to the terminal.
    WriteChars(String),
    /// Switch to the specified input mode.
    SwitchToMode(InputMode),
    /// Switch all connected clients to the specified input mode.
    SwitchModeForAllClients(InputMode),
    /// Shrink/enlarge focused pane at specified border
    Resize(Resize, Option<Direction>),
    /// Switch focus to next pane in specified direction.
    FocusNextPane,
    FocusPreviousPane,
    /// Move the focus pane in specified direction.
    SwitchFocus,
    MoveFocus(Direction),
    /// Tries to move the focus pane in specified direction.
    /// If there is no pane in the direction, move to previous/next Tab.
    MoveFocusOrTab(Direction),
    MovePane(Option<Direction>),
    MovePaneBackwards,
    /// Clear all buffers of a current screen
    ClearScreen,
    /// Dumps the screen to a file
    DumpScreen(String, bool),
    /// Scroll up in focus pane.
    EditScrollback,
    ScrollUp,
    /// Scroll up at point
    ScrollUpAt(Position),
    /// Scroll down in focus pane.
    ScrollDown,
    /// Scroll down at point
    ScrollDownAt(Position),
    /// Scroll down to bottom in focus pane.
    ScrollToBottom,
    /// Scroll up to top in focus pane.
    ScrollToTop,
    /// Scroll up one page in focus pane.
    PageScrollUp,
    /// Scroll down one page in focus pane.
    PageScrollDown,
    /// Scroll up half page in focus pane.
    HalfPageScrollUp,
    /// Scroll down half page in focus pane.
    HalfPageScrollDown,
    /// Toggle between fullscreen focus pane and normal layout.
    ToggleFocusFullscreen,
    /// Toggle frames around panes in the UI
    TogglePaneFrames,
    /// Toggle between sending text commands to all panes on the current tab and normal mode.
    ToggleActiveSyncTab,
    /// Open a new pane in the specified direction (relative to focus).
    /// If no direction is specified, will try to use the biggest available space.
    NewPane(Option<Direction>, Option<String>), // String is an optional pane name
    /// Open the file in a new pane using the default editor
    EditFile(
        PathBuf,
        Option<usize>,
        Option<PathBuf>,
        Option<Direction>,
        bool,
    ), // usize is an optional line number, Option<PathBuf> is an optional cwd, bool is floating true/false
    /// Open a new floating pane
    NewFloatingPane(Option<RunCommandAction>, Option<String>), // String is an optional pane name
    /// Open a new tiled (embedded, non-floating) pane
    NewTiledPane(Option<Direction>, Option<RunCommandAction>, Option<String>), // String is an
    // optional pane
    // name
    /// Embed focused pane in tab if floating or float focused pane if embedded
    TogglePaneEmbedOrFloating,
    /// Toggle the visibility of all floating panes (if any) in the current Tab
    ToggleFloatingPanes,
    /// Close the focus pane.
    CloseFocus,
    PaneNameInput(Vec<u8>),
    UndoRenamePane,
    /// Create a new tab, optionally with a specified tab layout.
    NewTab(
        Option<TiledPaneLayout>,
        Vec<FloatingPaneLayout>,
        Option<Vec<SwapTiledLayout>>,
        Option<Vec<SwapFloatingLayout>>,
        Option<String>,
    ), // the String is the tab name
    /// Do nothing.
    NoOp,
    /// Go to the next tab.
    GoToNextTab,
    /// Go to the previous tab.
    GoToPreviousTab,
    /// Close the current tab.
    CloseTab,
    GoToTab(u32),
    GoToTabName(String, bool),
    ToggleTab,
    TabNameInput(Vec<u8>),
    UndoRenameTab,
    /// Run specified command in new pane.
    Run(RunCommandAction),
    /// Detach session and exit
    Detach,
    LeftClick(Position),
    RightClick(Position),
    MiddleClick(Position),
    LaunchOrFocusPlugin(RunPlugin, bool), // bool => should float
    LeftMouseRelease(Position),
    RightMouseRelease(Position),
    MiddleMouseRelease(Position),
    MouseHoldLeft(Position),
    MouseHoldRight(Position),
    MouseHoldMiddle(Position),
    Copy,
    /// Confirm a prompt
    Confirm,
    /// Deny a prompt
    Deny,
    /// Confirm an action that invokes a prompt automatically
    SkipConfirm(Box<Action>),
    /// Search for String
    SearchInput(Vec<u8>),
    /// Search for something
    Search(SearchDirection),
    /// Toggle case sensitivity of search
    SearchToggleOption(SearchOption),
    ToggleMouseMode,
    PreviousSwapLayout,
    NextSwapLayout,
    /// Query all tab names
    QueryTabNames,
    /// Open a new tiled (embedded, non-floating) plugin pane
    NewTiledPluginPane(RunPluginLocation, Option<String>), // String is an optional name
    NewFloatingPluginPane(RunPluginLocation, Option<String>), // String is an optional name
    StartOrReloadPlugin(RunPlugin),
}

impl Action {
    /// Checks that two Action are match except their mutable attributes.
    pub fn shallow_eq(&self, other_action: &Action) -> bool {
        match (self, other_action) {
            (Action::NewTab(..), Action::NewTab(..)) => true,
            _ => self == other_action,
        }
    }

    pub fn actions_from_cli(
        cli_action: CliAction,
        get_current_dir: Box<dyn Fn() -> PathBuf>,
        config: Option<Config>,
    ) -> Result<Vec<Action>, String> {
        match cli_action {
            CliAction::Write { bytes } => Ok(vec![Action::Write(bytes)]),
            CliAction::WriteChars { chars } => Ok(vec![Action::WriteChars(chars)]),
            CliAction::Resize { resize, direction } => Ok(vec![Action::Resize(resize, direction)]),
            CliAction::FocusNextPane => Ok(vec![Action::FocusNextPane]),
            CliAction::FocusPreviousPane => Ok(vec![Action::FocusPreviousPane]),
            CliAction::MoveFocus { direction } => Ok(vec![Action::MoveFocus(direction)]),
            CliAction::MoveFocusOrTab { direction } => Ok(vec![Action::MoveFocusOrTab(direction)]),
            CliAction::MovePane { direction } => Ok(vec![Action::MovePane(direction)]),
            CliAction::MovePaneBackwards => Ok(vec![Action::MovePaneBackwards]),
            CliAction::Clear => Ok(vec![Action::ClearScreen]),
            CliAction::DumpScreen { path, full } => Ok(vec![Action::DumpScreen(
                path.as_os_str().to_string_lossy().into(),
                full,
            )]),
            CliAction::EditScrollback => Ok(vec![Action::EditScrollback]),
            CliAction::ScrollUp => Ok(vec![Action::ScrollUp]),
            CliAction::ScrollDown => Ok(vec![Action::ScrollDown]),
            CliAction::ScrollToBottom => Ok(vec![Action::ScrollToBottom]),
            CliAction::ScrollToTop => Ok(vec![Action::ScrollToTop]),
            CliAction::PageScrollUp => Ok(vec![Action::PageScrollUp]),
            CliAction::PageScrollDown => Ok(vec![Action::PageScrollDown]),
            CliAction::HalfPageScrollUp => Ok(vec![Action::HalfPageScrollUp]),
            CliAction::HalfPageScrollDown => Ok(vec![Action::HalfPageScrollDown]),
            CliAction::ToggleFullscreen => Ok(vec![Action::ToggleFocusFullscreen]),
            CliAction::TogglePaneFrames => Ok(vec![Action::TogglePaneFrames]),
            CliAction::ToggleActiveSyncTab => Ok(vec![Action::ToggleActiveSyncTab]),
            CliAction::NewPane {
                direction,
                command,
                plugin,
                cwd,
                floating,
                name,
                close_on_exit,
                start_suspended,
            } => {
                let current_dir = get_current_dir();
                let cwd = cwd
                    .map(|cwd| current_dir.join(cwd))
                    .or_else(|| Some(current_dir));
                if let Some(plugin) = plugin {
                    if floating {
                        let plugin = RunPluginLocation::parse(&plugin, cwd).map_err(|e| {
                            format!("Failed to parse plugin loction {plugin}: {}", e)
                        })?;
                        Ok(vec![Action::NewFloatingPluginPane(plugin, name)])
                    } else {
                        let plugin = RunPluginLocation::parse(&plugin, cwd).map_err(|e| {
                            format!("Failed to parse plugin location {plugin}: {}", e)
                        })?;
                        // it is intentional that a new tiled plugin pane cannot include a
                        // direction
                        // this is because the cli client opening a tiled plugin pane is a
                        // different client than the one opening the pane, and this can potentially
                        // create very confusing races if the client changes focus while the plugin
                        // is being loaded
                        // this is not the case with terminal panes for historical reasons of
                        // backwards compatibility to a time before we had auto layouts
                        Ok(vec![Action::NewTiledPluginPane(plugin, name)])
                    }
                } else if !command.is_empty() {
                    let mut command = command.clone();
                    let (command, args) = (PathBuf::from(command.remove(0)), command);
                    let hold_on_start = start_suspended;
                    let hold_on_close = !close_on_exit;
                    let run_command_action = RunCommandAction {
                        command,
                        args,
                        cwd,
                        direction,
                        hold_on_close,
                        hold_on_start,
                    };
                    if floating {
                        Ok(vec![Action::NewFloatingPane(
                            Some(run_command_action),
                            name,
                        )])
                    } else {
                        Ok(vec![Action::NewTiledPane(
                            direction,
                            Some(run_command_action),
                            name,
                        )])
                    }
                } else {
                    if floating {
                        Ok(vec![Action::NewFloatingPane(None, name)])
                    } else {
                        Ok(vec![Action::NewTiledPane(direction, None, name)])
                    }
                }
            },
            CliAction::Edit {
                direction,
                file,
                line_number,
                floating,
                cwd,
            } => {
                let mut file = file;
                let current_dir = get_current_dir();
                let cwd = cwd
                    .map(|cwd| current_dir.join(cwd))
                    .or_else(|| Some(current_dir));
                if file.is_relative() {
                    if let Some(cwd) = cwd.as_ref() {
                        file = cwd.join(file);
                    }
                }
                Ok(vec![Action::EditFile(
                    file,
                    line_number,
                    cwd,
                    direction,
                    floating,
                )])
            },
            CliAction::SwitchMode { input_mode } => {
                Ok(vec![Action::SwitchModeForAllClients(input_mode)])
            },
            CliAction::TogglePaneEmbedOrFloating => Ok(vec![Action::TogglePaneEmbedOrFloating]),
            CliAction::ToggleFloatingPanes => Ok(vec![Action::ToggleFloatingPanes]),
            CliAction::ClosePane => Ok(vec![Action::CloseFocus]),
            CliAction::RenamePane { name } => Ok(vec![
                Action::UndoRenamePane,
                Action::PaneNameInput(name.as_bytes().to_vec()),
            ]),
            CliAction::UndoRenamePane => Ok(vec![Action::UndoRenamePane]),
            CliAction::GoToNextTab => Ok(vec![Action::GoToNextTab]),
            CliAction::GoToPreviousTab => Ok(vec![Action::GoToPreviousTab]),
            CliAction::CloseTab => Ok(vec![Action::CloseTab]),
            CliAction::GoToTab { index } => Ok(vec![Action::GoToTab(index)]),
            CliAction::GoToTabName { name, create } => Ok(vec![Action::GoToTabName(name, create)]),
            CliAction::RenameTab { name } => Ok(vec![
                Action::TabNameInput(vec![0]),
                Action::TabNameInput(name.as_bytes().to_vec()),
            ]),
            CliAction::UndoRenameTab => Ok(vec![Action::UndoRenameTab]),
            CliAction::NewTab {
                name,
                layout,
                layout_dir,
                cwd,
            } => {
                let current_dir = get_current_dir();
                let cwd = cwd
                    .map(|cwd| current_dir.join(cwd))
                    .or_else(|| Some(current_dir));
                if let Some(layout_path) = layout {
                    let layout_dir = layout_dir
                        .or_else(|| config.and_then(|c| c.options.layout_dir))
                        .or_else(|| get_layout_dir(find_default_config_dir()));
                    let (path_to_raw_layout, raw_layout, swap_layouts) =
                        Layout::stringified_from_path_or_default(Some(&layout_path), layout_dir)
                            .map_err(|e| format!("Failed to load layout: {}", e))?;
                    let layout = Layout::from_str(&raw_layout, path_to_raw_layout, swap_layouts.as_ref().map(|(f, p)| (f.as_str(), p.as_str())), cwd).map_err(|e| {
                        let stringified_error = match e {
                            ConfigError::KdlError(kdl_error) => {
                                let error = kdl_error.add_src(layout_path.as_path().as_os_str().to_string_lossy().to_string(), String::from(raw_layout));
                                let report: Report = error.into();
                                format!("{:?}", report)
                            }
                            ConfigError::KdlDeserializationError(kdl_error) => {
                                let error_message = match kdl_error.kind {
                                    kdl::KdlErrorKind::Context("valid node terminator") => {
                                        format!("Failed to deserialize KDL node. \nPossible reasons:\n{}\n{}\n{}\n{}",
                                        "- Missing `;` after a node name, eg. { node; another_node; }",
                                        "- Missing quotations (\") around an argument node eg. { first_node \"argument_node\"; }",
                                        "- Missing an equal sign (=) between node arguments on a title line. eg. argument=\"value\"",
                                        "- Found an extraneous equal sign (=) between node child arguments and their values. eg. { argument=\"value\" }")
                                    },
                                    _ => String::from(kdl_error.help.unwrap_or("Kdl Deserialization Error")),
                                };
                                let kdl_error = KdlError {
                                    error_message,
                                    src: Some(NamedSource::new(layout_path.as_path().as_os_str().to_string_lossy().to_string(), String::from(raw_layout))),
                                    offset: Some(kdl_error.span.offset()),
                                    len: Some(kdl_error.span.len()),
                                    help_message: None,
                                };
                                let report: Report = kdl_error.into();
                                format!("{:?}", report)
                            },
                            e => format!("{}", e)
                        };
                        stringified_error
                    })?;
                    let mut tabs = layout.tabs();
                    if tabs.len() > 1 {
                        return Err(format!("Tab layout cannot itself have tabs"));
                    } else if !tabs.is_empty() {
                        let swap_tiled_layouts = Some(layout.swap_tiled_layouts.clone());
                        let swap_floating_layouts = Some(layout.swap_floating_layouts.clone());
                        let (tab_name, layout, floating_panes_layout) =
                            tabs.drain(..).next().unwrap();
                        let name = tab_name.or(name);
                        Ok(vec![Action::NewTab(
                            Some(layout),
                            floating_panes_layout,
                            swap_tiled_layouts,
                            swap_floating_layouts,
                            name,
                        )])
                    } else {
                        let swap_tiled_layouts = Some(layout.swap_tiled_layouts.clone());
                        let swap_floating_layouts = Some(layout.swap_floating_layouts.clone());
                        let (layout, floating_panes_layout) = layout.new_tab();
                        Ok(vec![Action::NewTab(
                            Some(layout),
                            floating_panes_layout,
                            swap_tiled_layouts,
                            swap_floating_layouts,
                            name,
                        )])
                    }
                } else {
                    Ok(vec![Action::NewTab(None, vec![], None, None, name)])
                }
            },
            CliAction::PreviousSwapLayout => Ok(vec![Action::PreviousSwapLayout]),
            CliAction::NextSwapLayout => Ok(vec![Action::NextSwapLayout]),
            CliAction::QueryTabNames => Ok(vec![Action::QueryTabNames]),
            CliAction::StartOrReloadPlugin { url } => {
                let current_dir = get_current_dir();
                let run_plugin_location = RunPluginLocation::parse(&url, Some(current_dir))
                    .map_err(|e| format!("Failed to parse plugin location: {}", e))?;
                let run_plugin = RunPlugin {
                    location: run_plugin_location,
                    _allow_exec_host_cmd: false,
                };
                Ok(vec![Action::StartOrReloadPlugin(run_plugin)])
            },
            CliAction::LaunchOrFocusPlugin { url, floating } => {
                let current_dir = get_current_dir();
                let run_plugin_location = RunPluginLocation::parse(url.as_str(), Some(current_dir))
                    .map_err(|e| format!("Failed to parse plugin location: {}", e))?;
                let run_plugin = RunPlugin {
                    location: run_plugin_location,
                    _allow_exec_host_cmd: false,
                };
                Ok(vec![Action::LaunchOrFocusPlugin(run_plugin, floating)])
            },
        }
    }
}

impl From<OnForceClose> for Action {
    fn from(ofc: OnForceClose) -> Action {
        match ofc {
            OnForceClose::Quit => Action::Quit,
            OnForceClose::Detach => Action::Detach,
        }
    }
}
