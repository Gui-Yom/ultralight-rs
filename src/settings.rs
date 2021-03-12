use ultralight_sys::{
    ulCreateSettings, ulDestroySettings, ulSettingsSetAppName, ulSettingsSetDeveloperName,
    ulSettingsSetFileSystemPath, ulSettingsSetForceCPURenderer,
    ulSettingsSetLoadShadersFromFileSystem, ULSettings,
};

use crate::string::ULString;

pub struct Settings {
    pub raw: ULSettings,
    created: bool,
}

impl Settings {
    /// Create settings with default values
    pub fn new() -> Self {
        unsafe {
            Settings {
                raw: ulCreateSettings(),
                created: true,
            }
        }
    }

    /// Set the name of the developer of this app.
    /// This is used to generate a unique path to store local application data on the user's machine.
    /// Default is "MyCompany"
    pub fn developer_name(&self, developer_name: &str) {
        unsafe {
            let ulstr: ULString = developer_name.into();
            ulSettingsSetDeveloperName(self.raw, ulstr.into());
        }
    }

    /// Set the name of this app.
    /// This is used to generate a unique path to store local application data on the user's machine.
    /// Default is "MyApp"
    pub fn app_name(&self, app_name: &str) {
        unsafe {
            let ulstr: ULString = app_name.into();
            ulSettingsSetAppName(self.raw, ulstr.into());
        }
    }

    /// Set the root file path for our file system, you should set this to the relative path where all of your app data is.
    /// This will be used to resolve all file URLs, eg file:///page.html
    ///
    /// The default path is "./assets/". This relative path is resolved using the following logic:
    ///  - Windows: relative to the executable path
    ///  - Linux:   relative to the executable path
    ///  - macOS:   relative to YourApp.app/Contents/Resources/
    pub fn file_system_path(&self, file_system_path: &str) {
        unsafe {
            let ulstr: ULString = file_system_path.into();
            ulSettingsSetFileSystemPath(self.raw, ulstr.into());
        }
    }

    /// Set whether or not we should load and compile shaders from the file system
    /// (eg, from the /shaders/ path, relative to file_system_path).
    /// If this is false (the default), we will instead load pre-compiled shaders
    /// from memory which speeds up application startup time.
    pub fn load_shaders_from_filesystem(&self, load_shaders_from_filesystem: bool) {
        unsafe {
            ulSettingsSetLoadShadersFromFileSystem(self.raw, load_shaders_from_filesystem);
        }
    }

    /// We try to use the GPU renderer when a compatible GPU is detected.
    /// Set this to true to force the engine to always use the CPU renderer.
    pub fn force_cpu_renderer(&self, force_cpu_renderer: bool) {
        unsafe {
            ulSettingsSetForceCPURenderer(self.raw, force_cpu_renderer);
        }
    }
}

impl From<ULSettings> for Settings {
    fn from(raw: ULSettings) -> Self {
        Settings {
            raw,
            created: false,
        }
    }
}

impl Into<ULSettings> for &Settings {
    fn into(self) -> ULSettings {
        self.raw
    }
}

impl Drop for Settings {
    fn drop(&mut self) {
        unsafe {
            if self.created {
                ulDestroySettings(self.raw);
            }
        }
    }
}
