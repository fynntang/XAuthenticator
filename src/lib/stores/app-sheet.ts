import {writable} from "svelte/store";

function createAppSheetStore() {
    const {subscribe, set, update} = writable<boolean>(false);

    return {
        subscribe,
        set,
        open: () => set(true),
        close: () => set(false),
        toggle: () => update(value => !value)
    }
}

export const appSheetStore = createAppSheetStore();