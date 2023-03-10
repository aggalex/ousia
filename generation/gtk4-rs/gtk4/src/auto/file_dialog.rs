// This file was generated by gir (https://github.com/gtk-rs/gir)
// from gir-files (https://github.com/gtk-rs/gir-files)
// DO NOT EDIT

use crate::{FileFilter, Window};
use glib::{
    prelude::*,
    signal::{connect_raw, SignalHandlerId},
    translate::*,
};
use std::{boxed::Box as Box_, fmt, mem::transmute, pin::Pin, ptr};

glib::wrapper! {
    #[doc(alias = "GtkFileDialog")]
    pub struct FileDialog(Object<ffi::GtkFileDialog, ffi::GtkFileDialogClass>);

    match fn {
        type_ => || ffi::gtk_file_dialog_get_type(),
    }
}

impl FileDialog {
    #[doc(alias = "gtk_file_dialog_new")]
    pub fn new() -> FileDialog {
        assert_initialized_main_thread!();
        unsafe { from_glib_full(ffi::gtk_file_dialog_new()) }
    }

    // rustdoc-stripper-ignore-next
    /// Creates a new builder-pattern struct instance to construct [`FileDialog`] objects.
    ///
    /// This method returns an instance of [`FileDialogBuilder`](crate::builders::FileDialogBuilder) which can be used to create [`FileDialog`] objects.
    pub fn builder() -> FileDialogBuilder {
        FileDialogBuilder::default()
    }

