// Copyright 2019 The xi-editor Authors.
//
// Licensed under the Apache License, Version 2.0 (the "License");
// you may not use this file except in compliance with the License.
// You may obtain a copy of the License at
//
//     http://www.apache.org/licenses/LICENSE-2.0
//
// Unless required by applicable law or agreed to in writing, software
// distributed under the License is distributed on an "AS IS" BASIS,
// WITHOUT WARRANTIES OR CONDITIONS OF ANY KIND, either express or implied.
// See the License for the specific language governing permissions and
// limitations under the License.

//! Windows implementation of features at the application scope.

use winapi::shared::minwindef::HINSTANCE;
use winapi::um::shellscalingapi::PROCESS_SYSTEM_DPI_AWARE;
use winapi::um::wingdi::CreateSolidBrush;
use winapi::um::winuser::{LoadIconW, PostQuitMessage, IDI_APPLICATION};

use super::clipboard::Clipboard;
use super::util::{self, CLASS_NAME, OPTIONAL_FUNCTIONS};

pub struct Application;

impl Application {
    /// Initialize the app. At the moment, this is mostly needed for hi-dpi.
    pub fn init() {
        util::attach_console();
        if let Some(func) = OPTIONAL_FUNCTIONS.SetProcessDpiAwareness {
            // This function is only supported on windows 10
            unsafe {
                func(PROCESS_SYSTEM_DPI_AWARE); // TODO: per monitor (much harder)
            }
        }

        unsafe {
            let icon = LoadIconW(0 as HINSTANCE, IDI_APPLICATION);
            let brush = CreateSolidBrush(0xff_ff_ff);
            let _class_atom = win_win::WindowClass::builder(CLASS_NAME)
                .icon(icon)
                .background(brush)
                .build()
                .unwrap();
        }
    }

    pub fn quit() {
        unsafe {
            PostQuitMessage(0);
        }
    }

    pub fn clipboard() -> Clipboard {
        Clipboard
    }

    pub fn get_locale() -> String {
        //TODO ahem
        "en-US".into()
    }
}
