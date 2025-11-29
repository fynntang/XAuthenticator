import { get, writable } from "svelte/store";
import type { AppStateResponse } from "$lib/api/types";
import { appState } from "$lib/api/api";
import type { Account } from "$lib/api/types";


export const appStore = writable<AppStateResponse | null>(null);
export const appIsLocked = writable<boolean>(false);
export const accountDialogState = writable<{ open: boolean, mode: 'create' | 'edit', account?: Account }>({ open: false, mode: 'create' });
export const refreshAccountsTrigger = writable<number>(0);


export const appStateChange = (v: AppStateResponse) => {
    appStore.set(v);
    appIsLocked.update(v => get(appStore)?.isLocked ? true : v)
}
let intervalId: number | undefined;

export const listenAppStateChange = () => {
    if (intervalId) {
        clearInterval(intervalId);
    }

    (async () => {
        appState().then(appStateChange)
        intervalId = setInterval(() => appState().then(appStateChange), 3000);
    })()
}

export const stopListeningAppStateChange = () => {
    if (intervalId) {
        clearInterval(intervalId);
        intervalId = undefined;
    }
}