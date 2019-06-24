const path = require("path");
const HtmlWebpackPlugin = require("html-webpack-plugin");
const webpack = require("webpack");
const WasmPackPlugin = require("@wasm-tool/wasm-pack-plugin");

let config = (module.exports = (env, argv) => {
  return {
    entry: path.resolve(__dirname, "js", "index.js"),
    output: {
      path: path.resolve(__dirname, "dist", argv.mode),
      filename: "index.js"
    },
    // mode: "development",
    plugins: [
      new HtmlWebpackPlugin({
        template: path.resolve(__dirname, "static", "index.html")
      }),
      new WasmPackPlugin({ crateDirectory: __dirname })
    ]
  };
});
