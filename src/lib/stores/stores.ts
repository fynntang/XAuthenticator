import {get, writable} from "svelte/store";
import type {AppStateResponse} from "$lib/api/types";
import {appState} from "$lib/api/api";


export const appStore = writable<AppStateResponse | null>(null);
export const appIsLocked = writable<boolean>(false);


export const appStateChange = (v: AppStateResponse) => {
    appStore.set(v);
    appIsLocked.update(v => get(appStore)?.isLocked ? true : v)
}
export const listenAppStateChange = () => {

    (async () => {
        appState().then(appStateChange)
        setInterval(() => appState().then(appStateChange), 3000);
    })()
}