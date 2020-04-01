/* eslint-disable @typescript-eslint/no-var-requires */
require('dotenv').config()
const path = require('path');
const webpack = require('webpack');
const ForkTsCheckerNotifierWebpackPlugin = require('fork-ts-checker-notifier-webpack-plugin');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');

module.exports = {
    mode: "none", //maybe a tiny speedup - but use DefinePlugin manually
    context: process.cwd(), // to automatically find tsconfig.json
    entry: "./typescript/index.ts",
    output: {
        path: path.resolve(__dirname, 'dist'),
        filename: '[name].js',
        publicPath: "/"
    },
    plugins: [
        new webpack.DefinePlugin({
            "process.env.release_target": JSON.stringify(process.env.release_target),
			"process.env.DEVELOPER": JSON.stringify(process.env.DEVELOPER)
        }),
        new ForkTsCheckerWebpackPlugin({
            eslint: false
        }),
        new ForkTsCheckerNotifierWebpackPlugin({ title: 'TypeScript', excludeWarnings: false }),
        new HtmlWebpackPlugin({
            inject: true,
            template: 'typescript/index.html'
        }),
        new ScriptExtHtmlWebpackPlugin({
            defaultAttribute: 'defer'
        }),

        new webpack.ProvidePlugin({
          TextDecoder: ['text-encoding', 'TextDecoder'],
          TextEncoder: ['text-encoding', 'TextEncoder'],
        })
        //new webpack.HotModuleReplacementPlugin()
    ],
    module: {
        rules: [
            {
                test: /\.css$/i,
                use: [ 
                    'lit-css-loader',
                    path.resolve(__dirname, "build-utils/transform-css.js"),
                ],
            },
            {
                test: /.ts$/,
                use: [
                    { loader: 'ts-loader', options: { transpileOnly: true } }
                ],
                exclude: [
                    path.resolve(__dirname, "typescript/tests/**/*"),
                ]

            },
        ]
    },
    resolve: {
        extensions: [".ts", ".js", ".css"],
        alias: {
            "@components": path.resolve(__dirname, "typescript/components"),
            "@utils": path.resolve(__dirname, "typescript/utils"),
            "@settings": path.resolve(__dirname, "typescript/settings"),
            "@events": path.resolve(__dirname, "typescript/events"),
        }
    },
    devtool: 'inline-source-map',
    devServer: {
        contentBase: path.resolve(__dirname, './_static'),
        compress: true,
        clientLogLevel: 'warning',
        open: true,
        historyApiFallback: true,
        stats: 'errors-only',
        watchOptions: {
            ignored: ['node_modules', 'target', 'pkg', '**/*.rs']
        },
        watchContentBase: true,
    }
}