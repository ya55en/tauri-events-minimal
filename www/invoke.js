/**
 * Setup the bridge for invoking rust functions.
 * (See https://tauri.studio/en/docs/usage/guides/command)
 */

// With the Tauri API npm package:
import { invoke } from '/modules/@tauri-apps/api/tauri.js';

// With the Tauri global script, enabled when
// `tauri.conf.json > build > withGlobalTauri` is set to true:
export const invoke_cmd = window.__TAURI__.invoke;
