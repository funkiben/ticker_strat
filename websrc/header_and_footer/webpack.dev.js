const { merge } = require('webpack-merge');
const common = require('./webpack.common.js');
const copyPlugin = require('copy-webpack-plugin');
const path = require('path');
const { CleanWebpackPlugin } = require('clean-webpack-plugin');

module.exports = merge(common, {
    mode: 'development',
    devtool: 'inline-source-map',
    output: {
        path: path.resolve(__dirname, 'build'),
    },
    optimization: {
        minimize: false,
    },
    plugins: [
        new copyPlugin({
            patterns: [
                { from: './tests/test.html' }
            ],
        }),
        new CleanWebpackPlugin({ cleanStaleWebpackAssets: false }),
    ],
});