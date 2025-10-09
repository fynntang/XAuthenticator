import {writable} from "svelte/store";

// windows

function createAppWindowStore() {
    const {subscribe, set, update} = writable<boolean>(false);

    return {
        subscribe,
        set,
        open: () => set(true),
        close: () => set(false),
        toggle: () => update(value => !value)
    }
}

export const appWindowStore = createAppWindowStore();