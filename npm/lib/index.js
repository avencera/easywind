"use strict";

const path = require("path");

module.exports.easyWindPath = path.join(
  __dirname,
  `../bin/easywind${process.platform === "win32" ? ".exe" : ""}`
);
