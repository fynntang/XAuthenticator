import type {LayoutLoad} from "./$types";
import {createTray, getTray} from "$lib/tray";


export const load: LayoutLoad = async () => {
    if (!await getTray()) {
        await createTray();
    }
    return {}
}