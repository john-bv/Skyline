// This script will generate the packets documentation from the /protocol/src/net directory
// which contains rust files with the packet definitions.
import fs from 'fs';
import path, { resolve as presolve, join } from 'path';
import { fileURLToPath } from 'url';
import Util from 'util';
import { spawn } from 'child_process';

const __dirname = path.dirname(fileURLToPath(import.meta.url));


function info(...args) {
    // white with [INFO]
    console.log('\x1b[36m%s\x1b[0m', '[INFO] ' + args.join(' '));
}

function error(...args) {
    // red with [ERROR]
    console.log('\x1b[31m%s\x1b[0m', '[ERROR] ' + args.join(' '));
}

function warn(...args) {
    // yellow with [WARN]
    console.log('\x1b[33m%s\x1b[0m', '[WARN] ' + args.join(' '));
}

function resolve(...args) {
    return presolve(__dirname, ...args);
}


/**
 * @returns {Promise<{"name": string, "id": number, "fields": {"name": string, "type": string, "comment": string}[]}[]>}
 */
async function getPackets(file) {}

error('Not implemented yet');