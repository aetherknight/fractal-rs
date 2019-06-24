const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

let config = (module.exports = (env, argv) => {
  return {
    entry: "./index.js",
    output: {
      path: path.resolve(__dirname, "dist", argv.mode),
      filename: "index.js"
    },
    // mode: "development",
    plugins: [
      new HtmlWebpackPlugin({ template: "index.html" }),
      new WasmPackPlugin({ crateDirectory: path.resolve(__dirname, "..") })
    ]
  };
});
