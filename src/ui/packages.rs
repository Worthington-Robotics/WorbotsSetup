use std::cell::RefCell;

use native_windows_derive::NwgPartial;
use native_windows_gui as nwg;

use crate::utils::tokio_exec;
use crate::{data::Data, utils::tokio_exec_deferred, output::CommonOutput};
use crate::package::{ALL_PACKAGES, Package};

#[derive(Default, NwgPartial)]
pub struct PackagesUI {
	// #[nwg_layout(max_size: [1000, 150], min_size: [100, 120])]
	#[nwg_layout]
	layout: nwg::GridLayout,

	#[nwg_control(item_count: 10, list_style: nwg::ListViewStyle::Simple, focus: true,
		ex_flags: nwg::ListViewExFlags::FULL_ROW_SELECT, 
	)]
	#[nwg_events(OnListViewItemChanged: [PackagesUI::select_pkg])]
	#[nwg_layout_item(layout: layout, col: 0, col_span: 1, row: 0, row_span: 10)]
	data_view: nwg::ListView,
	selected_pkg: RefCell<Option<Package>>,

	#[nwg_partial(parent: details_pane_frame)]
	details_pane: PackageDetailsPane,
	#[nwg_control]
	#[nwg_layout_item(layout: layout, col: 1, col_span: 2, row: 0, row_span: 10)]
	details_pane_frame: nwg::Frame,
	#[nwg_control]
	#[nwg_layout_item(layout: layout, col: 1, col_span: 2, row: 0, row_span: 10)]
	details_pane_empty_frame: nwg::Frame,
}

impl PackagesUI {
	pub fn init(&self) {
		self.details_pane_frame.set_visible(false);

		self.data_view.insert_column(nwg::InsertListViewColumn {
			index: Some(0),
			fmt: None,
			width: Some(150),
			text: Some("Package name:".into()),
		});
		self.data_view.set_headers_enabled(true);

		for package in ALL_PACKAGES {
			// Pad with spaces so that the selection box is larger
			let mut text = package.display_name().to_string();
			let pad_size = 45;
			if text.len() < pad_size {
				let pad_size = pad_size - text.len();
				text.push_str(&" ".repeat(pad_size));
			}
			
			self.data_view.insert_item(nwg::InsertListViewItem {
				index: None,
				column_index: 0,
				text: Some(text),
				image: None,
			});
		}
	}

	fn select_pkg(&self) {
		let old_val = self.selected_pkg.replace(self.data_view.selected_item().map(|x| {
			*ALL_PACKAGES.get(x).expect("Index out of range for available packages")
		}));

		// Update the details pane if the selected package has changed
		if old_val != *self.selected_pkg.borrow() {
			self.update_pane();
		}
	}

	fn update_pane(&self) {
		self.details_pane_frame.set_visible(false);
		self.details_pane_empty_frame.set_visible(false);
		if self.layout.has_child(&self.details_pane_frame) {
			self.layout.remove_child(&self.details_pane_frame);
		}
		if self.layout.has_child(&self.details_pane_empty_frame) {
			self.layout.remove_child(&self.details_pane_empty_frame);
		}

		let selected_pkg = self.selected_pkg.borrow();
		if let Some(pkg) = *selected_pkg {
			self.details_pane.set_package(Some(pkg));
			let item = nwg::GridLayoutItem::new(&self.details_pane_frame, 1, 0, 1, 4);
			self.layout.add_child_item(item);
			self.details_pane_frame.set_visible(true);
		} else {
			self.details_pane.set_package(None);
			let item = nwg::GridLayoutItem::new(&self.details_pane_empty_frame, 1, 0, 1, 4);
			self.layout.add_child_item(item);
			self.details_pane_empty_frame.set_visible(true);
		}
	}
}

#[derive(NwgPartial, Default)]
struct PackageDetailsPane {
	// #[nwg_layout(max_size: [1000, 320], min_size: [100, 420])]
	#[nwg_layout]
	layout: nwg::GridLayout,

	#[nwg_resource(family: "Segoe UI", size: 14)]
	font_sui_small: nwg::Font,

	selected_pkg: RefCell<Option<Package>>,

	#[nwg_control(text: "Package name")]
	#[nwg_layout_item(layout: layout, col: 0, row: 0)]
	name_label: nwg::Label,

	#[nwg_control(text: "Package description", font: Some(&data.font_sui_small))]
	#[nwg_layout_item(layout: layout, col: 0, row: 2, row_span: 2)]
	desc_label: nwg::Label,

	#[nwg_control(text: "Install/Update", size: (100, 50))]
	#[nwg_layout_item(layout: layout, col: 0, row: 5, row_span: 3)]
	#[nwg_events(OnButtonClick: [PackageDetailsPane::install_package])]
	install_button: nwg::Button,

	#[nwg_control(text: "Launch", size: (100, 50))]
	#[nwg_layout_item(layout: layout, col: 0, row: 9, row_span: 3)]
	#[nwg_events(OnButtonClick: [PackageDetailsPane::launch_package])]
	launch_button: nwg::Button,
}

impl PackageDetailsPane {
	fn set_package(&self, package: Option<Package>) {
		self.selected_pkg.replace(package);
		if let Some(package) = package {
			self.name_label.set_text(package.display_name());
			let desc_wrapped = textwrap::wrap(package.short_description(), 30).join("\r\n");
			self.desc_label.set_text(&desc_wrapped);
			self.launch_button.set_visible(package.can_launch());
		}
	}

	fn install_package(&self) {
		if let Some(pkg) = *self.selected_pkg.borrow() {
			self.install_button.set_enabled(false);
			self.install_button.set_text("Installing...");
			println!("Installing package {pkg}");

			tokio_exec_deferred(async move {
				let mut out = CommonOutput;
				let mut data = Data::new(&mut out).expect("Failed to create application data");
				pkg.install(&mut data).await.expect("Failed to install package");
			}).expect("Failed to execute task");

			self.install_button.set_text("Install/Update");
			self.install_button.set_enabled(true);
		}
	}

	fn launch_package(&self) {
		if let Some(pkg) = *self.selected_pkg.borrow() {
			self.launch_button.set_enabled(false);
			self.launch_button.set_text("Launching...");
			println!("Launching package {pkg}");

			tokio_exec(async {
				let mut out = CommonOutput;
				let mut data = Data::new(&mut out).expect("Failed to create application data");
				pkg.launch(&mut data).await.expect("Failed to install package");
			}).expect("Failed to execute task");

			self.launch_button.set_text("Launch");
			self.launch_button.set_enabled(true);
		}
	}
}
