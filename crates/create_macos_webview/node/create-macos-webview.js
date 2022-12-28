#!/usr/bin/env node

// Copyright 2019-2022 Tauri Programme within The Commons Conservancy
// SPDX-License-Identifier: Apache-2.0
// SPDX-License-Identifier: MIT

const cli = require("./index");
const path = require("path");

const [bin, script, ...arguments] = process.argv;
const binStem = path.parse(bin).name.toLowerCase();

let binName;

if (binStem.match(/(nodejs|node)([1-9]*)*$/g)) {
    const managerStem = process.env.npm_execpath
        ? path.parse(process.env.npm_execpath).name.toLowerCase()
        : null;
    if (managerStem) {
        let manager;
        switch (managerStem) {
            case "npm-cli":
                manager = "npm";
                break;

            default:
                manager = managerStem;
                break;
        }

        binName = `${manager} run ${process.env.npm_lifecycle_event}`;
    } else {

        const scriptNormal = path.normalize(path.relative(process.cwd(), script));
        binName = `${binStem} ${scriptNormal}`;
    }
} else {
    arguments.unshift(bin);
}

cli.run(arguments, binName);
