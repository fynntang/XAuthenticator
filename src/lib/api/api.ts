import {invoke} from "@tauri-apps/api/core";

export const initApp = async () => {
    await invoke("init_app")
}
// app_state

export const appState = async () => await invoke<{
    is_initialized: boolean,
    config: {
        path: string,
        builder: {
            settings: {
                theme: string,
                language: string,
                auto_lock: boolean,
                auto_lock_timeout: number
            }
        }
    }
    is_locked: boolean,
    master_key: string,
}>("app_state")