var __webpack_modules__ = ({
"./fixtures/answer.js": (function (__unused_webpack_module, __webpack_exports__) {
"use strict";
__webpack_require__.r(__webpack_exports__);
__webpack_require__.d(__webpack_exports__, {
  answer: function() { return answer; }
});
const answer = 42;

}),
"data:text/javascript,module.exports='Hello, World!'": (function (module) {
module.exports='Hello, World!'

}),

});
/************************************************************************/
// The module cache
var __webpack_module_cache__ = {};

// The require function
function __webpack_require__(moduleId) {

// Check if module is in cache
var cachedModule = __webpack_module_cache__[moduleId];
if (cachedModule !== undefined) {
return cachedModule.exports;
}
// Create a new module (and put it into the cache)
var module = (__webpack_module_cache__[moduleId] = {
exports: {}
});
// Execute the module function
__webpack_modules__[moduleId](module, module.exports, __webpack_require__);

// Return the exports of the module
return module.exports;

}

/************************************************************************/
var __webpack_exports__ = {};
// This entry need to be wrapped in an IIFE because it need to be in strict mode.
(() => {
"use strict";
__webpack_require__.r(__webpack_exports__);
/* harmony import */var _answer_js__WEBPACK_IMPORTED_MODULE_0__ = __webpack_require__("./fixtures/answer.js");
/* harmony import */var data_text_javascript_module_exports_Hello_World___WEBPACK_IMPORTED_MODULE_1__ = __webpack_require__("data:text/javascript,module.exports='Hello, World!'");
/* harmony import */var data_text_javascript_module_exports_Hello_World___WEBPACK_IMPORTED_MODULE_1___default = /*#__PURE__*/__webpack_require__.n(data_text_javascript_module_exports_Hello_World___WEBPACK_IMPORTED_MODULE_1__);


console.log('answer:',_answer_js__WEBPACK_IMPORTED_MODULE_0__.answer,(data_text_javascript_module_exports_Hello_World___WEBPACK_IMPORTED_MODULE_1___default()));
})();

