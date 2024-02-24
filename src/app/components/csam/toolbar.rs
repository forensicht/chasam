use crate::fl;

use relm4::{
    prelude::*,
    gtk::prelude::*,
    component::{
        AsyncComponent, 
        AsyncComponentParts, 
        AsyncComponentSender,
    },
    MessageBroker,
};
use relm4_icons::icon_name;

pub static SELECT_BROKER: MessageBroker<ToolbarInput> = MessageBroker::new();

pub struct ToolbarModel {
    selection_count: usize,
}

#[derive(Debug)]
pub enum SizeOption {
    Size0KB,
    Size30KB,
    Size100KB,
    Size500KB,
    SizeGreater500KB,
}

#[derive(Debug)]
pub enum ToolbarInput {
    CleanFilters,
    AllSizeOptions(bool),
    SelectedItem(bool),
    CheckButtonToggled(SizeOption, bool),
}

#[derive(Debug)]
pub enum ToolbarOutput {
    ZoomIn,
    ZoomOut,
    SelectAll(bool),
    SearchEntry(String),
    SizeFilter0KB(bool),
    SizeFilter30KB(bool),
    SizeFilter100KB(bool),
    SizeFilter500KB(bool),
    SizeFilterGreater500KB(bool),
}

#[relm4::component(pub async)]
impl AsyncComponent for ToolbarModel {
    type Init = ();
    type Input = ToolbarInput;
    type Output = ToolbarOutput;
    type CommandOutput = ();

