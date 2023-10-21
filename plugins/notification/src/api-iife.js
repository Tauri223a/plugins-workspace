if("__TAURI__"in window){var __TAURI_NOTIFICATION__=function(n){"use strict";var i=Object.defineProperty,e=(n,i,e)=>{if(!i.has(n))throw TypeError("Cannot "+e)},t=(n,i,t)=>(e(n,i,"read from private field"),t?t.call(n):i.get(n));function o(n,i=!1){return window.__TAURI_INTERNALS__.transformCallback(n,i)}((n,e)=>{for(var t in e)i(n,t,{get:e[t],enumerable:!0})})({},{Channel:()=>a,PluginListener:()=>l,addPluginListener:()=>f,convertFileSrc:()=>d,invoke:()=>_,transformCallback:()=>o});var r,a=class{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,((n,i,e)=>{if(i.has(n))throw TypeError("Cannot add the same private member more than once");i instanceof WeakSet?i.add(n):i.set(n,e)})(this,r,(()=>{})),this.id=o((n=>{t(this,r).call(this,n)}))}set onmessage(n){var i,t,o,a;o=n,e(i=this,t=r,"write to private field"),a?a.call(i,o):t.set(i,o)}get onmessage(){return t(this,r)}toJSON(){return`__CHANNEL__:${this.id}`}};r=new WeakMap;var c,s,u,l=class{constructor(n,i,e){this.plugin=n,this.event=i,this.channelId=e}async unregister(){return _(`plugin:${this.plugin}|remove_listener`,{event:this.event,channelId:this.channelId})}};async function f(n,i,e){let t=new a;return t.onmessage=e,_(`plugin:${n}|register_listener`,{event:i,handler:t}).then((()=>new l(n,i,t.id)))}async function _(n,i={},e){return window.__TAURI_INTERNALS__.invoke(n,i,e)}function d(n,i="asset"){return window.__TAURI_INTERNALS__.convertFileSrc(n,i)}return function(n){n.Year="year",n.Month="month",n.TwoWeeks="twoWeeks",n.Week="week",n.Day="day",n.Hour="hour",n.Minute="minute",n.Second="second"}(c||(c={})),n.Importance=void 0,(s=n.Importance||(n.Importance={}))[s.None=0]="None",s[s.Min=1]="Min",s[s.Low=2]="Low",s[s.Default=3]="Default",s[s.High=4]="High",n.Visibility=void 0,(u=n.Visibility||(n.Visibility={}))[u.Secret=-1]="Secret",u[u.Private=0]="Private",u[u.Public=1]="Public",n.active=async function(){return _("plugin:notification|get_active")},n.cancel=async function(n){return _("plugin:notification|cancel",{notifications:n})},n.cancelAll=async function(){return _("plugin:notification|cancel")},n.channels=async function(){return _("plugin:notification|listChannels")},n.createChannel=async function(n){return _("plugin:notification|create_channel",{...n})},n.isPermissionGranted=async function(){return"default"!==window.Notification.permission?Promise.resolve("granted"===window.Notification.permission):_("plugin:notification|is_permission_granted")},n.onAction=async function(n){return f("notification","actionPerformed",n)},n.onNotificationReceived=async function(n){return f("notification","notification",n)},n.pending=async function(){return _("plugin:notification|get_pending")},n.registerActionTypes=async function(n){return _("plugin:notification|register_action_types",{types:n})},n.removeActive=async function(n){return _("plugin:notification|remove_active",{notifications:n})},n.removeAllActive=async function(){return _("plugin:notification|remove_active")},n.removeChannel=async function(n){return _("plugin:notification|delete_channel",{id:n})},n.requestPermission=async function(){return window.Notification.requestPermission()},n.sendNotification=function(n){"string"==typeof n?new window.Notification(n):new window.Notification(n.title,n)},n}({});Object.defineProperty(window.__TAURI__,"notification",{value:__TAURI_NOTIFICATION__})}
