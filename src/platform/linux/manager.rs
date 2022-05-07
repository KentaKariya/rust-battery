use std::path::{Path, PathBuf};

use super::device::SysFsDevice;
use super::iterator::SysFsIterator;
use crate::platform::traits::*;
use crate::Result;

static SYSFS_ROOT: &str = "/sys/class/power_supply";

#[derive(Debug)]
pub struct SysFsManager {
    root: PathBuf,
}

impl SysFsManager {
    pub fn path(&self) -> &Path {
        self.root.as_path()
    }
}

impl BatteryManager for SysFsManager {
    type Iterator = SysFsIterator;

    fn new() -> Result<Self> {
        Ok(Self {
            root: PathBuf::from(SYSFS_ROOT),
        })
    }

    fn refresh(&self, device: &mut SysFsDevice) -> Result<()> {
        device.refresh()
    }
}
