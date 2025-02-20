use {
    crate::{IntoSfResult, SfResult, cpp::FBox, ffi::window as ffi, window::ContextSettings},
    std::ffi::CStr,
};

decl_opaque! {
/// Type holding a valid drawing context.
///
/// If you need to make OpenGL calls without having an active window (like in a thread),
/// you can use an instance of this type to get a valid context.
///
/// Having a valid context is necessary for every OpenGL call.
///
/// Note that a context is only active in its current thread,
/// if you create a new thread it will have no valid context by default.
///
/// To use a `Context` instance, just construct it and let it live as long as you need
/// a valid context. No explicit activation is needed, all it has to do is to exist.
/// Its destructor will take care of deactivating and freeing all the attached resources.
pub Context;
}

impl Context {
    /// Creates and activates a new context.
    pub fn new() -> SfResult<FBox<Self>> {
        FBox::new(unsafe { ffi::sfContext_new() }).into_sf_result()
    }

    /// Explicitly activates or deactivates the context.
    ///
    /// # Arguments
    /// * active - `true` to activate, `false` to deactivate
    pub fn set_active(&mut self, active: bool) -> SfResult<()> {
        unsafe { ffi::sfContext_setActive(self, active) }.into_sf_result()
    }
    /// Get the settings of the context.
    ///
    /// Note that these settings may be different than the ones passed to the constructor;
    /// they are indeed adjusted if the original settings are not directly supported by the system.
    #[must_use]
    pub fn settings(&self) -> &ContextSettings {
        unsafe { &*ffi::sfContext_getSettings(self) }
    }

    /// Return a raw pointer to the currently active context
    #[must_use]
    pub fn active_context() -> *const Context {
        unsafe { ffi::sfContext_getActiveContext() }
    }

    /// Get the currently active context's ID.
    ///
    /// The context ID is used to identify contexts when managing unshareable OpenGL resources.
    #[must_use]
    pub fn active_context_id() -> u64 {
        unsafe { ffi::sfContext_getActiveContextId() }
    }

    /// Get the address of an OpenGL function.
    /// # Arguments
    /// * name - Name of the function to get the address of
    ///
    /// Returns the address of the OpenGL function, 0 on failure
    #[must_use]
    pub fn get_function(name: &CStr) -> *const std::ffi::c_void {
        unsafe { ffi::sfContext_getFunction(name.as_ptr()) }
    }
}

#[cfg_attr(not(feature = "ci-headless"), test)]
fn test_settings() {
    use {crate::window::Window, std::thread};

    let window =
        Window::new_open((32, 32), "test", Default::default(), &Default::default()).unwrap();
    let win_settings = *window.settings();
    thread::spawn(move || {
        let context = Context::new().unwrap();
        assert_eq!(context.settings(), &win_settings);
    })
    .join()
    .unwrap();
}

impl Drop for Context {
    /// Deactivates and destroys the context.
    fn drop(&mut self) {
        unsafe {
            ffi::sfContext_del(self);
        }
    }
}
