if("__TAURI__"in window){var __TAURI_STORE__=function(e){"use strict";var t=Object.defineProperty,n=(e,n)=>{for(var a in n)t(e,a,{get:n[a],enumerable:!0})},a=(e,t,n)=>{if(!t.has(e))throw TypeError("Cannot "+n)},i=(e,t,n)=>(a(e,t,"read from private field"),n?n.call(e):t.get(e));function r(e,t=!1){return window.__TAURI_INTERNALS__.transformCallback(e,t)}n({},{Channel:()=>s,PluginListener:()=>o,addPluginListener:()=>h,convertFileSrc:()=>l,invoke:()=>u,transformCallback:()=>r});var _,s=class{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,((e,t,n)=>{if(t.has(e))throw TypeError("Cannot add the same private member more than once");t instanceof WeakSet?t.add(e):t.set(e,n)})(this,_,(()=>{})),this.id=r((e=>{i(this,_).call(this,e)}))}set onmessage(e){((e,t,n,i)=>{a(e,t,"write to private field"),i?i.call(e,n):t.set(e,n)})(this,_,e)}get onmessage(){return i(this,_)}toJSON(){return`__CHANNEL__:${this.id}`}};_=new WeakMap;var o=class{constructor(e,t,n){this.plugin=e,this.event=t,this.channelId=n}async unregister(){return u(`plugin:${this.plugin}|remove_listener`,{event:this.event,channelId:this.channelId})}};async function h(e,t,n){let a=new s;return a.onmessage=n,u(`plugin:${e}|register_listener`,{event:t,handler:a}).then((()=>new o(e,t,a.id)))}async function u(e,t={},n){return window.__TAURI_INTERNALS__.invoke(e,t,n)}function l(e,t="asset"){return window.__TAURI_INTERNALS__.convertFileSrc(e,t)}n({},{TauriEvent:()=>c,emit:()=>I,listen:()=>w,once:()=>d});var c=(e=>(e.WINDOW_RESIZED="tauri://resize",e.WINDOW_MOVED="tauri://move",e.WINDOW_CLOSE_REQUESTED="tauri://close-requested",e.WINDOW_CREATED="tauri://window-created",e.WINDOW_DESTROYED="tauri://destroyed",e.WINDOW_FOCUS="tauri://focus",e.WINDOW_BLUR="tauri://blur",e.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",e.WINDOW_THEME_CHANGED="tauri://theme-changed",e.WINDOW_FILE_DROP="tauri://file-drop",e.WINDOW_FILE_DROP_HOVER="tauri://file-drop-hover",e.WINDOW_FILE_DROP_CANCELLED="tauri://file-drop-cancelled",e.MENU="tauri://menu",e))(c||{});async function p(e,t){await u("plugin:event|unlisten",{event:e,eventId:t})}async function w(e,t,n){return u("plugin:event|listen",{event:e,windowLabel:n?.target,handler:r(t)}).then((t=>async()=>p(e,t)))}async function d(e,t,n){return w(e,(n=>{t(n),p(e,n.id).catch((()=>{}))}),n)}async function I(e,t,n){await u("plugin:event|emit",{event:e,windowLabel:n?.target,payload:t})}return e.Store=class{constructor(e){this.path=e}async set(e,t){return await window.__TAURI_INTERNALS__.invoke("plugin:store|set",{path:this.path,key:e,value:t})}async get(e){return await window.__TAURI_INTERNALS__.invoke("plugin:store|get",{path:this.path,key:e})}async has(e){return await window.__TAURI_INTERNALS__.invoke("plugin:store|has",{path:this.path,key:e})}async delete(e){return await window.__TAURI_INTERNALS__.invoke("plugin:store|delete",{path:this.path,key:e})}async clear(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|clear",{path:this.path})}async reset(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|reset",{path:this.path})}async keys(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|keys",{path:this.path})}async values(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|values",{path:this.path})}async entries(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|entries",{path:this.path})}async length(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|length",{path:this.path})}async load(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|load",{path:this.path})}async save(){return await window.__TAURI_INTERNALS__.invoke("plugin:store|save",{path:this.path})}async onKeyChange(e,t){return await w("store://change",(n=>{n.payload.path===this.path&&n.payload.key===e&&t(n.payload.value)}))}async onChange(e){return await w("store://change",(t=>{t.payload.path===this.path&&e(t.payload.key,t.payload.value)}))}},e}({});Object.defineProperty(window.__TAURI__,"store",{value:__TAURI_STORE__})}
