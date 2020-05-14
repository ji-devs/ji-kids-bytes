require('dotenv').config();

const spawn = require('cross-spawn');
const fs = require('fs');
const os = require('os');
const path = require('path');

const cmd = process.argv[2];

const CloudStorageMedia = `gs://bytes-ji-kids-eu/`;


if(!process.env.LOCAL_CDN_DIR || process.env.LOCAL_CDN_DIR === "") {
    console.log("Local CDN: set [LOCAL_CDN_DIR] in .env");
    process.exit(0);
}

const localPath = path.resolve(process.env.LOCAL_CDN_DIR);


if(cmd === "--hard") {
    console.log(`Syncing ${cmd}`);
    console.log(`From ${localPath}`);
    console.log(`To ${CloudStorageMedia}`);
    spawn.sync("gsutil", ["-m", "rsync", "-d", "-r", localPath, CloudStorageMedia]);
} else {
    console.log("invalid option!");
}
