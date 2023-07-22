import { writable } from "svelte/store";
import type { FileData } from "./types/files";

export const query = writable<string>();
export const files = writable<FileData[]>([]);