/* eslint-disable @typescript-eslint/no-var-requires */
require('dotenv').config()
const path = require('path');
const ForkTsCheckerWebpackPlugin = require('fork-ts-checker-webpack-plugin');
const ScriptExtHtmlWebpackPlugin = require('script-ext-html-webpack-plugin');
const HtmlWebpackPlugin = require('html-webpack-plugin');
const TerserPlugin = require('terser-webpack-plugin');
const webpack = require('webpack');

module.exports = env => ({
        mode: "production",
        optimization: {
            minimizer: [new TerserPlugin({
                parallel: true,
                //sourceMap: true
            })]
        },
        context: process.cwd(), // to automatically find tsconfig.json
        entry: "./typescript/index.ts",
        output: {
            path: path.join(process.cwd(), 'dist'),
            filename: '[name].js',
            publicPath: '/',
        },
        plugins: [
            new webpack.DefinePlugin({
                "process.env.release_target": JSON.stringify(env.release_target),
				"process.env.DEVELOPER": JSON.stringify(process.env.DEVELOPER)
            }),
			
            new ForkTsCheckerWebpackPlugin({
                async: false,
                useTypescriptIncrementalApi: true,
                memoryLimit: 4096
            }),
            new HtmlWebpackPlugin({
                hash: true,
                inject: true,
                template: 'typescript/index.html',
                minify: {
                    removeComments: true,
                    collapseWhitespace: true,
                    removeRedundantAttributes: true,
                    useShortDoctype: true,
                    removeEmptyAttributes: true,
                    removeStyleLinkTypeAttributes: true,
                    keepClosingSlash: true,
                    minifyJS: true,
                    minifyCSS: true,
                    minifyURLs: true,
                },
            }),
            new ScriptExtHtmlWebpackPlugin({
                defaultAttribute: 'defer'
            }),
        ],
        module: {
            rules: [
                {
                    test: /.ts$/,
                    use: [
                        { loader: 'ts-loader', options: { transpileOnly: true } }
                    ],
                    exclude: [
                        path.resolve(__dirname, "typescript/tests/**/*"),
                    ]
                },
                {

                    test: /\.css$/i,
                    use: [
                        'lit-css-loader',
                        path.resolve(__dirname, "build-utils/transform-css.js"),
                    ]
                },
            ]
        },
        resolve: {
            extensions: [".ts", ".js", ".css"],
            alias: {
                "@components": path.resolve(__dirname, "typescript/components"),
                "@styles": path.resolve(__dirname, "typescript/styles"),
                "@utils": path.resolve(__dirname, "typescript/utils"),
                "@settings": path.resolve(__dirname, "typescript/settings"),
                "@events": path.resolve(__dirname, "typescript/events"),
            }
        }
    })