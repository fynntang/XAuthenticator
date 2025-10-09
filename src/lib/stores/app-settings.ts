import {writable} from "svelte/store";

function createAppSettingsStore() {
    const {subscribe, set, update} = writable<boolean>(false);

    return {
        subscribe,
        set,
        open: () => set(true),
        close: () => set(false),
        toggle: () => update(value => !value)
    }
}

export const appSettingsStore = createAppSettingsStore();