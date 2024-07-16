use crate::{InstallationType, NavigationAction, Page, INSTALLATION_STATE};
use gettextrs::gettext;
use gtk::prelude::*;
use libhelium::prelude::*;
use relm4::{ComponentParts, ComponentSender, RelmWidgetExt, SimpleComponent};

pub struct InstallationTypePage {}

#[derive(Debug)]
pub enum InstallationTypePageMsg {
    Update,
    #[doc(hidden)]
    Navigate(NavigationAction),
    InstallationTypeSelected(InstallationType),
}

#[derive(Debug)]
pub enum InstallationTypePageOutput {
    Navigate(NavigationAction),
}

#[relm4::component(pub)]
impl SimpleComponent for InstallationTypePage {
    type Init = ();
    type Input = InstallationTypePageMsg;
    type Output = InstallationTypePageOutput;

    view! {
        libhelium::ViewMono {
            #[watch]
            set_title: &gettext("Installation Type"),
            set_vexpand: true,

            add = &gtk::Box {
                set_orientation: gtk::Orientation::Vertical,
                set_spacing: 6,

                gtk::Box {
                    set_orientation: gtk::Orientation::Vertical,
                    set_valign: gtk::Align::Center,
                    set_spacing: 18,

                    gtk::Box {
                        set_orientation: gtk::Orientation::Vertical,
                        set_spacing: 6,
                        set_vexpand: true,
                        set_hexpand: true,
                        set_valign: gtk::Align::Center,
                        set_halign: gtk::Align::Center,

                        gtk::Image {
                            set_icon_name: Some("drive-harddisk-symbolic"),
                            inline_css: "-gtk-icon-size: 128px"
                        },

                        gtk::Label {
                            #[watch]
                            set_label: &INSTALLATION_STATE.read().destination_disk.clone().map(|d| d.disk_name).unwrap_or("".to_owned()),
                            inline_css: "font-size: 16px; font-weight: bold"
                        },

                        gtk::Label {
                            #[watch]
                            set_label: &INSTALLATION_STATE.read().destination_disk.clone().map(|d| d.os_name).unwrap_or("".to_owned()),
                        }
                    },

                    gtk::Box {
                        set_spacing: 6,
                        set_halign: gtk::Align::Center,
                        set_valign: gtk::Align::End,
                        set_homogeneous: true,
                        // libhelium::PillButton {
                        //     set_label: &gettext("Entire Disk"),
                        //     inline_css: "padding-left: 48px; padding-right: 48px",
                        //     connect_clicked => InstallationTypePageMsg::InstallationTypeSelected(InstallationType::WholeDisk)
                        // },
                        // libhelium::PillButton {
                        //     set_label: &gettext("Dual Boot"),
                        //     inline_css: "padding-left: 48px; padding-right: 48px",
                        //     connect_clicked => InstallationTypePageMsg::InstallationTypeSelected(InstallationType::DualBoot)
                        // },
                        // libhelium::PillButton {
                        //     set_label: &gettext("Custom"),
                        //     inline_css: "padding-left: 48px; padding-right: 48px",
                        //     connect_clicked => InstallationTypePageMsg::InstallationTypeSelected(InstallationType::Custom)
                        // },
                        libhelium::PillButton {
                            #[watch]
                            set_label: &gettext("Chromebook"),
                            inline_css: "padding-left: 48px; padding-right: 48px",
                            connect_clicked => InstallationTypePageMsg::InstallationTypeSelected(InstallationType::ChromebookInstall)
                        },
                    }
                },

                gtk::Box {
                    set_orientation: gtk::Orientation::Horizontal,
                    set_spacing: 6,

                    libhelium::TextButton {
                        #[watch]
                        set_label: &gettext("Previous"),
                        connect_clicked => InstallationTypePageMsg::Navigate(NavigationAction::GoTo(crate::Page::Destination))
                    },

                    gtk::Box {
                        set_hexpand: true,
                    }
                }
            }
        }
    }

    fn init(
        _init: Self::Init,
        root: Self::Root,
        sender: ComponentSender<Self>,
    ) -> ComponentParts<Self> {
        let model = Self {};

        let widgets = view_output!();

        INSTALLATION_STATE.subscribe(sender.input_sender(), |_| InstallationTypePageMsg::Update);

        ComponentParts { model, widgets }
    }

    fn update(&mut self, message: Self::Input, sender: ComponentSender<Self>) {
        match message {
            InstallationTypePageMsg::InstallationTypeSelected(InstallationType::WholeDisk) => {
                let mut installation_state_guard = INSTALLATION_STATE.write();
                installation_state_guard.installation_type = Some(InstallationType::WholeDisk);
                sender
                    .output(InstallationTypePageOutput::Navigate(
                        NavigationAction::GoTo(Page::Confirmation),
                    ))
                    .unwrap()
            }
            InstallationTypePageMsg::InstallationTypeSelected(InstallationType::DualBoot(_)) => {
                todo!()
            }
            InstallationTypePageMsg::InstallationTypeSelected(InstallationType::Custom) => todo!(),
            InstallationTypePageMsg::InstallationTypeSelected(
                InstallationType::ChromebookInstall,
            ) => {
                let mut installation_state_guard = INSTALLATION_STATE.write();
                installation_state_guard.installation_type =
                    Some(InstallationType::ChromebookInstall);
                sender
                    .output(InstallationTypePageOutput::Navigate(
                        NavigationAction::GoTo(Page::Confirmation),
                    ))
                    .unwrap()
            }
            InstallationTypePageMsg::Navigate(action) => sender
                .output(InstallationTypePageOutput::Navigate(action))
                .unwrap(),
            InstallationTypePageMsg::Update => {}
        }
    }
}