    view! {
        #[name = "toolbar"]
        gtk::Box {
            set_orientation: gtk::Orientation::Horizontal,
            set_hexpand: true,
            set_margin_end: 6,
            set_css_classes: &["padding-all"],

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_width_request: 388,
                set_hexpand: false,
                set_halign: gtk::Align::Start,

                gtk::Button {
                    set_icon_name: icon_name::MINUS,
                    set_tooltip: fl!("deselect-all"),
                    add_css_class: "flat",
                    connect_clicked[sender] => move |_| {
                        sender.output(ToolbarOutput::SelectAll(false)).unwrap();
                    },
                },

                gtk::Button {
                    set_icon_name: icon_name::CHECKMARK,
                    set_tooltip: fl!("select-all"),
                    add_css_class: "flat",
                    connect_clicked[sender] => move |_| {
                        sender.output(ToolbarOutput::SelectAll(true)).unwrap();
                    },
                },

                gtk::Label {
                    set_label: fl!("selected-count"),
                    set_xalign: 0.0,
                    set_margin_start: 6,
                    add_css_class: "dim-label",
                },

                gtk::Label {
                    #[watch]
                    set_label: &model.selection_count.to_string(),
                    set_xalign: 0.0,
                    add_css_class: "dim-label",
                    set_margin_start: 4,
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_hexpand: true,
                set_margin_start: 12,
                set_margin_end: 12,
                set_halign: gtk::Align::Center,
                set_spacing: 6,

                gtk::Button {
                    set_icon_name: icon_name::MINUS,
                    set_tooltip: fl!("zoom-out"),
                    add_css_class: "circular",
                    connect_clicked[sender] => move |_| {
                        sender.output(ToolbarOutput::ZoomOut).unwrap_or_default();
                    }, 
                },

                gtk::Button {
                    set_icon_name: icon_name::PLUS,
                    set_tooltip: fl!("zoom-in"),
                    add_css_class: "circular",
                    connect_clicked[sender] => move |_| {
                        sender.output(ToolbarOutput::ZoomIn).unwrap_or_default();
                    },
                },
            },

            gtk::Box {
                set_orientation: gtk::Orientation::Horizontal,
                set_width_request: 388,
                set_hexpand: false,
                set_halign: gtk::Align::End,
                set_spacing: 6,

                append = &gtk::Button {
                    set_label: fl!("clean-filters"),
                    add_css_class: "flat",
                    connect_clicked => ToolbarInput::CleanFilters,
                },

                append = &gtk::MenuButton {
                    set_label: fl!("filter"),
                    set_always_show_arrow: true,
                    set_css_classes: &["flat"],
                    
                    #[wrap(Some)]
                    set_popover: popover = &gtk::Popover {
                        gtk::Box {
                            set_orientation: gtk::Orientation::Vertical, 
                            set_margin_all: 1,

                            gtk::Label {
                                set_label: fl!("size"),
                                set_xalign: 0.0,
                                set_margin_bottom: 3,
                                set_css_classes: &["title-4"],
                            },

                            gtk::Frame {
                                gtk::ListBox {
                                    set_selection_mode: gtk::SelectionMode::None,
                                    set_show_separators: false,
                                    set_css_classes: &["rich-list"],

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_all_size)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::AllSizeOptions(checkbox.is_active()));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: fl!("all-sizes"),
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_size_0kb)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::CheckButtonToggled(
                                                        SizeOption::Size0KB, 
                                                        checkbox.is_active(),
                                                    ));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: "0 KB",
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_size_30kb)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::CheckButtonToggled(
                                                        SizeOption::Size30KB, 
                                                        checkbox.is_active(),
                                                    ));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: "1 - 30 KB",
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_size_100kb)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::CheckButtonToggled(
                                                        SizeOption::Size100KB, 
                                                        checkbox.is_active(),
                                                    ));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: "31 - 100 KB",
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_size_500kb)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::CheckButtonToggled(
                                                        SizeOption::Size500KB, 
                                                        checkbox.is_active(),
                                                    ));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: "101 - 500 KB",
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },

                                    gtk::ListBoxRow {
                                        gtk::Box {
                                            #[name(chk_size_greater_500kb)]
                                            gtk::CheckButton {
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_margin_start: 0,
                                                set_margin_end: 0,
                                                set_active: true,
                                                connect_toggled[sender] => move |checkbox| {
                                                    sender.input(ToolbarInput::CheckButtonToggled(
                                                        SizeOption::SizeGreater500KB, 
                                                        checkbox.is_active(),
                                                    ));
                                                },
                                            },
                                            gtk::Label {
                                                set_label: fl!("greater-than-500-kb"),
                                                set_xalign: 0.0,
                                                set_halign: gtk::Align::Start,
                                                set_valign: gtk::Align::Center,
                                                set_hexpand: true,
                                            },
                                        }
                                    },
                                }
                            },
                        },
                    },
                },

                #[name(search_entry)]
                append = &gtk::SearchEntry {
                    set_placeholder_text: Some(fl!("search")),
                    set_search_delay: 500,
                    connect_search_changed[sender] => move |entry| {
                        let query = entry.text().as_str().to_string();
                        sender.output(ToolbarOutput::SearchEntry(query)).unwrap_or_default();
                    }
                },
            }
        }
    }

    async fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: AsyncComponentSender<Self>,
    ) -> AsyncComponentParts<Self> {
        let model = ToolbarModel {
            selection_count: 0,
        };
        let widgets = view_output!();

        AsyncComponentParts { model, widgets }
    }

    async fn update_with_view(
        &mut self,
        widgets: &mut Self::Widgets,
        message: Self::Input,
        sender: AsyncComponentSender<Self>,
        _root: &Self::Root,
    ) {
        match message {
            ToolbarInput::AllSizeOptions(is_selected) => {
                if is_selected {
                    widgets.chk_size_0kb.set_active(true);
                    widgets.chk_size_30kb.set_active(true);
                    widgets.chk_size_100kb.set_active(true);
                    widgets.chk_size_500kb.set_active(true);
                    widgets.chk_size_greater_500kb.set_active(true);
                }
            }
            ToolbarInput::CleanFilters => {                
                widgets.chk_all_size.set_active(true);
                widgets.search_entry.set_text("");
            }
            ToolbarInput::SelectedItem(is_selected) => {
                if is_selected {
                    self.selection_count += 1;
                } else if self.selection_count > 0 {
                    self.selection_count -= 1;
                } else {
                    self.selection_count = 0;
                }
            }
            ToolbarInput::CheckButtonToggled(size_option, is_selected) => {
                if !is_selected {
                    widgets.chk_all_size.set_active(false);
                }

                match size_option {
                    SizeOption::Size0KB => { 
                        sender.output(ToolbarOutput::SizeFilter0KB(is_selected))
                            .unwrap_or_default();
                    }
                    SizeOption::Size30KB => { 
                        sender.output(ToolbarOutput::SizeFilter30KB(is_selected))
                            .unwrap_or_default();
                    }
                    SizeOption::Size100KB => { 
                        sender.output(ToolbarOutput::SizeFilter100KB(is_selected))
                            .unwrap_or_default();
                    }
                    SizeOption::Size500KB => {
                        sender.output(ToolbarOutput::SizeFilter500KB(is_selected))
                            .unwrap_or_default();
                    }
                    SizeOption::SizeGreater500KB => {
                        sender.output(ToolbarOutput::SizeFilterGreater500KB(is_selected))
                            .unwrap_or_default();
                    }
                }
            }
        }

        self.update_view(widgets, sender);    
    }

}
