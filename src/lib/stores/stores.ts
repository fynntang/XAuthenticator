import {writable} from "svelte/store";
import type {AppStateResponse} from "$lib/api/types";


export const appStore = writable<AppStateResponse | null>(null);