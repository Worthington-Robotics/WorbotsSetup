use anyhow::Context;
use native_windows_derive::NwgUi;
use native_windows_gui as nwg;
use nwg::NativeUi;

use self::{clone::CloneUI, packages::PackagesUI};
use crate::assets::WORBOTS_ICON;

mod clone;
mod packages;

pub fn start_app() -> anyhow::Result<()> {
	nwg::init().context("Failed to start Native Windows GUI")?;
	nwg::Font::set_global_family("Segoe UI").context("Failed to set default font")?;
	let _ui = App::build_ui(Default::default()).context("Failed to build UI")?;
	nwg::dispatch_thread_events();
	Ok(())
}

#[derive(Default, NwgUi)]
pub struct App {
	// App structure
	#[nwg_control(title: "Worbots Setup", icon: Some(&data.icon))]
	#[nwg_events(
		OnWindowClose: [App::exit],
		OnInit: [App::init],
	)]
	window: nwg::Window,

	// Tray
	#[nwg_resource(source_bin: Some(WORBOTS_ICON))]
	icon: nwg::Icon,
	#[nwg_control(icon: Some(&data.icon), tip: Some("Worbots Setup"))]
	tray: nwg::TrayNotification,
	#[nwg_control(parent: window, popup: true)]
	tray_menu: nwg::Menu,

	#[nwg_layout(parent: window)]
	layout: nwg::GridLayout,

	// Tabs
	#[nwg_control(collection: vec!["Packages", "Clone"], size: (150, 80))]
	#[nwg_events( OnListBoxSelect: [App::change_interface] )]
	#[nwg_layout_item(layout: layout, row: 0, row_span: 1)]
	tab_menu: nwg::ListBox<&'static str>,

	#[nwg_control(position: (0, 100))]
	#[nwg_layout_item(layout: layout, row: 1, row_span: 5)]
	packages_frame: nwg::Frame,
	#[nwg_partial(parent: packages_frame)]
	packages_ui: PackagesUI,

	#[nwg_control(position: (0, 100))]
	#[nwg_layout_item(layout: layout, row: 1, row_span: 5)]
	clone_frame: nwg::Frame,
	#[nwg_partial(parent: packages_frame)]
	clone_ui: CloneUI,
}

impl App {
	fn init(&self) {
		self.tray.set_visibility(true);
		self.packages_ui.init();
		self.clone_ui.init();
	}

	fn change_interface(&self) {
		self.packages_frame.set_visible(false);
		self.clone_frame.set_visible(false);

		let layout = &self.layout;
		if layout.has_child(&self.packages_frame) {
			layout.remove_child(&self.packages_frame);
		}
		if layout.has_child(&self.clone_frame) {
			layout.remove_child(&self.clone_frame);
		}

		use nwg::stretch::{
			geometry::Size,
			style::{Dimension, Style},
		};
		let mut style = Style::default();
		style.size = Size {
			width: Dimension::Percent(1.0),
			height: Dimension::Auto,
		};

		match self.tab_menu.selection() {
			None | Some(0) => {
				self.set_tab(&self.packages_frame);
			}
			Some(1) => {
				self.set_tab(&self.clone_frame);
			}
			Some(_) => unreachable!(),
		}
	}

	fn set_tab(&self, frame: &nwg::Frame) {
		let item = nwg::GridLayoutItem::new(frame, 0, 1, 1, 5);
		self.layout.add_child_item(item);
		frame.set_visible(true);
	}

	fn exit(&self) {
		nwg::stop_thread_dispatch();
	}
}
