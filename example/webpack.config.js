const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require('path');

module.exports = {
  entry: [
    "./src/bootstrap.js",
  ],
  output: {
    path: path.resolve(__dirname, "dist"),
    filename: "bundle.js",
  },
  mode: "development",
  devServer: {
    static: {
      directory: path.resolve(__dirname, "dist"),
      watch: true,
    },
  },
  plugins: [
    new CopyWebpackPlugin({
      patterns: ['public/index.html'],
    })
  ],
  experiments: {
    syncWebAssembly: true,
  },
};
