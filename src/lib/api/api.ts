import {invoke} from "@tauri-apps/api/core";

export const initApp = async () => {
    await invoke("init_app")
}


export type AppStateResponse = {
    isInitialized: boolean,
    config: {
        path: string,
        builder: {
            settings: {
                theme: string,
                language: string,
                autoLock: boolean,
                autoLockTimeout: number
            }
        }
    }
    isLocked: boolean
}
export const appState = async () => await invoke<AppStateResponse>("app_state")