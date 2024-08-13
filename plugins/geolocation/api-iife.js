if("__TAURI__"in window){var __TAURI_PLUGIN_GEOLOCATION__=function(t){"use strict";function r(t,r,e,s){if("a"===e&&!s)throw new TypeError("Private accessor was defined without a getter");if("function"==typeof r?t!==r||!s:!r.has(t))throw new TypeError("Cannot read private member from an object whose class did not declare it");return"m"===e?s:"a"===e?s.call(t):s?s.value:r.get(t)}function e(t,r,e,s,o){if("function"==typeof r?t!==r||!o:!r.has(t))throw new TypeError("Cannot write private member to an object whose class did not declare it");return r.set(t,e),e}var s,o,i,a;"function"==typeof SuppressedError&&SuppressedError;class n{constructor(){this.__TAURI_CHANNEL_MARKER__=!0,s.set(this,(()=>{})),o.set(this,0),i.set(this,{}),this.id=function(t,r=!1){return window.__TAURI_INTERNALS__.transformCallback(t,r)}((({message:t,id:a})=>{if(a===r(this,o,"f")){e(this,o,a+1),r(this,s,"f").call(this,t);const n=Object.keys(r(this,i,"f"));if(n.length>0){let t=a+1;for(const e of n.sort()){if(parseInt(e)!==t)break;{const o=r(this,i,"f")[e];delete r(this,i,"f")[e],r(this,s,"f").call(this,o),t+=1}}e(this,o,t)}}else r(this,i,"f")[a.toString()]=t}))}set onmessage(t){e(this,s,t)}get onmessage(){return r(this,s,"f")}toJSON(){return`__CHANNEL__:${this.id}`}}async function c(t,r={},e){return window.__TAURI_INTERNALS__.invoke(t,r,e)}s=new WeakMap,o=new WeakMap,i=new WeakMap,function(t){t.WINDOW_RESIZED="tauri://resize",t.WINDOW_MOVED="tauri://move",t.WINDOW_CLOSE_REQUESTED="tauri://close-requested",t.WINDOW_DESTROYED="tauri://destroyed",t.WINDOW_FOCUS="tauri://focus",t.WINDOW_BLUR="tauri://blur",t.WINDOW_SCALE_FACTOR_CHANGED="tauri://scale-change",t.WINDOW_THEME_CHANGED="tauri://theme-changed",t.WINDOW_CREATED="tauri://window-created",t.WEBVIEW_CREATED="tauri://webview-created",t.DRAG_ENTER="tauri://drag-enter",t.DRAG_OVER="tauri://drag-over",t.DRAG_DROP="tauri://drag-drop",t.DRAG_LEAVE="tauri://drag-leave"}(a||(a={}));const u={async getCurrentPosition(t){try{return{status:"ok",data:await c("plugin:geolocation|get_current_position",{options:t})}}catch(t){if(t instanceof Error)throw t;return{status:"error",error:t}}},async watchPosition(t,r){try{return{status:"ok",data:await c("plugin:geolocation|watch_position",{options:t,channel:r})}}catch(t){if(t instanceof Error)throw t;return{status:"error",error:t}}},async clearWatch(t){try{return{status:"ok",data:await c("plugin:geolocation|clear_watch",{channelId:t})}}catch(t){if(t instanceof Error)throw t;return{status:"error",error:t}}},async checkPermissions(){try{return{status:"ok",data:await c("plugin:geolocation|check_permissions")}}catch(t){if(t instanceof Error)throw t;return{status:"error",error:t}}},async requestPermissions(t){try{return{status:"ok",data:await c("plugin:geolocation|request_permissions",{permissions:t})}}catch(t){if(t instanceof Error)throw t;return{status:"error",error:t}}}};const{getCurrentPosition:_,clearWatch:h,checkPermissions:f,requestPermissions:w}=u;return t.checkPermissions=f,t.clearWatch=h,t.getCurrentPosition=_,t.requestPermissions=w,t.watchPosition=async function(t,r){const e=new n;return e.onmessage=r,await u.watchPosition(t,e),e.id},t}({});Object.defineProperty(window.__TAURI__,"geolocation",{value:__TAURI_PLUGIN_GEOLOCATION__})}
