import { invoke, Channel } from '@tauri-apps/api/core';
import '@tauri-apps/api/event';

// @ts-nocheck
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.
/** user-defined commands **/
const commands = {
    async getCurrentPosition(options) {
        try {
            return {
                status: "ok",
                data: await invoke("plugin:geolocation|get_current_position", {
                    options,
                }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async watchPosition(options, channel) {
        try {
            return {
                status: "ok",
                data: await invoke("plugin:geolocation|watch_position", {
                    options,
                    channel,
                }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async clearWatch(channelId) {
        try {
            return {
                status: "ok",
                data: await invoke("plugin:geolocation|clear_watch", {
                    channelId,
                }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async checkPermissions() {
        try {
            return {
                status: "ok",
                data: await invoke("plugin:geolocation|check_permissions"),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async requestPermissions(permissions) {
        try {
            return {
                status: "ok",
                data: await invoke("plugin:geolocation|request_permissions", {
                    permissions,
                }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
};

// Copyright 2019-2023 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT
/* eslint-disable @typescript-eslint/unbound-method */
async function watchPosition(options, 
// TODO: This can receive errors too
cb) {
    const channel = new Channel();
    channel.onmessage = cb;
    await commands.watchPosition(options, channel);
    return channel.id;
}
const { getCurrentPosition, clearWatch, checkPermissions, requestPermissions, } = commands;
// export { events };

export { checkPermissions, clearWatch, getCurrentPosition, requestPermissions, watchPosition };
