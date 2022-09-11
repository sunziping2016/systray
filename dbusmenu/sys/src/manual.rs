use crate::{DbusmenuClient, DbusmenuServer};

extern "C" {
    pub fn dbusmenu_client_get_icon_paths(client: *mut DbusmenuClient) -> glib::GStrv;
    pub fn dbusmenu_server_get_icon_paths(server: *mut DbusmenuServer) -> glib::GStrv;
    pub fn dbusmenu_server_set_icon_paths(server: *mut DbusmenuServer, icon_paths: glib::GStrv);
}
