import type {LayoutLoad} from "./$types";
import {createTray} from "$lib/tray";
import {listenAppStateChange} from "$lib/stores/stores";


export const load: LayoutLoad = async () => {
    await createTray();

    listenAppStateChange();

    return {}
}