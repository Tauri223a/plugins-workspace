if("__TAURI__"in window){var __TAURI_PROCESS__=function(_){"use strict";return _.exit=async function(_=0){return window.__TAURI_INTERNALS__.invoke("plugin:process|exit",{code:_})},_.relaunch=async function(){return window.__TAURI_INTERNALS__.invoke("plugin:process|restart")},_}({});Object.defineProperty(window.__TAURI__,"process",{value:__TAURI_PROCESS__})}
