#!/usr/bin/env node
'use strict';

const { spawnSync } = require('child_process');
const path = require('path');

const bin = path.join(
  __dirname,
  'bin',
  process.platform === 'win32' ? 'oxide.exe' : 'oxide'
);

const result = spawnSync(bin, process.argv.slice(2), { stdio: 'inherit' });

if (result.error) {
  if (result.error.code === 'ENOENT') {
    console.error('oxide-cli: binary not found. Try reinstalling: npm install -g @anesis-cli/anesis');
  } else {
    console.error(`oxide-cli: ${result.error.message}`);
  }
  process.exit(1);
}

process.exit(result.status ?? 1);
