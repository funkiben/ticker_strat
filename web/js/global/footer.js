!function(n){var t={};function e(r){if(t[r])return t[r].exports;var o=t[r]={i:r,l:!1,exports:{}};return n[r].call(o.exports,o,o.exports,e),o.l=!0,o.exports}e.m=n,e.c=t,e.d=function(n,t,r){e.o(n,t)||Object.defineProperty(n,t,{enumerable:!0,get:r})},e.r=function(n){"undefined"!=typeof Symbol&&Symbol.toStringTag&&Object.defineProperty(n,Symbol.toStringTag,{value:"Module"}),Object.defineProperty(n,"__esModule",{value:!0})},e.t=function(n,t){if(1&t&&(n=e(n)),8&t)return n;if(4&t&&"object"==typeof n&&n&&n.__esModule)return n;var r=Object.create(null);if(e.r(r),Object.defineProperty(r,"default",{enumerable:!0,value:n}),2&t&&"string"!=typeof n)for(var o in n)e.d(r,o,function(t){return n[t]}.bind(null,o));return r},e.n=function(n){var t=n&&n.__esModule?function(){return n.default}:function(){return n};return e.d(t,"a",t),t},e.o=function(n,t){return Object.prototype.hasOwnProperty.call(n,t)},e.p="",e(e.s=9)}([function(n,t,e){"use strict";n.exports=function(n){var t=[];return t.toString=function(){return this.map((function(t){var e=function(n,t){var e=n[1]||"",r=n[3];if(!r)return e;if(t&&"function"==typeof btoa){var o=(c=r,a=btoa(unescape(encodeURIComponent(JSON.stringify(c)))),u="sourceMappingURL=data:application/json;charset=utf-8;base64,".concat(a),"/*# ".concat(u," */")),i=r.sources.map((function(n){return"/*# sourceURL=".concat(r.sourceRoot||"").concat(n," */")}));return[e].concat(i).concat([o]).join("\n")}var c,a,u;return[e].join("\n")}(t,n);return t[2]?"@media ".concat(t[2]," {").concat(e,"}"):e})).join("")},t.i=function(n,e,r){"string"==typeof n&&(n=[[null,n,""]]);var o={};if(r)for(var i=0;i<this.length;i++){var c=this[i][0];null!=c&&(o[c]=!0)}for(var a=0;a<n.length;a++){var u=[].concat(n[a]);r&&o[u[0]]||(e&&(u[2]?u[2]="".concat(e," and ").concat(u[2]):u[2]=e),t.push(u))}},t}},function(n,t,e){"use strict";const r=document.currentScript.src;t.a=function(n){let t=r.split("/"),e=n.src.split("/");return t.splice(0,t.length-1).join("/")+"/"+e[e.length-1]}},function(n,t,e){"use strict";n.exports=function(n,t){return t||(t={}),"string"!=typeof(n=n&&n.__esModule?n.default:n)?n:(t.hash&&(n+=t.hash),t.maybeNeedQuotes&&/[\t\n\f\r "'=<>`]/.test(n)?'"'.concat(n,'"'):n)}},function(n,t,e){"use strict";e.r(t),t.default=e.p+"9316b93a339996bca8fc97dff4ade263.png"},,,function(n,t,e){var r="<div id=footer> <img id=logo src="+e(2)(e(3))+" alt=Tickerstrat.io> </div>";n.exports=r},function(n,t,e){"use strict";var r=e(0),o=e.n(r)()(!1);o.push([n.i,":host {\r\n    --footer-height: 80px;\r\n    --navy: #182628;\r\n    --green: #3B945E;\r\n}\r\n\r\n#footer {\r\n    width: 100%;\r\n    height: var(--footer-height);\r\n    background-color: var(--navy);\r\n    color: var(--green);\r\n    display: flex;\r\n    align-items: center;\r\n    justify-content: space-between;\r\n}\r\n\r\n#logo {\r\n    height: 40px;\r\n}\r\n\r\n/*#links {*/\r\n/*    list-style-type: none;*/\r\n/*    margin-right: 10px;*/\r\n/*    padding: 0;*/\r\n/*}*/\r\n\r\n/*.m-link {*/\r\n/*    float: left;*/\r\n/*    margin: 10px;*/\r\n/*    text-decoration: underline;*/\r\n/*}*/\r\n\r\n/*.sub-links {*/\r\n/*    list-style-type: none;*/\r\n/*    padding: 0;*/\r\n/*    margin: 0;*/\r\n/*}*/\r\n\r\n/*.s-link {*/\r\n/*    text-decoration-line: none;*/\r\n/*}*/",""]),t.a=o},,function(n,t,e){"use strict";e.r(t);var r=e(6),o=e.n(r),i=e(7),c=e(1);class a extends HTMLElement{constructor(){super();const n=this.attachShadow({mode:"closed"});let t=document.createElement("style");t.innerHTML=i.a,n.appendChild(t);let e=document.createElement("div");e.innerHTML=o.a,n.appendChild(e);let r=n.getElementById("logo");r.src=Object(c.a)(r)}}window.addEventListener("load",(function(){customElements.define("global-footer",a);let n=document.createElement("global-footer");document.body.appendChild(n)}))}]);