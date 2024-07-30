'use strict';

var core = require('@tauri-apps/api/core');
require('@tauri-apps/api/event');

// @ts-nocheck
// This file was generated by [tauri-specta](https://github.com/oscartbeaumont/tauri-specta). Do not edit this file manually.
/** user-defined commands **/
const commands = {
    async vibrate(duration) {
        try {
            return {
                status: "ok",
                data: await core.invoke("plugin:haptics|vibrate", { duration }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async impactFeedback(style) {
        try {
            return {
                status: "ok",
                data: await core.invoke("plugin:haptics|impact_feedback", { style }),
            };
        }
        catch (e) {
            if (e instanceof Error)
                throw e;
            else
                return { status: "error", error: e };
        }
    },
    async notificationFeedback(type) {
        try {
            return {
                status: "ok",
                data: await core.invoke("plugin:haptics|notification_feedback", {
                    type,
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
    async selectionFeedback() {
        try {
            return {
                status: "ok",
                data: await core.invoke("plugin:haptics|selection_feedback"),
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
const { vibrate, impactFeedback, notificationFeedback, selectionFeedback, } = commands;
// export { events };

exports.impactFeedback = impactFeedback;
exports.notificationFeedback = notificationFeedback;
exports.selectionFeedback = selectionFeedback;
exports.vibrate = vibrate;
