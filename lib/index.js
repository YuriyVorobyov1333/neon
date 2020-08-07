// var addon = require('../native');

// console.log(addon.threadCount());

// module.exports = addon;


const {
  convertVecToArray,
  returnJsArray,
  returnJsArrayWithNumber,
  returnJsArrayWithString,
  makeAnArray,
} = require('../native/index.node');

console.log(
  convertVecToArray(),
  returnJsArray(),
  returnJsArrayWithNumber(),
  returnJsArrayWithString(),
  makeAnArray(),
);

module.exports = require('../native/index.node');