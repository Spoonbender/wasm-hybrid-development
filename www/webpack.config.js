const CopyWebpackPlugin = require("copy-webpack-plugin");
const path = require("path");
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = {
    entry: "./bootstrap.js",
    output: {
        path: path.resolve(__dirname, "dist"),
        filename: "bootstrap.js",
    },
    mode: "development",
    plugins: [
        new CleanWebpackPlugin({ cleanOnceBeforeBuildPatterns: ["dist"] }),
        new CopyWebpackPlugin([
            'index.html',
            { from: 'assets', to: 'assets' },
            { from: 'static', to: 'static' },
            { from: 'themes', to: 'themes' }
        ]),
    ],
};