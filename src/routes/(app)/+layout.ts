import type {LayoutLoad} from "./$types";
import {createTray} from "$lib/tray";


export const load: LayoutLoad = async () => {
    await createTray();

    return {}
}