    #[doc(alias = "gtk_file_dialog_get_accept_label")]
    #[doc(alias = "get_accept_label")]
    pub fn accept_label(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_file_dialog_get_accept_label(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_get_default_filter")]
    #[doc(alias = "get_default_filter")]
    pub fn default_filter(&self) -> Option<FileFilter> {
        unsafe {
            from_glib_none(ffi::gtk_file_dialog_get_default_filter(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_dialog_get_filters")]
    #[doc(alias = "get_filters")]
    pub fn filters(&self) -> Option<gio::ListModel> {
        unsafe { from_glib_none(ffi::gtk_file_dialog_get_filters(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_get_initial_file")]
    #[doc(alias = "get_initial_file")]
    pub fn initial_file(&self) -> Option<gio::File> {
        unsafe { from_glib_none(ffi::gtk_file_dialog_get_initial_file(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_get_initial_folder")]
    #[doc(alias = "get_initial_folder")]
    pub fn initial_folder(&self) -> Option<gio::File> {
        unsafe {
            from_glib_none(ffi::gtk_file_dialog_get_initial_folder(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_dialog_get_initial_name")]
    #[doc(alias = "get_initial_name")]
    pub fn initial_name(&self) -> Option<glib::GString> {
        unsafe { from_glib_none(ffi::gtk_file_dialog_get_initial_name(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_get_modal")]
    #[doc(alias = "get_modal")]
    pub fn is_modal(&self) -> bool {
        unsafe { from_glib(ffi::gtk_file_dialog_get_modal(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_get_shortcut_folders")]
    #[doc(alias = "get_shortcut_folders")]
    pub fn shortcut_folders(&self) -> Option<gio::ListModel> {
        unsafe {
            from_glib_none(ffi::gtk_file_dialog_get_shortcut_folders(
                self.to_glib_none().0,
            ))
        }
    }

    #[doc(alias = "gtk_file_dialog_get_title")]
    #[doc(alias = "get_title")]
    pub fn title(&self) -> glib::GString {
        unsafe { from_glib_none(ffi::gtk_file_dialog_get_title(self.to_glib_none().0)) }
    }

    #[doc(alias = "gtk_file_dialog_open")]
    pub fn open<P: FnOnce(Result<gio::File, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn open_trampoline<
            P: FnOnce(Result<gio::File, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_file_dialog_open_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = open_trampoline::<P>;
        unsafe {
            ffi::gtk_file_dialog_open(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn open_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::File, glib::Error>> + 'static>> {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.open(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_file_dialog_open_multiple")]
    pub fn open_multiple<P: FnOnce(Result<gio::ListModel, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn open_multiple_trampoline<
            P: FnOnce(Result<gio::ListModel, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_file_dialog_open_multiple_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = open_multiple_trampoline::<P>;
        unsafe {
            ffi::gtk_file_dialog_open_multiple(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn open_multiple_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::ListModel, glib::Error>> + 'static>>
    {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.open_multiple(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_file_dialog_save")]
    pub fn save<P: FnOnce(Result<gio::File, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn save_trampoline<
            P: FnOnce(Result<gio::File, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_file_dialog_save_finish(_source_object as *mut _, res, &mut error);
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = save_trampoline::<P>;
        unsafe {
            ffi::gtk_file_dialog_save(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn save_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::File, glib::Error>> + 'static>> {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.save(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_file_dialog_select_folder")]
    pub fn select_folder<P: FnOnce(Result<gio::File, glib::Error>) + 'static>(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn select_folder_trampoline<
            P: FnOnce(Result<gio::File, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_file_dialog_select_folder_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = select_folder_trampoline::<P>;
        unsafe {
            ffi::gtk_file_dialog_select_folder(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn select_folder_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<Box_<dyn std::future::Future<Output = Result<gio::File, glib::Error>> + 'static>> {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.select_folder(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_file_dialog_select_multiple_folders")]
    pub fn select_multiple_folders<
        P: FnOnce(Result<Option<gio::ListModel>, glib::Error>) + 'static,
    >(
        &self,
        parent: Option<&impl IsA<Window>>,
        cancellable: Option<&impl IsA<gio::Cancellable>>,
        callback: P,
    ) {
        let main_context = glib::MainContext::ref_thread_default();
        let is_main_context_owner = main_context.is_owner();
        let has_acquired_main_context = (!is_main_context_owner)
            .then(|| main_context.acquire().ok())
            .flatten();
        assert!(
            is_main_context_owner || has_acquired_main_context.is_some(),
            "Async operations only allowed if the thread is owning the MainContext"
        );

        let user_data: Box_<glib::thread_guard::ThreadGuard<P>> =
            Box_::new(glib::thread_guard::ThreadGuard::new(callback));
        unsafe extern "C" fn select_multiple_folders_trampoline<
            P: FnOnce(Result<Option<gio::ListModel>, glib::Error>) + 'static,
        >(
            _source_object: *mut glib::gobject_ffi::GObject,
            res: *mut gio::ffi::GAsyncResult,
            user_data: glib::ffi::gpointer,
        ) {
            let mut error = ptr::null_mut();
            let ret = ffi::gtk_file_dialog_select_multiple_folders_finish(
                _source_object as *mut _,
                res,
                &mut error,
            );
            let result = if error.is_null() {
                Ok(from_glib_full(ret))
            } else {
                Err(from_glib_full(error))
            };
            let callback: Box_<glib::thread_guard::ThreadGuard<P>> =
                Box_::from_raw(user_data as *mut _);
            let callback: P = callback.into_inner();
            callback(result);
        }
        let callback = select_multiple_folders_trampoline::<P>;
        unsafe {
            ffi::gtk_file_dialog_select_multiple_folders(
                self.to_glib_none().0,
                parent.map(|p| p.as_ref()).to_glib_none().0,
                cancellable.map(|p| p.as_ref()).to_glib_none().0,
                Some(callback),
                Box_::into_raw(user_data) as *mut _,
            );
        }
    }

    pub fn select_multiple_folders_future(
        &self,
        parent: Option<&(impl IsA<Window> + Clone + 'static)>,
    ) -> Pin<
        Box_<
            dyn std::future::Future<Output = Result<Option<gio::ListModel>, glib::Error>> + 'static,
        >,
    > {
        let parent = parent.map(ToOwned::to_owned);
        Box_::pin(gio::GioFuture::new(self, move |obj, cancellable, send| {
            obj.select_multiple_folders(
                parent.as_ref().map(::std::borrow::Borrow::borrow),
                Some(cancellable),
                move |res| {
                    send.resolve(res);
                },
            );
        }))
    }

    #[doc(alias = "gtk_file_dialog_set_accept_label")]
    pub fn set_accept_label(&self, accept_label: Option<&str>) {
        unsafe {
            ffi::gtk_file_dialog_set_accept_label(
                self.to_glib_none().0,
                accept_label.to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_dialog_set_default_filter")]
    pub fn set_default_filter(&self, filter: Option<&FileFilter>) {
        unsafe {
            ffi::gtk_file_dialog_set_default_filter(self.to_glib_none().0, filter.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_dialog_set_filters")]
    pub fn set_filters(&self, filters: &impl IsA<gio::ListModel>) {
        unsafe {
            ffi::gtk_file_dialog_set_filters(
                self.to_glib_none().0,
                filters.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_dialog_set_initial_file")]
    pub fn set_initial_file(&self, file: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_file_dialog_set_initial_file(
                self.to_glib_none().0,
                file.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_dialog_set_initial_folder")]
    pub fn set_initial_folder(&self, folder: Option<&impl IsA<gio::File>>) {
        unsafe {
            ffi::gtk_file_dialog_set_initial_folder(
                self.to_glib_none().0,
                folder.map(|p| p.as_ref()).to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_dialog_set_initial_name")]
    pub fn set_initial_name(&self, name: Option<&str>) {
        unsafe {
            ffi::gtk_file_dialog_set_initial_name(self.to_glib_none().0, name.to_glib_none().0);
        }
    }

    #[doc(alias = "gtk_file_dialog_set_modal")]
    pub fn set_modal(&self, modal: bool) {
        unsafe {
            ffi::gtk_file_dialog_set_modal(self.to_glib_none().0, modal.into_glib());
        }
    }

    #[doc(alias = "gtk_file_dialog_set_shortcut_folders")]
    pub fn set_shortcut_folders(&self, shortcut_folders: &impl IsA<gio::ListModel>) {
        unsafe {
            ffi::gtk_file_dialog_set_shortcut_folders(
                self.to_glib_none().0,
                shortcut_folders.as_ref().to_glib_none().0,
            );
        }
    }

    #[doc(alias = "gtk_file_dialog_set_title")]
    pub fn set_title(&self, title: &str) {
        unsafe {
            ffi::gtk_file_dialog_set_title(self.to_glib_none().0, title.to_glib_none().0);
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "accept-label")]
    pub fn connect_accept_label_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_accept_label_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::accept-label\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_accept_label_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "default-filter")]
    pub fn connect_default_filter_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_default_filter_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::default-filter\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_default_filter_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "filters")]
    pub fn connect_filters_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_filters_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::filters\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_filters_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "initial-file")]
    pub fn connect_initial_file_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_initial_file_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::initial-file\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_initial_file_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "initial-folder")]
    pub fn connect_initial_folder_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_initial_folder_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::initial-folder\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_initial_folder_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "initial-name")]
    pub fn connect_initial_name_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_initial_name_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::initial-name\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_initial_name_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "modal")]
    pub fn connect_modal_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_modal_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::modal\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_modal_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "shortcut-folders")]
    pub fn connect_shortcut_folders_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_shortcut_folders_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::shortcut-folders\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_shortcut_folders_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    #[doc(alias = "title")]
    pub fn connect_title_notify<F: Fn(&Self) + 'static>(&self, f: F) -> SignalHandlerId {
        unsafe extern "C" fn notify_title_trampoline<F: Fn(&FileDialog) + 'static>(
            this: *mut ffi::GtkFileDialog,
            _param_spec: glib::ffi::gpointer,
            f: glib::ffi::gpointer,
        ) {
            let f: &F = &*(f as *const F);
            f(&from_glib_borrow(this))
        }
        unsafe {
            let f: Box_<F> = Box_::new(f);
            connect_raw(
                self.as_ptr() as *mut _,
                b"notify::title\0".as_ptr() as *const _,
                Some(transmute::<_, unsafe extern "C" fn()>(
                    notify_title_trampoline::<F> as *const (),
                )),
                Box_::into_raw(f),
            )
        }
    }
}

#[cfg(any(feature = "v4_10", feature = "dox"))]
#[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
impl Default for FileDialog {
    fn default() -> Self {
        Self::new()
    }
}

#[derive(Clone, Default)]
// rustdoc-stripper-ignore-next
/// A [builder-pattern] type to construct [`FileDialog`] objects.
///
/// [builder-pattern]: https://doc.rust-lang.org/1.0.0/style/ownership/builders.html
#[must_use = "The builder must be built to be used"]
pub struct FileDialogBuilder {
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    accept_label: Option<String>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    default_filter: Option<FileFilter>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    filters: Option<gio::ListModel>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    initial_file: Option<gio::File>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    initial_folder: Option<gio::File>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    initial_name: Option<String>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    modal: Option<bool>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    shortcut_folders: Option<gio::ListModel>,
    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    title: Option<String>,
}

impl FileDialogBuilder {
    // rustdoc-stripper-ignore-next
    /// Create a new [`FileDialogBuilder`].
    pub fn new() -> Self {
        Self::default()
    }

    // rustdoc-stripper-ignore-next
    /// Build the [`FileDialog`].
    #[must_use = "Building the object from the builder is usually expensive and is not expected to have side effects"]
    pub fn build(self) -> FileDialog {
        let mut properties: Vec<(&str, &dyn ToValue)> = vec![];
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref accept_label) = self.accept_label {
            properties.push(("accept-label", accept_label));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref default_filter) = self.default_filter {
            properties.push(("default-filter", default_filter));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref filters) = self.filters {
            properties.push(("filters", filters));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref initial_file) = self.initial_file {
            properties.push(("initial-file", initial_file));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref initial_folder) = self.initial_folder {
            properties.push(("initial-folder", initial_folder));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref initial_name) = self.initial_name {
            properties.push(("initial-name", initial_name));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref modal) = self.modal {
            properties.push(("modal", modal));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref shortcut_folders) = self.shortcut_folders {
            properties.push(("shortcut-folders", shortcut_folders));
        }
        #[cfg(any(feature = "v4_10", feature = "dox"))]
        if let Some(ref title) = self.title {
            properties.push(("title", title));
        }
        glib::Object::new::<FileDialog>(&properties)
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn accept_label(mut self, accept_label: &str) -> Self {
        self.accept_label = Some(accept_label.to_string());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn default_filter(mut self, default_filter: &FileFilter) -> Self {
        self.default_filter = Some(default_filter.clone());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn filters(mut self, filters: &impl IsA<gio::ListModel>) -> Self {
        self.filters = Some(filters.clone().upcast());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_file(mut self, initial_file: &impl IsA<gio::File>) -> Self {
        self.initial_file = Some(initial_file.clone().upcast());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_folder(mut self, initial_folder: &impl IsA<gio::File>) -> Self {
        self.initial_folder = Some(initial_folder.clone().upcast());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn initial_name(mut self, initial_name: &str) -> Self {
        self.initial_name = Some(initial_name.to_string());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn modal(mut self, modal: bool) -> Self {
        self.modal = Some(modal);
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn shortcut_folders(mut self, shortcut_folders: &impl IsA<gio::ListModel>) -> Self {
        self.shortcut_folders = Some(shortcut_folders.clone().upcast());
        self
    }

    #[cfg(any(feature = "v4_10", feature = "dox"))]
    #[cfg_attr(feature = "dox", doc(cfg(feature = "v4_10")))]
    pub fn title(mut self, title: &str) -> Self {
        self.title = Some(title.to_string());
        self
    }
}

impl fmt::Display for FileDialog {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        f.write_str("FileDialog")
    }
}
