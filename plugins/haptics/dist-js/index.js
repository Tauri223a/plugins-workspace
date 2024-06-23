import { invoke } from '@tauri-apps/api/core';
import * as TAURI_API_EVENT from '@tauri-apps/api/event';

// @ts-nocheck
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.
/** user-defined commands **/
const commands = {
    async vibrate(duration) {
        try {
            return { status: "ok", data: await invoke("plugin:haptics|vibrate", { duration }) };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    }
};
/** user-defined events **/
__makeEvents__({
    randomNumber: "plugin:haptics:random-number"
});
function __makeEvents__(mappings) {
    return new Proxy({}, {
        get: (_, event) => {
            const name = mappings[event];
            return new Proxy((() => { }), {
                apply: (_, __, [window]) => ({
                    listen: (arg) => window.listen(name, arg),
                    once: (arg) => window.once(name, arg),
                    emit: (arg) => window.emit(name, arg),
                }),
                get: (_, command) => {
                    switch (command) {
                        case "listen":
                            return (arg) => TAURI_API_EVENT.listen(name, arg);
                        case "once":
                            return (arg) => TAURI_API_EVENT.once(name, arg);
                        case "emit":
                            return (arg) => TAURI_API_EVENT.emit(name, arg);
                    }
                },
            });
        },
    });
}

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/* eslint-disable @typescript-eslint/unbound-method */
const { vibrate } = commands;
// export { events };

export { vibrate };
